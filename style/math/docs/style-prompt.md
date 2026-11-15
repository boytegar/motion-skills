# Style Prompt: Math (Manim)

Manim-style mathematical motion graphics: chalkboard dark stage, written equations, smooth easing on 1s.

Mathematical visual explainer. Color-coded formulas, axes and graphs with ShowCreation sweeps, Transform morphs and little pi hero.

## The same style as kyfram `.lgv`

For a kyfram episode (engine `kyfram`) do not restate the look: `style/math/kyfram/PRELUDE.lgv`
already carries the palette and the furniture (`drawBackdrop`, `drawBanner`, `drawCaptions`,
`drawPanel`, `drawDot`, `drawFrame`). Read it first; scene files use its tokens, never raw hex.

What a scene author needs, beyond the shared traps in `$MOTION_LIB/core/kyfram/README.md`:

- This prelude is minimal by design. Papercut is the reference for richer furniture
  (`style/papercut/kyfram/PRELUDE.lgv`); extend this one in the style, never in the scene.
- No style TTFs ship here, so text uses kyfram's built-in bold sans. Do not pass `font:`
  unless you first add a font variable to the prelude and copy the TTF into the scaffold.
- Keep the flat-print/flat-shade rule: solid fills from the token palette, no gradients,
  which also matches what kyfram tweens well (fills tween, strokes stay flat).
