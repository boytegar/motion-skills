# keyframe-demo — a kyfram episode from scaffold to mp4

A 20 s, 5-scene vertical reel about kyfram itself: the hook is "you change one
number and the whole cut moves", and the last shot is the CTA. It is the smallest
honest example of the `kyfram` engine end to end: papercut style, one `.lgv` per
scene, generated headers, per-scene cache, voice mux.

There is no audio in this example. `motion-hook`'s `SAY` lines are the real
narration text; the timing in `narration/script.json` is estimated at 3 words a
second by `core/tools/placeholder_vo.py`. Record a real take and the cuts land on
it without touching a single scene file — that is the point of the engine.

## Build it

```sh
cd /path/to/your-series
$MOTION_LIB/core/new_episode.sh . 01-keyframe KeyframeDemo kyfram-demo \
  --style papercut --engine kyfram

E=episodes/01-keyframe
cp -r $MOTION_LIB/../examples/keyframe-demo/narration/script.json  $E/narration/
cp $MOTION_LIB/../examples/keyframe-demo/episode.json               $E/video/
cp $MOTION_LIB/../examples/keyframe-demo/scenes/*.lgv               $E/video/scenes/
cp $MOTION_LIB/../examples/keyframe-demo/STORYBOARD.md              $E/video/

cd $E/narration && python3 $MOTION_LIB/core/tools/placeholder_vo.py
cd $E/video     && python3 $MOTION_LIB/core/tools/build_vo.py
python3 $MOTION_LIB/core/tools/kyfram_stubs.py      # no-ops: the scenes already exist
python3 $MOTION_LIB/core/tools/render_kyfram.py --voice
# -> out/kyfram-demo.mp4, 604 frames, 20.13 s, 1080x1920 @ 30 fps
```

Check a beat: `bash $MOTION_LIB/core/tools/qa.sh S03Plan` (scene ids default to
their own frame span). Preview the exact compiled script:
`kyfram view out/cache/kyfram/build/S03Plan.lgv`.

## What each scene shows

| Scene | Frames | What it proves |
|---|---|---|
| `S01Hook` | 106 | the hero is already mid-slide on frame 0, a `1` tag stamps on the word "number", the camera pulls back then pushes into the cut |
| `S02Code` | 93 | four bars type in, one per stressed word, on a static screen plate |
| `S03Plan` | 132 | two cards and an arrow: `scene.lgv` (shapes only) vs `plan.json` (`DUR`, `FPSR`, `BANNER`), then "retime = edit this" |
| `S04Drift` | 123 | a counter ticks `38 → 64 → 90` and turns red, `drifts silently` gets struck, `STOP` stamps |
| `S05Cta` | 150 | ten cards fan out with a stagger, `10 / 10 styles`, `git clone motion` |

Every scene file is shapes only. `DUR`, `FPSR`, `BANNER` and `CAPS` come from the
generated header, so retiming any beat is an `episode.json` / `script.json` edit.

## Layout rules this example learned the hard way

- **1480 is off limits.** Captions sit at `CAPTION_Y = 1470`. Anything else above
  ~1380 or it lands inside the caption band (S05's CTA plate had to move from
  1420 to 1275).
- **`popText` is centre-anchored.** Two `popText` calls at similar x/y overlap
  into unreadable mush — that is how the `BANNER` row in S03 first shipped.
- **A shape off-canvas renders nothing and raises nothing.** S01's motion streak
  was halfway off the right edge for two rounds; only a crop of the real frame
  showed it. Check crops, not just contact sheets.
- **Repeat `color` on every keyframe.** The prelude helpers already do; anything
  you hand-write must too, or the shape draws white.
