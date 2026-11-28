#!/usr/bin/env python3
"""Generate narration takes with a hosted TTS instead of local Qwen3-TTS.

Reads the settings from the environment, which `env.sh` fills from
`${MOTION_CONFIG:-~/.config/motion-explainer/config.env}` (the file the install
writes). Anything already set in the shell wins.

  TTS_PROVIDER   openai | elevenlabs | custom   (default `custom` when TTS_URL is set, else local Qwen3-TTS)
  TTS_URL        the endpoint to POST to. Also sets the provider when TTS_PROVIDER is empty.
                   openai      replaces https://api.openai.com/v1/audio/speech (gateways, Azure, Ollama, ...)
                   elevenlabs  replaces https://api.elevenlabs.io/v1/text-to-speech/<voice>
                   custom      required; `{voice}` and `{model}` in the URL are substituted
                 TTS_CUSTOM_URL is accepted as an alias.
  TTS_MODEL      model id; defaults: gpt-4o-mini-tts, eleven_multilingual_v2
  TTS_VOICE      voice id; defaults: alloy, 21m00Tcm4TlvDq8ikWAM
  TTS_SPEED      0.25-4.0, provider default 1.0 (OpenAI, ElevenLabs `speed`)
  TTS_LANG       language tag sent where the API takes one, default `language` in voice.json
  TTS_AUTH       custom only: `none` (default), `bearer` or a header NAME sent as-is
  TTS_PAYLOAD    custom only: JSON body template; {text} {voice} {model} {lang} {speed} are substituted
  TTS_FORMAT     response encoding, default mp3 (custom: whatever the endpoint returns)
  TTS_API_KEY    key for openai / elevenlabs / a bearer custom endpoint
  TTS_TIMEOUT    request timeout in seconds, default 90
  TTS_<PROVIDER>_KEY, TTS_ELEVENLABS_URL   per-provider overrides

  usage (from an episode's narration/ folder, like narrate.sh):
    python3 tts_provider.py VOICE.json SCRIPT.json OUT_DIR
    ONLY=l1_hook,l2_x python3 tts_provider.py voice/voice.json script.json voice/takes/narrator

  Writes OUT_DIR/<line>_<k>.wav, 16-bit PCM at 48 kHz mono (the shape finalize.sh
  and pace.py read), and prints one line per take.
"""
import json, os, shutil, subprocess, sys, tempfile, urllib.error, urllib.request

PROVIDERS = {"openai", "elevenlabs", "custom"}
DEFAULT_MODEL = {"openai": "gpt-4o-mini-tts", "elevenlabs": "eleven_multilingual_v2", "custom": ""}
DEFAULT_VOICE = {"openai": "alloy", "elevenlabs": "21m00Tcm4TlvDq8ikWAM", "custom": ""}
ERRS = {401: "key rejected", 403: "key lacks this model or voice", 404: "wrong URL, model or voice id",
        413: "line too long for this endpoint", 429: "rate limit or quota; wait or raise the plan"}


def env(name, default=None):
    v = os.environ.get(name, "").strip()
    return v or default


def die(msg):
    print("tts_provider:", msg, file=sys.stderr)
    sys.exit(2)


def fill(tpl, text, voice, model, lang, speed):
    for k, v in {"text": text, "voice": voice, "model": model, "lang": lang, "speed": speed}.items():
        tpl = tpl.replace("{" + k + "}", json.dumps(v)[1:-1] if k == "text" else str(v))
    return tpl


def request(url, body, headers, timeout):
    req = urllib.request.Request(url, data=body, headers={"Content-Type": "application/json", **headers})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.read()
    except urllib.error.HTTPError as e:
        detail = e.read().decode("utf-8", "replace")[:300]
        hint = ERRS.get(e.code, "")
        die(f"HTTP {e.code} {hint}{': ' + detail if detail else ''}")
    except urllib.error.URLError as e:
        die(f"cannot reach {url}: {e.reason}")


def transcode(raw, fmt, dst):
    """The API answers with bytes; ffmpeg turns them into the WAV the pipeline wants."""
    if not shutil.which("ffmpeg"):
        die("ffmpeg is needed to convert the API audio to WAV")
    tmp = tempfile.mkdtemp()
    src = os.path.join(tmp, "raw." + fmt)
    with open(src, "wb") as f:
        f.write(raw)
    try:
        subprocess.run(["ffmpeg", "-v", "error", "-y", "-i", src, "-ac", "1", "-ar", "48000",
                        "-c:a", "pcm_s16le", dst], check=True)
    except subprocess.CalledProcessError:
        die(f"could not decode the {fmt} response; check TTS_FORMAT")
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


