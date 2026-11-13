//! Whole-frame camera and film treatments: weave, fibre, dirt, lamp flicker, halftone, graph paper.
//!
//! These wrap a scene's content rather than drawing props, and the Remotion originals live in
//! `../remotion/kit/paper/look.tsx`. Each one returns a `<g>` the scene composes at the root.

use fframes::{svgr, Svgr};

use crate::anim::{clamp01, map, random};
use crate::kit::Slab;
use crate::tokens::palette as C;

/// A blend mode the SVG renderer understands (`mix-blend-mode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Blend {
    Normal,
    Screen,
    Multiply,
    Overlay,
    SoftLight,
    ColorDodge,
}

impl Blend {
    #[must_use]
    pub const fn css(self) -> &'static str {
        match self {
            Blend::Normal => "normal",
            Blend::Screen => "screen",
            Blend::Multiply => "multiply",
            Blend::Overlay => "overlay",
            Blend::SoftLight => "soft-light",
            Blend::ColorDodge => "color-dodge",
        }
    }
}

/// A tiled image fill, clipped to a rect. Used for the texture overlays.
fn image_fill(href: &str, w: f64, h: f64, opacity: f64, blend: Option<Blend>) -> Svgr<'static> {
    let href = href.to_string();
    match blend {
        Some(b) => svgr!(<image
            href={href}
            width={w}
            height={h}
            opacity={opacity}
            preserveAspectRatio="xMidYMid slice"
            style={format!("mix-blend-mode:{}", b.css())}
        />),
        None => svgr!(<image
            href={href}
            width={w}
            height={h}
            opacity={opacity}
            preserveAspectRatio="xMidYMid slice"
        />),
    }
}

/// Projector weave: a slow sub-pixel drift plus a tiny exposure wobble, so a locked-off frame
/// still breathes. `amp` scales the drift in pixels, `flicker` the exposure wobble.
pub fn film_weave<'a>(
    f: usize,
    amp: f64,
    flicker: f64,
    seed: &str,
    children: Svgr<'a>,
) -> Svgr<'a> {
    let a = if amp <= 0.0 { 1.0 } else { amp };
    let step = f / 2;
    let r = |k: &str| random(&format!("{seed}{k}{step}")) * 2.0 - 1.0;
    let fx = f as f64;
    let dx = ((fx / 37.0).sin() * 0.9 + r("x") * 0.8) * a;
    let dy = ((fx / 29.0).cos() * 0.8 + r("y") * 0.7) * a;
    let rot = ((fx / 53.0).sin() * 0.05 + r("r") * 0.045) * a;
    let exposure = 1.0 + flicker * (r("e") * 0.02 + (fx / 6.5).sin() * 0.008);
    // Overscan so the drifting edge never shows a hard line at the frame boundary.
    let over = 1.0 + 0.012 * a.max(1.0);
    let cx = crate::tokens::W as f64 / 2.0;
    let cy = crate::tokens::H as f64 / 2.0;
    svgr!(<g
        transform={format!("translate({dx:.2} {dy:.2}) rotate({rot:.3} {cx} {cy}) translate({} {})", cx * (1.0 - over), cy * (1.0 - over))}
        style={format!("filter:brightness({exposure:.4})")}
    >{children}</g>)
}

/// A felt-fibre overlay: high-contrast, screened over the frame.
pub fn felt_fibre(href: &str, opacity: f64, blend: Blend) -> Svgr<'static> {
    let w = crate::tokens::W as f64;
    let h = crate::tokens::H as f64;
    svgr!(<g style={format!("mix-blend-mode:{}", blend.css())} opacity={opacity}>
        {image_fill(href, w, h, 1.0, None)}
    </g>)
}

