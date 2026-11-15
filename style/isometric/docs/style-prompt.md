# Style Prompt: Isometric

Calm, precise product line art in 2:1 dimetric isometric with constant hairlines and spring settles.

Clean isometric product illustration style on light/dark ground. 2.0px constant hairlines, rounded slabs, stacked layers and spring motion on 1s.

## The same style as kyfram `.lgv`

For a kyfram episode (engine `kyfram`) do not restate the look: `style/isometric/kyfram/PRELUDE.lgv`
already carries the palette and the furniture (`drawBackdrop`, `drawBanner`, `drawCaptions`,
`drawPanel`, `drawDot`, `drawFrame`). Read it first; scene files use its tokens, never raw hex.

What a scene author needs, beyond the shared traps in `$MOTION_LIB/core/kyfram/README.md`:

- This prelude is minimal by design. Papercut is the reference for richer furniture
  (`style/papercut/kyfram/PRELUDE.lgv`); extend this one in the style, never in the scene.
- No style TTFs ship here, so text uses kyfram's built-in bold sans. Do not pass `font:`
  unless you first add a font variable to the prelude and copy the TTF into the scaffold.
- Keep the flat-print/flat-shade rule: solid fills from the token palette, no gradients,
  which also matches what kyfram tweens well (fills tween, strokes stay flat).