def resolve():
    """(provider, url) from the env. TTS_URL makes an OpenAI-compatible gateway usable as a provider.

      TTS_PROVIDER=openai|elevenlabs|custom   default `custom` when TTS_URL is set, else unset -> local TTS
      TTS_URL=<endpoint>
        with openai      -> replaces https://api.openai.com/v1/audio/speech
        with elevenlabs  -> replaces https://api.elevenlabs.io/v1/text-to-speech/<voice>
        with custom      -> required; {voice} {model} in the URL are substituted
      TTS_CUSTOM_URL is accepted as an alias of TTS_URL.
    """
    url = env("TTS_URL") or env("TTS_CUSTOM_URL")
    provider = (env("TTS_PROVIDER") or ("custom" if url else "")).lower()
    if not provider:
        die("TTS_PROVIDER is empty and TTS_URL is not set: nothing to call")
    return provider, url


def endpoint(provider, url, voice, model):
    """The URL to POST to, with {voice} and {model} substituted."""
    def sub(u):
        return u.replace("{voice}", voice).replace("{model}", model)
    if url:
        return sub(url)
    if provider == "openai":
        return "https://api.openai.com/v1/audio/speech"
    if provider == "elevenlabs":
        return f"https://api.elevenlabs.io/v1/text-to-speech/{voice}?output_format=mp3_44100_128"
    die("TTS_PROVIDER=custom needs TTS_URL (the endpoint to POST to)")


def synth(provider, url, text, voice, model, lang, speed, out_path):
    timeout = float(env("TTS_TIMEOUT", "90"))
    fmt = env("TTS_FORMAT", "mp3")
    key = env("TTS_API_KEY") or env("TTS_" + provider.upper() + "_KEY")
    target = endpoint(provider, url, voice, model)
    if provider == "openai":
        if not key:
            die("TTS_API_KEY is empty: put the key in the motion config file (TTS_API_KEY=...)")
        body = {"model": model, "voice": voice, "input": text, "response_format": fmt}
        if speed != 1.0:
            body["speed"] = speed
        raw = request(target, json.dumps(body).encode(), {"Authorization": "Bearer " + key}, timeout)
    elif provider == "elevenlabs":
        if not key:
            die("TTS_API_KEY is empty: put the key in the motion config file (TTS_API_KEY=...)")
        body = {"text": text, "model_id": model}
        if speed != 1.0:
            body["voice_settings"] = {"speed": speed}
        if lang:
            body["language_code"] = lang
        raw = request(target, json.dumps(body).encode(), {"xi-api-key": key}, timeout)
    else:
        payload = env("TTS_PAYLOAD") or '{"text": "{text}", "voice": "{voice}", "model": "{model}"}'
        auth = env("TTS_AUTH", "none")
        headers = {}
        if auth == "bearer":
            headers["Authorization"] = "Bearer " + (key or die("TTS_AUTH=bearer needs TTS_API_KEY"))
        elif auth != "none":
            headers[auth] = key or die(f"TTS_AUTH={auth} needs TTS_API_KEY")
        raw = request(target, fill(payload, text, voice, model, lang, speed).encode(), headers, timeout)
    if raw[:4] == b"RIFF":
        open(out_path, "wb").write(raw)
    else:
        transcode(raw, fmt, out_path)


def main():
    if len(sys.argv) < 4:
        die("usage: tts_provider.py VOICE.json SCRIPT.json OUT_DIR   (TTS_PROVIDER=openai|elevenlabs|custom, TTS_URL)")
    vpath, spath, out = (os.path.abspath(p) for p in sys.argv[1:4])
    provider, url = resolve()
    if provider not in PROVIDERS:
        die(f"TTS_PROVIDER must be one of {', '.join(sorted(PROVIDERS))}; it is '{provider}'")
    V = json.load(open(vpath))
    lines = json.load(open(spath))
    os.makedirs(out, exist_ok=True)
    model = env("TTS_MODEL") or V.get("tts_model") or DEFAULT_MODEL[provider]
    voice = env("TTS_VOICE") or V.get("tts_voice") or DEFAULT_VOICE.get(provider, "")
    lang = env("TTS_LANG") or V.get("language") or "en"
    speed = float(env("TTS_SPEED") or V.get("tts_speed") or 1.0)
    if not 0.25 <= speed <= 4.0:
        die(f"TTS_SPEED {speed} is out of range 0.25-4.0")
    only = [x for x in env("ONLY", "").split(",") if x]
    spk = env("SPEAKER")
    takes = int(env("TAKES", str(V.get("takes_per_line", 1))))
    t0 = int(env("TAKE0", "0"))
    target = endpoint(provider, url, voice, model)          # fail before the first request, not mid-run
    print(f"provider {provider} {target} model {model} voice {voice} speed {speed} takes {takes}", flush=True)
    for ln in lines:
        if only and ln["id"] not in only:
            continue
        if spk and ln.get("speaker") != spk:
            continue
        text = os.environ.get("TTS_TEXT") or ln.get("tts", ln["text"])
        for k in range(takes):
            p = os.path.join(out, f"{ln['id']}_{k + t0}.wav")
            synth(provider, url, text, voice, model, lang, speed, p)
            print(p, flush=True)


if __name__ == "__main__":
    main()