/// Dust and smudges on the lens: fades in at `at` and holds, optionally masked to a circle.
pub fn lens_dirt(
    f: usize,
    at: usize,
    href: &str,
    opacity: f64,
    blend: Blend,
    mask: Option<(f64, f64, f64)>,
) -> Svgr<'static> {
    let k = map(f as f64, at as f64, at as f64 + 10.0, 0.0, 1.0);
    if k <= 0.0 {
        return Svgr::empty();
    }
    let w = crate::tokens::W as f64;
    let h = crate::tokens::H as f64;
    let image = image_fill(href, w, h, opacity * k, None);
    let group = svgr!(<g style={format!("mix-blend-mode:{}", blend.css())}>{image}</g>);
    match mask {
        None => group,
        Some((mx, my, r)) => {
            // A radial alpha ramp, so the dirt falls off instead of ending on a hard circle.
            let id = format!("pc-dirt-mask-{at}");
            let def = svgr!(<defs>
                <radialGradient id={id.clone()} gradientUnits="userSpaceOnUse" cx={mx} cy={my} r={r}>
                    <stop offset="0%" stop-color="#fff" stop-opacity="1" />
                    <stop offset="50%" stop-color="#fff" stop-opacity="0.5" />
                    <stop offset="100%" stop-color="#fff" stop-opacity="0" />
                </radialGradient>
                <mask id={format!("{id}-m")} maskUnits="userSpaceOnUse" x="0" y="0" width={w} height={h}>
                    <rect x="0" y="0" width={w} height={h} fill={format!("url(#{id})")} />
                </mask>
            </defs>);
            svgr!(<g>{def}<g mask={format!("url(#{id}-m)")}>{group}</g></g>)
        }
    }
}

/// An old lamp's brightness at `f`: a slow sine plus occasional dips that fade back over ~16 frames.
/// Returns a factor around 1.0; clamp it where you use it, as the Remotion original does.
#[must_use]
pub fn flicker_at(f: usize, seed: &str, depth: f64, every: usize) -> f64 {
    let ff = f as f64;
    let mut v = 1.0
        + 0.02 * (ff / 4.3 + random(&format!("{seed}ph")) * 6.0).sin()
        + 0.012 * (ff / 1.7).sin();
    let every = every.max(1);
    let slot = f / every;
    for s in slot.saturating_sub(1)..=slot {
        if random(&format!("{seed}p{s}")) > 0.6 {
            continue;
        }
        let start =
            (s * every) as f64 + random(&format!("{seed}t{s}")) * (every.saturating_sub(12) as f64);
        let u = ff - start;
        if (0.0..16.0).contains(&u) {
            let shape = if u < 2.0 {
                u / 2.0
            } else {
                (-(u - 2.0) / 4.5).exp()
            };
            v -= depth * shape * (0.8 + 0.2 * (u * 1.7).cos());
        }
    }
    v.max(0.3)
}

/// Wrap `children` in the lamp flicker: opacity plus a matching exposure drop.
pub fn lamp_flicker<'a>(
    f: usize,
    seed: &str,
    depth: f64,
    every: usize,
    children: Svgr<'a>,
) -> Svgr<'a> {
    let v = flicker_at(f, seed, depth, every);
    let brightness = if v < 0.99 {
        format!("filter:brightness({:.3})", 0.82 + 0.18 * v)
    } else {
        String::new()
    };
    svgr!(<g opacity={v.min(1.0)} style={brightness}>{children}</g>)
}

/// A halftone screen that screens over the frame at `opacity`. Kept separate from `children`
/// because the tone shift and the dot screen are two layers in the Remotion original.
pub fn halftone_screen(
    f: usize,
    at: usize,
    href: &str,
    opacity: f64,
    blend: Blend,
) -> Svgr<'static> {
    let k = map(f as f64, at as f64, at as f64 + 6.0, 0.0, 1.0);
    if k <= 0.0 {
        return Svgr::empty();
    }
    let w = crate::tokens::W as f64;
    let h = crate::tokens::H as f64;
    svgr!(<g style={format!("mix-blend-mode:{}", blend.css())} opacity={opacity * k}>
        {image_fill(href, w, h, 1.0, None)}
    </g>)
}

