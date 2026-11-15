#!/usr/bin/env python3
"""Write one editable scenes/<Scene>.lgv per scene in episode.json (engine "kyfram").

Also scaffolds the SFX cue sheet folder (tools/cue_sheet.d/) so scene agents can
write their SFX rows alongside the scene file — identical pattern to remotion/fframes.
Each tools/cue_sheet.d/<Scene>.json starts as an empty list; agents fill it with
[[word_prefix, nth_match, sfx_name, offset_frames], ...] entries picked from
core/audio/CATALOG.md.  cues.py reads every *.json in that folder automatically.
"""
import json
import os
import re
import sys

TEMPLATE = '''// {name}: shapes only.
//
// The renderer prepends core/kyfram/lgv/core.lgv (engine helpers), style/<style>/kyfram/PRELUDE.lgv
// (tokens + furniture) and a generated header, so these already exist here:
//   SCENE_ID, BANNER (this beat's taped headline), DUR, FPSR (fps), CAPS (word cues [{{w,f,e}}])
// plus the helpers: textW, pad, clamp, popText, popPlate, drawTag, drawCaptionRow, pushIn.
// scene(DUR, FPSR) is emitted for you: never call scene() in this file.
//
// Canvas {w}x{h}. Draw every prop inside it and check the coordinates.
//
// SFX for this scene: edit tools/cue_sheet.d/{name}.json
//   [[word_prefix, nth_match, sfx_name, offset_frames], ...]
//   e.g. [["Bang", 0, "hit", -6], ["kalender", 0, "pop", 0]]
//   SFX names from $MOTION_LIB/core/audio/CATALOG.md.
//
// Cut-transition SFX in episode.json "cutSfx": {{"*": "cut", "S03Name": "whip"}}.
//   Match the storyboard cut kind: whip->"whip", tearAway->"rip", tapePull->"slide",
//   zoomThrough->"zoom", page-flip->"page-flip". Hard cut: omit or "cut".
'''

CUE_HEADER = '''\
// SFX cue rows for {name} live in this file.
// Format: {{"{name}": [[word_prefix, nth_match, sfx_name, offset_frames], ...]}}
// Refer to the scene-file header or core/audio/CATALOG.md for field definitions.
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
    os.makedirs('tools/cue_sheet.d', exist_ok=True)

    # Ensure a base cue_sheet.json exists (empty dict = no entries).
    if not os.path.isfile('tools/cue_sheet.json'):
        with open('tools/cue_sheet.json', 'w', encoding='utf-8') as f:
            f.write('{}\n')

    made_lgv = 0
    made_cue = 0
    for name in scenes:
        if not re.fullmatch(r'[A-Za-z][A-Za-z0-9_-]*', name):
            sys.exit(f'error: unsafe scene id {name!r}; use letters, digits, _ or -')

        lgv = f'scenes/{name}.lgv'
        if not os.path.exists(lgv):
            with open(lgv, 'w', encoding='utf-8') as f:
                f.write(TEMPLATE.replace('{name}', name)
                                .replace('{w}', str(w))
                                .replace('{h}', str(h)))
            made_lgv += 1

        cue = f'tools/cue_sheet.d/{name}.json'
        if not os.path.exists(cue):
            with open(cue, 'w', encoding='utf-8') as f:
                f.write(json.dumps({name: []}, indent=2) + '\n')
            made_cue += 1

    print(f'[kyfram] {len(scenes)} scenes in scenes/, {made_lgv} new .lgv stubs, '
          f'{made_cue} new cue_sheet.d stubs.')


if __name__ == '__main__':
    run()
