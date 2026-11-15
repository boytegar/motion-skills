# Style Prompt: Risograph

Three-ink risograph print on cream paper: halftone screens, overprints, misregistration and ink dropouts on 2s.

Physical print shop look. Pure CMY/spot ink separation, halftone screens at distinct angles, mechanical misregistration and paper grain.

## The same style as kyfram `.lgv`

For a kyfram episode (engine `kyfram`) do not restate the look: `style/riso/kyfram/PRELUDE.lgv`
already carries the palette and the furniture (`drawBackdrop`, `drawBanner`, `drawCaptions`,
`drawPanel`, `drawDot`, `drawFrame`). Read it first; scene files use its tokens, never raw hex.

What a scene author needs, beyond the shared traps in `$MOTION_LIB/core/kyfram/README.md`:

- This prelude is minimal by design. Papercut is the reference for richer furniture
  (`style/papercut/kyfram/PRELUDE.lgv`); extend this one in the style, never in the scene.
- No style TTFs ship here, so text uses kyfram's built-in bold sans. Do not pass `font:`
  unless you first add a font variable to the prelude and copy the TTF into the scaffold.
- Keep the flat-print/flat-shade rule: solid fills from the token palette, no gradients,
  which also matches what kyfram tweens well (fills tween, strokes stay flat).