/// The tone part of [`halftone_screen`]: desaturate, warm and lift contrast as the screen arrives.
pub fn halftone_tone(f: usize, at: usize, tone: f64, children: Svgr<'_>) -> Svgr<'_> {
    let k = map(f as f64, at as f64, at as f64 + 6.0, 0.0, 1.0);
    let g = tone * k;
    svgr!(<g style={format!(
        "filter:grayscale({g:.3}) sepia({:.3}) contrast({:.3})",
        0.25 * k,
        1.0 + 0.08 * k
    )}>{children}</g>)
}

/// A sheet of graph paper: a paper slab holding a texture, with an optional tint and dim.
/// `(x, y)` is the top-left; pass `None` to fill the frame.
#[allow(clippy::too_many_arguments)]
pub fn graph_paper<'a>(
    x: Option<f64>,
    y: Option<f64>,
    w: f64,
    h: f64,
    rot: f64,
    href: &str,
    tint: Option<&str>,
    dim: f64,
    children: Svgr<'a>,
) -> Svgr<'a> {
    match (x, y) {
        (Some(x), Some(y)) if w > 0.0 && h > 0.0 => {
            let tex = image_fill(href, w, h, 1.0, None);
            let shade = match tint {
                Some(t) => {
                    let t = t.to_string();
                    svgr!(<rect x="0" y="0" width={w} height={h} fill={t} style="mix-blend-mode:multiply" />)
                }
                None => Svgr::empty(),
            };
            let dark = if dim > 0.0 {
                svgr!(<rect x="0" y="0" width={w} height={h} fill="#000" opacity={dim.min(1.0)} />)
            } else {
                Svgr::empty()
            };
            let inner = svgr!(<g>{tex}{shade}{dark}{children}</g>);
            let slab = Slab::new(x, y, w, h, C::CREAM).r(4.0).rot(rot).lift(8.0);
            slab.render(inner)
        }
        _ => {
            svgr!(<g>{image_fill(href, crate::tokens::W as f64, crate::tokens::H as f64, 1.0, None)}{children}</g>)
        }
    }
}

/// A warm vignette at the frame edges, from the kit's `pc-vignette` radial gradient.
pub fn vignette(opacity: f64) -> Svgr<'static> {
    svgr!(<rect
        x="0"
        y="0"
        width={crate::tokens::W as f64}
        height={crate::tokens::H as f64}
        fill="url(#pc-vignette)"
        opacity={clamp01(opacity)}
    />)
}

/// A top or bottom fade, from the kit's `pc-fade-top` / `pc-fade-bottom` gradients.
pub fn edge_fade(top: bool, opacity: f64) -> Svgr<'static> {
    let id = if top { "pc-fade-top" } else { "pc-fade-bottom" };
    svgr!(<rect
        x="0"
        y="0"
        width={crate::tokens::W as f64}
        height={crate::tokens::H as f64}
        fill={format!("url(#{id})")}
        opacity={clamp01(opacity)}
    />)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{is_blank, markup};

    #[test]
    fn blend_names_match_css() {
        assert_eq!(Blend::SoftLight.css(), "soft-light");
        assert_eq!(Blend::ColorDodge.css(), "color-dodge");
    }

    #[test]
    fn weave_drift_is_bounded_and_deterministic() {
        let a = markup(film_weave(20, 1.0, 1.0, "w", Svgr::empty()));
        let b = markup(film_weave(20, 1.0, 1.0, "w", Svgr::empty()));
        assert_eq!(a, b, "same frame must render identically");
        assert!(a.contains("translate("));
        assert!(!is_blank(film_weave(21, 1.0, 1.0, "w", Svgr::empty())));
    }

    #[test]
    fn flicker_stays_in_range() {
        for f in 0..200 {
            let v = flicker_at(f, "lamp", 0.4, 42);
            assert!((0.3..=1.1).contains(&v), "frame {f} gave {v}");
        }
    }

    #[test]
    fn lens_dirt_is_empty_before_it_starts() {
        assert!(is_blank(lens_dirt(
            0,
            10,
            "dirt.jpg",
            0.3,
            Blend::Screen,
            None
        )));
        assert!(!is_blank(lens_dirt(
            20,
            10,
            "dirt.jpg",
            0.3,
            Blend::Screen,
            None
        )));
    }

    #[test]
    fn masked_dirt_emits_a_mask() {
        let out = markup(lens_dirt(
            20,
            0,
            "dirt.jpg",
            0.3,
            Blend::Screen,
            Some((540.0, 960.0, 560.0)),
        ));
        assert!(out.contains("<mask"), "got {out}");
        assert!(out.contains("radialGradient"));
    }
}
