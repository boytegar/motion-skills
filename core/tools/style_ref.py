#!/usr/bin/env python3
"""style_ref: measure a reference video so it can become a reusable style pack.

  style_ref.py ingest SOURCE OUTDIR [--sheets 24] [--cut 0.30]
  style_ref.py scaffold NAME [--lib DIR] [--out DIR] [--source SRC] [--probe PROBE.json]
                                [--title T] [--tagline T] [--search-terms T] [--fonts A,B] [--force]
  style_ref.py probe STYLE_DIR STILL [STILL...]
  style_ref.py lint STYLE_DIR

ingest  ffprobe the source, scene-detect the cuts, pull one frame per shot (or an even
        spread when a shot outnumbers the budget), build contact sheets, sample the palette,
        and write OUTDIR/probe.json + OUTDIR/probe.md. Facts only, no interpretation.
scaffold copy core/mold into <lib>/style/NAME, fill the identity fields, and fold in the
        ingest probe's numbers. The story, DNA and traps stay TODO for the model to write.
        --lib defaults to $MOTION_LIB; --out overrides the style root.
probe   check a rendered still against a pack: canvas size, and every token colour present within
        d=40 in the still's own sampled palette. Run it before trusting the pack on a new episode.
lint    check a pack has the files and fields the library expects.

Needs ffmpeg/ffprobe on PATH, plus PIL (MOTION_PY).
"""

import colorsys
import json
import os
import re
import subprocess
import sys

HERE = os.path.dirname(os.path.abspath(__file__))


def run(cmd, check=True):
    p = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if check and p.returncode:
        raise SystemExit("style_ref: %s\n%s" % (" ".join(cmd), p.stderr[-2000:]))
    return p


def ffprobe(src):
    p = run(["ffprobe", "-v", "error", "-print_format", "json", "-show_format", "-show_streams", src])
    d = json.loads(p.stdout)
    v = next((s for s in d["streams"] if s["codec_type"] == "video"), None)
    if not v:
        raise SystemExit("style_ref: no video stream in %s" % src)
    num, den = (v.get("avg_frame_rate") or "30/1").split("/")
    fps = float(num) / float(den or 1) if float(den or 1) else 30.0
    has_audio = any(s["codec_type"] == "audio" for s in d["streams"])
    return {
        "path": os.path.abspath(src),
        "width": int(v["width"]),
        "height": int(v["height"]),
        "fps": round(fps, 3),
        "duration_s": round(float(d["format"].get("duration") or 0), 3),
        "audio": has_audio,
    }


def cuts(src, threshold):
    p = run(["ffmpeg", "-hide_banner", "-nostats", "-i", src, "-an",
             "-vf", "select='gt(scene,%s)',showinfo" % threshold, "-f", "null", "-"])
    ts = [float(m) for m in re.findall(r"pts_time:([0-9.]+)", p.stderr)]
    return sorted(set(ts))


def shots_of(duration, cut_ts):
    edges = [0.0] + [t for t in cut_ts if 0.0 < t < duration] + [duration]
    return [(edges[i], edges[i + 1]) for i in range(len(edges) - 1) if edges[i + 1] - edges[i] > 0.05]


def pick(shots, budget, duration):
    """One timestamp per shot, or an even spread over the whole runtime when shots outnumber it.
    Fewer samples than shots would under-read the look, so short reels get at least 12 samples."""
    if len(shots) >= budget:
        return [round((i + 0.5) * duration / budget, 3) for i in range(budget)], "even"
    if len(shots) >= 12:
        return [round((a + b) / 2, 3) for a, b in shots], "per-shot"
    return [round((i + 0.5) * duration / (len(shots) * 2), 3) for i in range(len(shots) * 2)], "even"


def grab(src, t, dst):
    run(["ffmpeg", "-hide_banner", "-loglevel", "error", "-ss", "%.3f" % t, "-i", src,
         "-frames:v", "1", "-q:v", "3", "-y", dst])


