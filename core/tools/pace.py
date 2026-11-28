#!/usr/bin/env python3
"""Measure the spoken pace of a narration line and write a per-word timing curve.

Works from the audio alone (stdlib only: `wave` + `struct`), so no ASR, numpy
or soundfile is needed.

  usage (from an episode's narration/ folder):
    python3 $MOTION_LIB/core/tools/pace.py --script script.json --out voice/pace_narrator.json
    python3 $MOTION_LIB/core/tools/pace.py voice/final_narrator/*.wav      # table on stdout

  --script  script file; its ids name the lines (words come from the transcript)
  --text    transcript json: voice/asr.py output, or {id | path: {"text": ...}}
            (default: voice/asr_<ROLE>_final.json, then voice/asr_<ROLE>_takes.json)
  --chosen  voice/chosen_<ROLE>.json {id: path}      (default voice/chosen_<ROLE>.json)
  --dir     folder of per-line wavs                  (default voice/final_<ROLE>)
  --out     write {path|id: {wps, speech, lead, tail, dur, words: [[s, e], ...]}} here
  --target  preferred pace, words per second (default 3.3); lines slower or
            faster than 12% of it are reported at the end

  Each word is placed on the audio's energy envelope: loudness is cut at the
  voiced threshold and the speech stretches are split in proportion to word
  length. A pause of 140 ms or more becomes a gap between two words, so a slow
  or paused delivery produces a slow, gappy cue track instead of an even one.
"""
import argparse, json, math, os, re, struct, sys, wave

FLOOR_DB = -42.0      # relative to the 95th percentile of a file
VOICED_DB = -70.0     # absolute floor, for a near-silent file
WIN = 0.010           # energy window, seconds
PAD = 0.060           # a word keeps at most this much of the pause before it
MIN_PAUSE = 0.140
EDGE_TRIM = 0.100     # drop edge fragments shorter than this
DEFAULT_TARGET, WPS_MAX = 3.3, 5.5


def envelope(path):
    """(per-window RMS in dBFS, window seconds, file seconds) from channel 0."""
    with wave.open(path) as w:
        sr, ch, width, n = w.getframerate(), w.getnchannels(), w.getsampwidth(), w.getnframes()
        if width != 2:
            raise SystemExit(f"{path}: pace.py needs 16-bit PCM (ffmpeg -c:a pcm_s16le)")
        pcm = w.readframes(n)
    n_samp = len(pcm) // 2
    if ch > 1:
        pcm = b"".join(pcm[i:i + 2] for i in range(0, len(pcm), 2 * ch))   # channel 0
        n_samp = len(pcm) // 2
    if not n_samp:
        return [], WIN, 0.0
    per = max(1, int(sr * WIN))
    levels = []
    for i in range(0, n_samp, per):
        end = min(n_samp, i + per)
        chunk = struct.unpack(f"<{end - i}h", pcm[2 * i:2 * end])
        rms = math.sqrt(sum(x * x for x in chunk) / len(chunk)) if chunk else 0.0
        levels.append(20.0 * math.log10(rms / 32768.0) if rms > 0 else -120.0)
    return levels, per / sr, n_samp / sr


def voiced(levels, step):
    """Bounding stretches of speech [start, end] in seconds."""
    if not levels:
        return []
    ref = sorted(levels)[int(0.95 * (len(levels) - 1))]
    cut = max(ref + FLOOR_DB, VOICED_DB)
    runs, on, start = [], False, 0.0
    for i, db in enumerate(levels + [cut - 1]):
        t = i * step
        if db > cut and not on:
            on, start = True, t
        elif db <= cut and on:
            on = False
            runs.append([start, t])
    while runs and runs[0][1] - runs[0][0] < EDGE_TRIM:
        runs.pop(0)
    while runs and runs[-1][1] - runs[-1][0] < EDGE_TRIM:
        runs.pop()
    return runs


def spans_of(runs):
    """Join runs whose pause is too short to be a word boundary."""
    out = []
    for s, e in runs:
        if out and s - out[-1][1] < MIN_PAUSE:
            out[-1][1] = e
        else:
            out.append([s, e])
    return out


def word_times(words, spans, dur):
    """Words over the speech stretches, by length; pauses stay wordless."""
    n = len(words)
    if not n:
        return []
    weights = [len(re.sub(r"\W", "", w)) + 2 for w in words]
    if not spans:
        return [[round(i * dur / n, 3), round((i + 1) * dur / n, 3)] for i in range(n)]
    speech = sum(e - s for s, e in spans) or 1e-6
    # words per stretch, by its share of the speech; the share caps a fast delivery
    if n / speech > WPS_MAX:
        n = max(1, int(round(speech * WPS_MAX)))
        spans = [spans[0]] if n == 1 else spans
        weights = weights[:n]
    cum, counts, used = 0.0, [], 0
    for i, (s, e) in enumerate(spans):
        cum += e - s
        want = n - used if i == len(spans) - 1 else int(round(n * cum / speech)) - used
        want = max(0, min(n - used, want))
        counts.append(want)
        used += want
    times, wi = [], 0
    for (s, e), c in zip(spans, counts):
        if not c:
            continue
        ws = weights[wi:wi + c]
        wi += c
        total = float(sum(ws))
        t = s
        room = e - s
        for k, wt in enumerate(ws):
            d = room * wt / total
            times.append([round(t, 3), round(min(t + d, e), 3)])
            t += d
    while len(times) < n:
        times.append([round(spans[-1][1], 3), round(spans[-1][1], 3)])
    return times


