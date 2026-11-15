# Style Prompt: Pixel Art

Low-res pixel art drawn at native raster grids and upscaled nearest-neighbour to 1080x1920.

Retro pixel art style. Dithered shading, 5x7 bitmap typography, CRT/scanline/LCD effects and resolution eras that tell time.

## The same style as kyfram `.lgv`

For a kyfram episode (engine `kyfram`) do not restate the look: `style/pixel/kyfram/PRELUDE.lgv`
already carries the palette and the furniture (`drawBackdrop`, `drawBanner`, `drawCaptions`,
`drawPanel`, `drawDot`, `drawFrame`). Read it first; scene files use its tokens, never raw hex.

What a scene author needs, beyond the shared traps in `$MOTION_LIB/core/kyfram/README.md`:

- This prelude is minimal by design. Papercut is the reference for richer furniture
  (`style/papercut/kyfram/PRELUDE.lgv`); extend this one in the style, never in the scene.
- No style TTFs ship here, so text uses kyfram's built-in bold sans. Do not pass `font:`
  unless you first add a font variable to the prelude and copy the TTF into the scaffold.
- Keep the flat-print/flat-shade rule: solid fills from the token palette, no gradients,
  which also matches what kyfram tweens well (fills tween, strokes stay flat).