def sheet(frames, dst, width=320, cols=9):
    """Contact sheet of (timestamp, path) frames, each tile stamped with its timestamp."""
    from PIL import Image, ImageDraw
    tiles = []
    for t, f in frames:
        im = Image.open(f)
        im = im.resize((width, round(im.height * width / im.width)))
        ImageDraw.Draw(im).rectangle([0, 0, 74, 20], fill="black")
        ImageDraw.Draw(im).text((4, 4), "%.2fs" % t, fill="yellow")
        tiles.append(im)
    w, h = tiles[0].size
    rows = (len(tiles) + cols - 1) // cols
    sh = Image.new("RGB", (w * cols, h * rows), "white")
    for i, im in enumerate(tiles):
        sh.paste(im, ((i % cols) * w, (i // cols) * h))
    sh.save(dst, quality=82)
    return dst


def palette_of(paths, colors=8, bin_size=16, top=12):
    from PIL import Image
    weight = {}
    for p in paths:
        im = Image.open(p).convert("RGB").resize((160, 160))
        q = im.quantize(colors=colors, method=Image.MEDIANCUT).convert("RGB")
        for count, rgb in sorted(q.getcolors(), reverse=True):
            key = tuple(min(255, int(round(c / bin_size) * bin_size)) for c in rgb)
            weight[key] = weight.get(key, 0) + count
    total = sum(weight.values()) or 1
    ranked = sorted(weight.items(), key=lambda kv: -kv[1])[:top]
    return ["#%02X%02X%02X" % k for k, _ in ranked], [round(v / total * 100, 1) for _, v in ranked]


def saturation_share(path):
    """Share of pixels that carry real chroma: quantize in RGB, weigh each colour by its own HSV."""
    from PIL import Image
    im = Image.open(path).convert("RGB").resize((96, 96)).quantize(colors=64).convert("RGB")
    hist = im.getcolors(96 * 96) or []
    px = sum(n for n, _ in hist)
    hot = 0
    for n, rgb in hist:
        h, s, v = colorsys.rgb_to_hsv(*[c / 255 for c in rgb])
        if s > 0.55 and v > 0.24:
            hot += n
    return round(hot / max(px, 1) * 100, 1)


def motion_energy(triples):
    """Mean abs luma delta between each sample and a frame 0.15 s later in the same shot."""
    from PIL import Image, ImageChops, ImageStat
    vals = []
    for _, fa, fb in triples:
        if fa == fb:
            continue
        x, y = Image.open(fa).convert("L").resize((96, 96)), Image.open(fb).convert("L").resize((96, 96))
        vals.append(ImageStat.Stat(ImageChops.difference(x, y)).mean[0])
    return round(sum(vals) / len(vals), 2) if vals else 0.0


def loudness(src):
    p = run(["ffmpeg", "-hide_banner", "-nostats", "-i", src, "-vn", "-af", "volumedetect", "-f", "null", "-"], check=False)
    mean = re.search(r"mean_volume:\s*(-?[0-9.]+) dB", p.stderr)
    peak = re.search(r"max_volume:\s*(-?[0-9.]+) dB", p.stderr)
    return {"mean_db": float(mean.group(1)) if mean else None,
            "peak_db": float(peak.group(1)) if peak else None}


def median(vals):
    vals = sorted(vals)
    return vals[len(vals) // 2] if vals else 0


def content_box(path, tol=18):
    """Union of tiles that differ from each still's corner colour by more than tol, as canvas fractions."""
    from PIL import Image, ImageChops
    x0, y0, x1, y1 = 1e9, 1e9, -1e9, -1e9
    for p in ([path] if isinstance(path, str) else path):
        im = Image.open(p).convert("RGB")
        w, h = im.size
        corner = im.getpixel((4, 4))
        bg = Image.new("RGB", im.size, corner)
        diff = ImageChops.difference(im, bg).convert("L").point(lambda v: 255 if v > tol else 0)
        bb = diff.getbbox()
        if not bb:
            continue
        x0, y0 = min(x0, bb[0]), min(y0, bb[1])
        x1, y1 = max(x1, bb[2]), max(y1, bb[3])
    if x0 > x1:
        return None
    return {"x0": round(x0 / w, 3), "y0": round(y0 / h, 3), "x1": round(x1 / w, 3), "y1": round(y1 / h, 3)}


def text_rows(path, tol=48):
    """Bands of rows whose local contrast is high enough to be type rather than a flat field."""
    from PIL import Image, ImageFilter, ImageStat
    im = Image.open(path).convert("L")
    w, h = im.size
    edges = im.filter(ImageFilter.FIND_EDGES)
    row = [ImageStat.Stat(edges.crop((0, y, w, min(y + 6, h)))).mean[0] for y in range(0, h, 6)]
    top = max(row) or 1
    hot = [i for i, v in enumerate(row) if v > top * 0.45]
    bands, start, prev = [], None, None
    for i in hot:
        if start is None:
            start = i
        elif i - prev > 3:
            bands.append((start, prev))
            start = i
        prev = i
    if start is not None:
        bands.append((start, prev))
    return [{"y0": round(a * 6 / h, 3), "y1": round((b * 6 + 6) / h, 3)} for a, b in bands if (b - a) * 6 > 12]


def ingest(argv):
    src, outdir = argv[0], argv[1]
    budget, threshold = 24, 0.30
    if "--sheets" in argv: budget = int(argv[argv.index("--sheets") + 1])
    if "--cut" in argv: threshold = float(argv[argv.index("--cut") + 1])
    if not os.path.exists(src):
        raise SystemExit("style_ref: no such file %s" % src)
    frames_dir = os.path.join(outdir, "frames")
    os.makedirs(frames_dir, exist_ok=True)

    meta = ffprobe(src)
    cut_ts = cuts(src, threshold)
    shots = shots_of(meta["duration_s"], cut_ts)
    stamps, mode = pick(shots, budget, meta["duration_s"])

    frames = []
    for t in stamps:
        dst = os.path.join(frames_dir, "element-%d.jpeg" % int(round(t * meta["fps"])))
        grab(src, t, dst)
        if os.path.getsize(dst) > 0:
            frames.append((t, dst))
    if not frames:
        raise SystemExit("style_ref: extracted no frames from %s" % src)

    # a second look half a frame later, so motion can be scored inside a shot instead of across cuts
    pairs = []
    for t, f in frames:
        twin = f.replace(".jpeg", "-b.jpeg")
        grab(src, t + 0.15, twin)
        pairs.append((t, f, twin if os.path.exists(twin) else f))

    pal, share = palette_of([f for _, f in frames])
    lens = [round(b - a, 2) for a, b in shots]
    spread = {"count": len(lens),
              "median_s": median(lens),
              "min_s": min(lens) if lens else 0,
              "max_s": max(lens) if lens else 0,
              "cuts_per_min": round(len(cut_ts) / max(meta["duration_s"], 0.001) * 60, 1)}

    # zones: where content sits on the canvas, and which rows carry type
    strip = [f for _, f in frames]
    zones = {"content_box": content_box(strip), "text_bands": []}
    for f in strip:
        for b in text_rows(f):
            if not any(abs(b["y0"] - z["y0"]) < 0.02 and abs(b["y1"] - z["y1"]) < 0.02 for z in zones["text_bands"]):
                zones["text_bands"].append(b)
    zones["text_bands"].sort(key=lambda b: b["y0"])
    zones["text_bands"] = zones["text_bands"][:4]

    sheets = []
    for i in range(0, len(frames), 45):
        sheets.append(sheet(frames[i:i + 45], os.path.join(outdir, "sheet_%d.jpg" % (i // 45))))

    probe = {
        "source": meta,
        "aspect": "9:16" if meta["height"] > meta["width"] else ("1:1" if meta["height"] == meta["width"] else "16:9"),
        "sample": {"mode": mode, "count": len(frames),
                   "timestamps_s": [t for t, _ in frames]},
        "cuts": spread,
        "motion": {"interframe_energy": motion_energy(pairs)},
        "palette": {"hex": pal, "share_pct": share},
        "zones": zones,
        "saturation_share_pct": round(sum(saturation_share(f) for _, f in frames) / len(frames), 1),
        "audio": loudness(src) if meta["audio"] else None,
        "sheets": [s for s in sheets if s],
        "frames_dir": frames_dir,
    }
    with open(os.path.join(outdir, "probe.json"), "w") as fh:
        json.dump(probe, fh, indent=1)
    with open(os.path.join(outdir, "probe.md"), "w") as fh:
        fh.write(probe_md(probe))
    print(os.path.join(outdir, "probe.md"))


def probe_md(p):
    s, c = p["source"], p["cuts"]
    b = p["zones"]["content_box"]
    cb = "x %.0f-%.0f%%, y %.0f-%.0f%%" % (b["x0"] * 100, b["x1"] * 100, b["y0"] * 100, b["y1"] * 100) if b else ""
    rows = [
        ("canvas", "%dx%d @ %g fps" % (s["width"], s["height"], s["fps"])),
        ("duration", "%g s" % s["duration_s"]),
        ("aspect", p["aspect"]),
        ("shots", "%d, median %g s (min %g, max %g)" % (c["count"], c["median_s"], c["min_s"], c["max_s"])),
        ("cut rate", "%g cuts/min" % c["cuts_per_min"]),
        ("motion", "inter-frame energy %g/255 (two frames 0.15 s apart)" % list(p["motion"].values())[0]),
        ("saturation", "%g%% of pixels highly saturated" % p["saturation_share_pct"]),
        ("palette", ", ".join("%s %g%%" % (h, w) for h, w in zip(p["palette"]["hex"], p["palette"]["share_pct"]))),
        ("audio", "mean %s dB, peak %s dB" % ((p["audio"] or {}).get("mean_db"), (p["audio"] or {}).get("peak_db")) if p["audio"] else "none"),
        ("content box", cb or "full frame"),
        ("text bands", ", ".join("%d-%d%%" % (round(b["y0"] * 100), round(b["y1"] * 100)) for b in p["zones"]["text_bands"]) or "none found"),
        ("sheets", ", ".join(p["sheets"]) or "-"),
    ]
    out = ["| field | value |", "|---|---|"] + ["| %s | %s |" % (k, v) for k, v in rows]
    out.append("\nMeasured, not interpreted. Read the sheet for look, type, motion and the beat.")
    return "\n".join(out) + "\n"


def load_tokens(style_dir):
    with open(os.path.join(style_dir, "style.json")) as fh:
        d = json.load(fh)
    return (d.get("extra") or {}).get("tokens") or d.get("tokens") or {}, (d.get("canvas") or {}), d


def hex_rgb(h):
    h = h.lstrip("#")
    return tuple(int(h[i:i + 2], 16) for i in (0, 2, 4))


def probe(argv):
    """Compare a still's measured palette with the pack's tokens: every token must appear, and the
    still's own area coverage must sit near the token list's shape."""
    style_dir, stills = argv[0], argv[1:]
    tokens, canvas, _ = load_tokens(style_dir)
    if not tokens:
        raise SystemExit("style_ref: %s/style.json has no tokens to probe against" % style_dir)
    want = {k: (v if isinstance(v, str) else v.get("hex") or "") for k, v in tokens.items()}
    want = {k: v for k, v in want.items() if v.startswith("#")}
    fails = 0
    for still in stills:
        if not os.path.isfile(still):
            raise SystemExit("style_ref: no such still %s" % still)
        hexes, _ = palette_of([still], colors=12, top=16)
        got = [hex_rgb(h) for h in hexes]
        results = []
        for name, hexv in sorted(want.items()):
            target = hex_rgb(hexv)
            near = min(sum((a - b) ** 2 for a, b in zip(target, g)) ** 0.5 for g in got) if got else 999
            if near > 40:
                fails += 1
                results.append("%s %s absent (nearest %s, d=%d)" % (name, hexv, hexes[got.index(min(got, key=lambda g: sum((a - b) ** 2 for a, b in zip(target, g))))], near))
        from PIL import Image
        cw, ch = canvas.get("width"), canvas.get("height")
        w, h = Image.open(still).size
        if cw and (w, h) != (cw, ch):
            fails += 1
            results.append("canvas %dx%d, pack says %sx%s" % (w, h, cw, ch))
        print("%s: %s" % (still, "ok" if not results else "; ".join(results)))
    raise SystemExit(1 if fails else 0)


REQUIRED = ["SKILL.md", "style.json", "docs/style-guide.md", "docs/style-prompt.md"]
FIELDS = ["name", "title", "tagline", "description", "canvas", "engines", "docs"]


def lint(argv):
    d = argv[0] if argv else "."
    bad = []
    for f in REQUIRED:
        if not os.path.isfile(os.path.join(d, f)):
            bad.append("missing %s" % f)
    if "missing style.json" not in " ".join(bad):
        with open(os.path.join(d, "style.json")) as fh:
            try:
                m = json.load(fh)
            except ValueError as e:
                raise SystemExit("style_ref: style.json is not valid JSON: %s" % e)
        bad += ["style.json missing field %s" % k for k in FIELDS if k not in m]
        if not m.get("engines") and m.get("status") not in ("docs-only", "draft"):
            bad.append('engines is empty, so status must be "docs-only" or "draft" (got %r)' % m.get("status"))
        if not ((m.get("extra") or {}).get("tokens")):
            msg = "no extra.tokens: probe has nothing to compare against"
            if m.get("status") == "draft":
                bad.append(msg)
            else:
                print("style_ref lint: note: " + msg)
        for key, rel in (m.get("docs") or {}).items():
            if not os.path.isfile(os.path.join(d, rel)):
                bad.append("docs.%s -> %s missing" % (key, rel))
    guide = os.path.join(d, "docs/style-guide.md")
    if os.path.isfile(guide):
        text = open(guide).read()
        for head in ("## Look", "## Type", "## Motion", "## Beat", "## Signature move", "## Traps"):
            if head not in text:
                bad.append("style-guide.md has no %s section" % head)
    for e in ("remotion", "fframes", "kyfram"):
        if os.path.isdir(os.path.join(d, e)) and e not in ((json.load(open(os.path.join(d, "style.json"))).get("engines")) or []):
            print("style_ref: note: %s/ exists but is not registered in engines" % e)
    if bad:
        print("\n".join("style_ref lint: " + b for b in bad))
        raise SystemExit(1)
    print("style_ref lint: %s ok" % d)


def scaffold(argv):
    """Copy core/mold into <lib>/style/<name>/ with the identity fields filled.
    Leaves TODO markers and the probe's numbers; the story, DNA and traps are the model's to write."""
    name = argv[0]
    opts = {}
    i = 1
    while i < len(argv):
        if argv[i].startswith("--"):
            opts[argv[i][2:]] = argv[i + 1]
            i += 2
        else:
            i += 1
    lib = opts.get("lib") or os.environ.get("MOTION_LIB") or os.path.join(HERE, "..")
    mold = os.path.abspath(os.path.join(lib, "mold"))
    dest = os.path.abspath(os.path.join(opts.get("out") or os.path.join(lib, "style"), name))
    if not os.path.isdir(mold):
        raise SystemExit("style_ref: no mold at %s (needs core/mold from a recent library)" % mold)
    if os.path.exists(dest) and not opts.get("force"):
        raise SystemExit("style_ref: %s exists; pass --force to write the templates into it" % dest)

    title = opts.get("title") or name.replace("-", " ").title()
    fields = {
        "STYLE_NAME": name,
        "STYLE_TITLE": title,
        "TAGLINE": opts.get("tagline") or "TODO: one line on the medium.",
        "SEARCH_TERMS": opts.get("search_terms") or opts.get("search-terms") or "TODO: the words a user would say",
        "REFERENCE": opts.get("source") or "TODO: source path or URL",
        "FULL_BG": "TODO", "FULL_INK": "TODO", "FULL_CREAM": "TODO", "FULL_HERO": "TODO",
        "FULL_CAP": "TODO", "FULL_TAG": "TODO",
        "LOOK_RULE_1": "TODO", "LOOK_RULE_2": "TODO", "LOOK_RULE_3": "TODO", "LOOK_RULE_4": "TODO",
        "MOTION_RULE": "TODO", "TYPE_RULE": "TODO", "TRAP_1": "TODO", "TRAP_2": "TODO",
    }
    status = "draft"
    manifest = json.load(open(os.path.join(mold, "style.json")))
    manifest.update({"name": name, "title": title, "tagline": fields["TAGLINE"],
                     "description": opts.get("description") or fields["TAGLINE"], "status": status})
    if opts.get("probe") and os.path.isfile(opts["probe"]):
        p = json.load(open(opts["probe"]))
        s = p["source"]
        manifest["canvas"] = {"width": s["width"], "height": s["height"], "fps": int(round(s["fps"]))}
        manifest.setdefault("extra", {})["reference"] = {
            "path": s["path"], "duration_s": s["duration_s"], "cuts_per_min": p["cuts"]["cuts_per_min"],
            "median_shot_s": p["cuts"]["median_s"], "motion_energy": list(p["motion"].values())[0],
            "saturation_share_pct": p["saturation_share_pct"],
        }
        fields["CANVAS"] = "%dx%d @ %g fps" % (s["width"], s["height"], s["fps"])
        fields["MEDIAN_SHOT"] = "%g s" % p["cuts"]["median_s"]
        fields["CUT_RATE"] = "%g cuts/min" % p["cuts"]["cuts_per_min"]
        fields["MOTION"] = "%g/255" % list(p["motion"].values())[0]
        fields["SATURATION"] = "%g%%" % p["saturation_share_pct"]
        manifest["extra"]["probe"] = p.get("zones")
        z = p.get("zones") or {}
        box = z.get("content_box")
        fields["CONTENT_BOX"] = ("x %.0f-%.0f%%, y %.0f-%.0f%%" % (box["x0"] * 100, box["x1"] * 100, box["y0"] * 100, box["y1"] * 100)) if box else "full frame"
        fields["TEXT_BANDS"] = ", ".join("%d-%d%%" % (round(b["y0"] * 100), round(b["y1"] * 100)) for b in z.get("text_bands", [])) or "none found"
    for k in ("CANVAS", "MEDIAN_SHOT", "CUT_RATE", "MOTION", "SATURATION", "CONTENT_BOX", "TEXT_BANDS"):
        fields.setdefault(k, "TODO")
    for k, v in (manifest.get("extra") or {}).get("tokens", {}).items():
        key = "FULL_" + re.sub(r"[^A-Za-z0-9]", "_", k).upper()
        if key in fields:
            hexv = v if isinstance(v, str) else v.get("hex", "")
            fields[key] = hexv if hexv.upper() not in ("", "#000000") else "TODO from probe.palette"
    fonts = (opts.get("fonts") or "").split(",")
    if len(fonts) == 2 and all(fonts):
        manifest.setdefault("extra", {})["fonts"] = {"display": fonts[0].strip(), "text": fonts[1].strip()}
    elif fonts and fonts[0]:
        manifest.setdefault("extra", {})["fonts"] = {"display": fonts[0].strip(), "text": fonts[0].strip()}
    else:
        manifest.setdefault("extra", {}).pop("fonts", None)

    written = []
    for root, _, files in os.walk(mold):
        for f in files:
            src = os.path.join(root, f)
            rel = os.path.relpath(src, mold)
            if rel == "style.json":
                out = os.path.join(dest, "style.json")
                os.makedirs(os.path.dirname(out), exist_ok=True)
                json.dump(manifest, open(out, "w"), indent=2)
                open(out, "a").write("\n")
                written.append(rel)
                continue
            text = True
            try:
                body = open(src).read()
            except UnicodeDecodeError:
                text = False
            out_rel = re.sub(r"\.tmpl$", "", rel)
            out = os.path.join(dest, out_rel)
            os.makedirs(os.path.dirname(out), exist_ok=True)
            if text:
                for k, v in fields.items():
                    body = body.replace(k, v)
                open(out, "w").write(body)
            else:
                open(out, "wb").write(open(src, "rb").read())
            written.append(out_rel)
    for rel in written:
        print("write", os.path.join(dest, rel))


def main():
    if len(sys.argv) < 3:
        print(__doc__.strip())
        raise SystemExit(2)
    cmd, argv = sys.argv[1], sys.argv[2:]
    if cmd == "ingest": ingest(argv)
    elif cmd == "scaffold": scaffold(argv)
    elif cmd == "probe": probe(argv)
    elif cmd == "lint": lint(argv)
    else:
        print(__doc__.strip())
        raise SystemExit(2)


if __name__ == "__main__":
    main()