def measure(path, text="", target=DEFAULT_TARGET):
    if not path or not os.path.exists(path):
        return None
    levels, step, dur = envelope(path)
    if dur <= 0:
        return None
    runs = voiced(levels, step)
    spans = spans_of(runs) if runs else [[0.0, dur]]
    words = text.split()
    lead = max(0.0, spans[0][0] - PAD)
    tail = max(0.0, dur - spans[-1][1] - PAD)
    speech = sum(e - s for s, e in spans)
    wps = round(len(words) / speech, 2) if words and speech > 0 else 0.0
    out = {"wps": wps, "speech": round(speech, 3), "lead": round(lead, 3), "tail": round(tail, 3),
           "dur": round(dur, 3), "words": word_times(words, spans, dur) if words else []}
    if wps:
        out["advice"] = "slower" if wps < target * 0.88 else "faster" if wps > target * 1.12 else "ok"
    return out


def main():
    ap = argparse.ArgumentParser(description="measure narration pace and word timing")
    ap.add_argument("wavs", nargs="*", help="WAVs to report on stdout")
    ap.add_argument("--script")
    ap.add_argument("--text")
    ap.add_argument("--chosen", default="voice/chosen_narrator.json")
    ap.add_argument("--dir", default="voice/final_narrator")
    ap.add_argument("--role", default="narrator")
    ap.add_argument("--out")
    ap.add_argument("--target", type=float, default=DEFAULT_TARGET)
    ap.add_argument("--num", help="numeral map json, applied to the script text")
    a = ap.parse_args()

    if a.wavs:
        for p in a.wavs:
            r = measure(p, target=a.target)
            if r:
                print(f"{os.path.basename(p):42s} {r['speech']:5.2f}s speech of {r['dur']:5.2f}s"
                      f"  {r.get('advice', '-')}")
            else:
                print(f"{os.path.basename(p):42s} unreadable")
        return

    tx_path = a.text or ""
    if not tx_path:
        for cand in (f"voice/asr_{a.role}_final.json", f"voice/asr_{a.role}_takes.json"):
            if os.path.exists(cand):
                tx_path = cand
                break
    asr = json.load(open(tx_path)) if tx_path and os.path.exists(tx_path) else {}
    tx = {}
    for k, v in asr.items():
        text = v["text"] if isinstance(v, dict) else str(v)
        tx[os.path.splitext(os.path.basename(k))[0]] = text
        tx[k] = text

    script = []
    if a.script and os.path.exists(a.script):
        script = json.load(open(a.script))
    elif os.path.exists("script.json"):
        script = json.load(open("script.json"))
    if not script:
        print("no script text: pass --script or run from a narration folder", file=sys.stderr)
        sys.exit(2)
    num = json.load(open(a.num)) if a.num and os.path.exists(a.num) else {}
    chosen = {}
    if os.path.exists(a.chosen):
        chosen = json.load(open(a.chosen))

    result, slow, fast, missing = {}, [], [], []
    for ln in script:
        lid, text = ln["id"], ln["text"]
        for k, v in num.items():
            text = text.replace(k, v)
        path = chosen.get(lid) or os.path.join(a.dir, f"{lid}.wav")
        r = measure(path, tx.get(lid) or tx.get(path) or text, a.target)
        if not r:
            missing.append(lid)
            continue
        result[path] = r
        result[lid] = r
        (slow if r.get("advice") == "slower" else fast if r.get("advice") == "faster" else []).append(lid)
    if a.out:
        os.makedirs(os.path.dirname(os.path.abspath(a.out)), exist_ok=True)
        json.dump(result, open(a.out, "w"), indent=1)
    for lid in [l["id"] for l in script]:
        r = result.get(lid)
        if r:
            print(f"{lid:16s} {r['wps']:5.2f} wps  {r['dur']:5.2f}s  speech {r['speech']:5.2f}s  {r.get('advice', 'ok')}")
    if missing:
        print("no audio:", ", ".join(missing), file=sys.stderr)
    if slow:
        print(f"slow ({a.target} wps target): {', '.join(slow)} -> shorten the line or raise the voice's tempo")
    if fast:
        print(f"fast: {', '.join(fast)} -> split the line or lower the voice's tempo")
    if a.out:
        print("pace ->", a.out)


if __name__ == "__main__":
    main()
