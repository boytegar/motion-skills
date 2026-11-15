#!/usr/bin/env python3
"""Write one editable scenes/<Scene>.lgv per scene in episode.json (engine "kyfram")."""
import json
import os
import re
import sys

TEMPLATE = '''// {name}: shapes only.
//
// The renderer prepends core/kyfram/lgv/core.lgv (engine helpers), style/<style>/kyfram/PRELUDE.lgv
// (tokens + furniture) and a generated header, so these already exist here:
//   SCENE_ID, BANNER (this beat's taped headline), DUR, FPSR (fps), CAPS (word cues [{w,f,e}])
// plus the helpers: textW, pad, clamp, popText, popPlate, drawTag, drawCaptionRow, pushIn.
// scene(DUR, FPSR) is emitted for you: never call scene() in this file.
//
// Canvas {w}x{h}. Draw every prop inside it and check the coordinates.
'''


def run():
    if not os.path.isfile('episode.json'):
        sys.exit('error: episode.json not found in current directory')
    ep = json.load(open('episode.json', encoding='utf-8'))
    if ep.get('engine') != 'kyfram':
        sys.exit(f"error: episode.json engine is {ep.get('engine')!r}, not 'kyfram'")
    scenes = [s[0] for s in ep.get('scenes', [])]
    if not scenes:
        sys.exit('error: no scenes in episode.json')
    w, h = ep.get('width', 1080), ep.get('height', 1920)
    os.makedirs('scenes', exist_ok=True)
    made = 0
    for name in scenes:
        if not re.fullmatch(r'[A-Za-z][A-Za-z0-9_-]*', name):
            sys.exit(f'error: unsafe scene id {name!r}; use letters, digits, _ or -')
        path = f'scenes/{name}.lgv'
        if os.path.exists(path):
            continue
        with open(path, 'w', encoding='utf-8') as f:
            f.write(TEMPLATE.replace('{name}', name).replace('{w}', str(w)).replace('{h}', str(h)))
        made += 1
    print(f'[kyfram] {len(scenes)} scenes in scenes/, {made} new .lgv stubs generated.')


if __name__ == '__main__':
    run()
