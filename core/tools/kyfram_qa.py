#!/usr/bin/env python3
"""QA stills for a kyfram episode: every nth frame of the rendered segments -> out/qa/NAME/element-*.jpeg.

  python3 core/tools/kyfram_qa.py NAME [FROM-TO] [NTH=12] [SCALE=1.0]     (from the episode project)

kyfram only writes a still at t=0, so the stills come out of the per-scene segments the renderer
already cached. Each file is named element-<globalFrame // nth>.jpeg so core/tools/sheet.py labels
the sheet with real frame numbers, exactly like the Remotion path.

Needs the segments from a render: run core/tools/render.sh once (or it renders nothing here and says so).
"""
import json
import os
import subprocess
import sys

CACHE = 'out/cache/kyfram/seg'


def die(msg):
    sys.exit(f'kyfram_qa: {msg}')


def total(plan):
    # build_vo.py writes a top-level durationInFrames; a hand-written plan may not.
    if 'durationInFrames' in plan:
        return plan['durationInFrames']
    return sum(sc['durationInFrames'] for sc in plan['scenes'])


def fill_from(plan):
    # scenes are sequential in the concat; build_vo.py writes "from" explicitly.
    at = 0
    for sc in plan['scenes']:
        sc.setdefault('from', at)
        at += sc['durationInFrames']
    return plan


def main():
    args = sys.argv[1:]
    if not args:
        die('usage: kyfram_qa.py NAME [FROM-TO] [NTH=12] [SCALE=1.0]')
    name = args[0]
    if '/' in name or name.startswith('.'):
        die(f'NAME must be a plain folder name, got {name!r}')
    rng = args[1] if len(args) > 1 else ''
    nth = int(args[2]) if len(args) > 2 else 12
    scale = float(args[3]) if len(args) > 3 else 1.0
    if scale <= 0:
        die('SCALE must be > 0')

    if not os.path.isfile('src/plan.json'):
        die('src/plan.json missing: run core/tools/build_vo.py first')
    plan = fill_from(json.load(open('src/plan.json')))
    if not os.path.isdir(CACHE):
        die(f'no rendered segments in {CACHE}: run core/tools/render.sh first')

    lo, hi = 0, total(plan) - 1
    # NAME a scene id and leave the range out: default to that scene's own frames.
    # Remotion's --frames is a global timeline range, so an explicit range always wins.
    if not rng:
        for sc in plan['scenes']:
            if sc['id'] == name:
                lo, hi = sc['from'], sc['from'] + sc['durationInFrames'] - 1
                print(f'  {name}: frames {lo}-{hi} (its own span; pass FROM-TO for a global range)',
                      file=sys.stderr)
                break
    if rng:
        a, _, b = rng.partition('-')
        lo, hi = int(a), (int(b) if b else total(plan) - 1)

    outdir = f'out/qa/{name}'
    subprocess.run(['rm', '-rf', outdir], check=True)
    os.makedirs(outdir, exist_ok=True)

    segs = {}
    for f in os.listdir(CACHE):
        if f.endswith('.mp4'):
            segs[f.rsplit('-', 1)[0]] = os.path.join(CACHE, f)

    made = 0
    for sc in plan['scenes']:
        seg = segs.get(sc['id'])
        if not seg:
            print(f"  skipping {sc['id']}: no cached segment", file=sys.stderr)
            continue
        for gf in range(sc['from'], sc['from'] + sc['durationInFrames']):
            if gf < lo or gf > hi or gf % nth:
                continue
            local = gf - sc['from']
            dest = f'{outdir}/element-{gf // nth:04d}.jpeg'
            vf = f'select=eq(n\\,{local})'
            if scale != 1.0:
                vf += f',scale=iw*{scale}:-1'
            r = subprocess.run(['ffmpeg', '-v', 'error', '-y', '-i', seg, '-vf', vf, '-fps_mode', 'passthrough',
                                '-frames:v', '1', '-q:v', '3', dest], capture_output=True, text=True)
            if r.returncode or not os.path.isfile(dest):
                die(f'frame {gf} of {sc["id"]} failed:\n{r.stderr[-1500:]}')
            made += 1
    if not made:
        die(f'no frames matched range {rng or "all"} (nth={nth})')
    print(f'{made} stills in {outdir}')


if __name__ == '__main__':
    main()
