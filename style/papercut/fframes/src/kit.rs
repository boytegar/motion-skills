//! Paper primitives. Everything a scene draws goes through these: a piece of paper is a rounded
//! rect filled with the grain texture, with a darker underside strip and a lighter top strip
//! instead of outlines or gradients, and a drop shadow from the top-left light.
//!
//! All coordinates are composition pixels (1080x1920). `(x, y)` is a top-left corner for
//! [`slab`], a centre for [`disc`] and [`pill`].

use fframes::{svgr, Svgr};

use crate::anim::{alpha, random};
use crate::tokens::{palette as C, FONT};

/// The paper filters, mounted once per frame at the root of the SVG.
///
/// `rough` is the cut edge for props, `torn` the ragged edge for paper scraps, banners and tags,
/// `soft` a small displacement for labels that must stay legible. `id_prefix` keeps the ids unique
/// when two kits are on screen (a nested video, a proof card).
pub fn defs() -> Svgr<'static> {
    svgr!(<defs>
        <filter id="pc-rough" x="-4%" y="-4%" width="108%" height="108%">
            <feTurbulence type="fractalNoise" baseFrequency="0.035" numOctaves="2" seed="7" result="n" />
            <feDisplacementMap in="SourceGraphic" in2="n" scale="3" xChannelSelector="R" yChannelSelector="G" />
        </filter>
        <filter id="pc-torn" x="-6%" y="-10%" width="112%" height="120%">
            <feTurbulence type="fractalNoise" baseFrequency="0.11" numOctaves="3" seed="11" result="n" />
            <feDisplacementMap in="SourceGraphic" in2="n" scale="7" xChannelSelector="R" yChannelSelector="G" />
        </filter>
        <filter id="pc-soft" x="-3%" y="-6%" width="106%" height="112%">
            <feTurbulence type="fractalNoise" baseFrequency="0.08" numOctaves="2" seed="3" result="n" />
            <feDisplacementMap in="SourceGraphic" in2="n" scale="2" xChannelSelector="R" yChannelSelector="G" />
        </filter>
        <filter id="pc-shadow" x="-20%" y="-20%" width="140%" height="150%">
            <feDropShadow dx="2" dy="6" stdDeviation="5" flood-color="#000" flood-opacity="0.38" />
        </filter>
        <linearGradient id="pc-fade-bottom" x1="0" y1="0" x2="0" y2="1">
            <stop offset="55%" stop-color="#000" stop-opacity="0" />
            <stop offset="100%" stop-color="#000" stop-opacity="0.5" />
        </linearGradient>
        <linearGradient id="pc-fade-top" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#000" stop-opacity="0.45" />
            <stop offset="45%" stop-color="#000" stop-opacity="0" />
        </linearGradient>
        <radialGradient id="pc-vignette" cx="50%" cy="46%" r="72%">
            <stop offset="60%" stop-color="#000" stop-opacity="0" />
            <stop offset="100%" stop-color="#000" stop-opacity="0.34" />
        </radialGradient>
    </defs>)
}

/// Which rough edge a piece gets. `Cut` is the default for props; `Torn` for scraps and labels;
/// `None` for pieces that must stay exact (a screen, a chart bar).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Cut,
    Torn,
    Soft,
    None,
}

impl Edge {
    pub fn filter(self) -> &'static str {
        match self {
            Edge::Cut => "url(#pc-rough)",
            Edge::Torn => "url(#pc-torn)",
            Edge::Soft => "url(#pc-soft)",
            Edge::None => "",
        }
    }
}

/// One cut piece of paper: rounded rect with the grain, an optional darker underside and lighter
/// top strip, and an optional drop shadow.
pub struct Slab {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub color: String,
    pub r: f64,
    pub shade: Option<String>,
    pub shade_h: f64,
    pub hi: Option<String>,
    pub hi_h: f64,
    pub rot: f64,
    /// Grain strength, 0..1 (the texture is multiplied, so 0.55 is the kit default).
    pub grain: f64,
    pub edge: Edge,
    pub lift: f64,
    pub opacity: f64,
    /// Rotate around the centre instead of the origin.
    pub center_rot: bool,
}

impl Slab {
    pub fn new(x: f64, y: f64, w: f64, h: f64, color: &str) -> Self {
        Slab {
            x,
            y,
            w,
            h,
            color: color.to_string(),
            r: 4.0,
            shade: None,
            shade_h: 0.18,
            hi: None,
            hi_h: 0.12,
            rot: 0.0,
            grain: 0.55,
            edge: Edge::Cut,
            lift: 0.0,
            opacity: 1.0,
            center_rot: true,
        }
    }

    pub fn r(mut self, r: f64) -> Self {
        self.r = r;
        self
    }

    pub fn radius_pct(mut self) -> Self {
        self.r = -1.0;
        self
    }

    pub fn shade(mut self, color: &str, h: f64) -> Self {
        self.shade = Some(color.to_string());
        self.shade_h = h;
        self
    }

    pub fn hi(mut self, color: &str, h: f64) -> Self {
        self.hi = Some(color.to_string());
        self.hi_h = h;
        self
    }

    pub fn rot(mut self, deg: f64) -> Self {
        self.rot = deg;
        self
    }

    pub fn grain(mut self, g: f64) -> Self {
        self.grain = g;
        self
    }

    pub fn edge(mut self, e: Edge) -> Self {
        self.edge = e;
        self
    }

    pub fn lift(mut self, l: f64) -> Self {
        self.lift = l;
        self
    }

    pub fn opacity(mut self, o: f64) -> Self {
        self.opacity = o;
        self
    }

    /// Render, with `children` drawn inside the slab and clipped to it.
    /// All slab data is owned, so the result borrows only from `children`.
    pub fn render<'a>(&self, children: Svgr<'a>) -> Svgr<'a> {
        let cx = self.x + self.w / 2.0;
        let cy = self.y + self.h / 2.0;
        let transform = if self.rot.abs() > f64::EPSILON {
            if self.center_rot {
                format!("rotate({} {} {})", self.rot, cx, cy)
            } else {
                format!("rotate({} {} {})", self.rot, self.x, self.y)
            }
        } else {
            String::new()
        };
        let grain_fill = grain_fill(&self.color, self.grain);
        let filter = self.edge.filter();
        let shadow = if self.lift > 0.0 {
            Some(format!(
                "drop-shadow({}px {}px {}px rgba(0,0,0,0.38))",
                (self.lift * 0.5).round(),
                self.lift.round(),
                (self.lift * 2.0).round()
            ))
        } else {
            None
        };
        let shade = self.shade.as_deref().map(|c| {
            let hh = self.h * self.shade_h;
            svgr!(<rect x={self.x} y={self.y + self.h - hh} width={self.w} height={hh} fill={c.to_string()} />)
        });
        let hi = self.hi.as_deref().map(|c| {
            let hh = self.h * self.hi_h;
            svgr!(<rect x={self.x} y={self.y} width={self.w} height={hh} fill={c.to_string()} />)
        });
        svgr!(<g
            transform={transform}
            opacity={self.opacity}
            filter={filter}
            style={shadow.unwrap_or_default()}
        >
            <rect
                x={self.x}
                y={self.y}
                width={self.w}
                height={self.h}
                rx={if self.r < 0.0 { self.h / 2.0 } else { self.r }}
                fill={grain_fill}
            />
            {hi}
            {shade}
            <g clip-path={clip_path(self.x, self.y, self.w, self.h, self.r)}>{children}</g>
        </g>)
    }
}

fn clip_path(x: f64, y: f64, w: f64, h: f64, r: f64) -> String {
    let r = if r < 0.0 {
        h / 2.0
    } else {
        r.max(0.0).min(w / 2.0).min(h / 2.0)
    };
    if r <= 0.5 {
        let (x2, y2) = (x + w, y + h);
        format!("path('M{x} {y} H{x2} V{y2} H{x} Z')")
    } else {
        let (xr, yr, xw, yh) = (x + r, y + r, x + w, y + h);
        let (xw_r, yh_r) = (x + w - r, y + h - r);
        format!(
            "path('M{xr} {y} H{xw_r} A{r} {r} 0 0 1 {xw} {yr} V{yh_r} A{r} {r} 0 0 1 {xw_r} {yh} H{xr} A{r} {r} 0 0 1 {x} {yh_r} V{yr} A{r} {r} 0 0 1 {xr} {y} Z')"
        )
    }
}

/// A flat colour with the grain texture multiplied over it, as one `fill` value.
/// The grain image is tiled at 520 px like the kit.
pub fn grain_fill(color: &str, strength: f64) -> String {
    // fframes' media hrefs are resolved by the renderer; a pattern keeps the fill a single value.
    let _ = (color, strength);
    format!("url(#pc-grain-{})", grain_key(color, strength))
}

fn grain_key(color: &str, strength: f64) -> String {
    let pct = (strength * 100.0).round() as i64;
    format!("{}-{pct}", color.trim_start_matches('#'))
}

/// The grain patterns for the colours used this frame, mounted once at the root.
/// Call with every distinct (colour, strength) a scene draws, or use [`grain_defs_for`] with the
/// palette's palette.
pub fn grain_patterns(pairs: &[(&str, f64)], grain_href: &str) -> Svgr<'static> {
    let mut out: Vec<Svgr<'static>> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for (color, strength) in pairs {
        let key = grain_key(color, *strength);
        if seen.contains(&key) {
            continue;
        }
        seen.push(key.clone());
        let opacity = (1.0 - strength).clamp(0.0, 1.0);
        let href = grain_href.to_string();
        let color = (*color).to_string();
        out.push(svgr!(<pattern
            id={format!("pc-grain-{key}")}
            width="520"
            height="520"
            patternUnits="userSpaceOnUse"
        >
            <rect width="520" height="520" fill={color} />
            <image href={href.clone()} width="520" height="520" opacity={opacity} style="mix-blend-mode:multiply" />
        </pattern>));
    }
    svgr!(<defs>{out}</defs>)
}

/// Every palette colour at the default grain, so any scene can use [`grain_fill`] without
/// remembering to declare its patterns first.
pub fn default_grain_patterns(grain_href: &str) -> Svgr<'static> {
    let all = [
        C::CREAM,
        C::CREAM_HI,
        C::CREAM_LO,
        C::TAPE,
        C::INK,
        C::INK_SOFT,
        C::WOOD,
        C::WOOD_LO,
        C::WOOD_HI,
        C::FLOOR,
        C::FLOOR_LO,
        C::BRASS,
        C::BRASS_LO,
        C::METAL,
        C::METAL_LO,
        C::BOARD,
        C::BOARD_LO,
        C::BOARD_HI,
        C::SAGE,
        C::SAGE_LO,
        C::SCREEN,
        C::SCREEN_HI,
        C::SCREEN_TEXT,
        C::SCREEN_DIM,
        C::HERO,
        C::HERO_LO,
        C::CAP,
        C::CAP_LO,
        C::TAG,
        C::TAG_TEXT,
        C::PURPLE,
        C::GO,
        C::GO_DIM,
        C::NOGO,
        C::NOGO_DIM,
        C::AMBER,
        C::AMBER_DIM,
        C::SPARK_YELLOW,
        C::SPARK_PINK,
        C::BRAND,
        C::BRAND_LO,
    ];
    let pairs: Vec<(&str, f64)> = all
        .iter()
        .flat_map(|c| [(*c, 0.55), (*c, 0.0), (*c, 0.35), (*c, 0.8)])
        .collect();
    grain_patterns(&pairs, grain_href)
}

/// A paper disc. `(x, y)` is the centre.
pub fn disc(x: f64, y: f64, d: f64, color: &str, shade: Option<&str>) -> Slab {
    let mut s = Slab::new(x - d / 2.0, y - d / 2.0, d, d, color).r(d / 2.0);
    if let Some(sh) = shade {
        s = s.shade(sh, 0.28);
    }
    s
}

/// A pill label in the bold sans, caps. `(x, y)` is the centre unless `center` is false.
#[allow(clippy::too_many_arguments)]
pub fn pill(
    x: f64,
    y: f64,
    text: &str,
    bg: &str,
    fg: &str,
    size: f64,
    rot: f64,
    center: bool,
) -> Svgr<'static> {
    // Rough text metrics: the label font is Inter 800 with 0.02em tracking. Good enough for layout;
    // the renderer measures the real text with `frame.text_width` when exactness matters.
    let w = text.chars().count() as f64 * size * 0.62 + size * 1.0;
    let h = size * 1.32;
    let left = if center { x - w / 2.0 } else { x };
    let top = if center { y - h / 2.0 } else { y };
    let text = text.to_string();
    let bg = bg.to_string();
    let fg = fg.to_string();
    let (cx, cy) = (left + w / 2.0, top + h / 2.0 + size * 0.36);
    let ls = size * 0.02;
    Slab::new(left, top, w, h, &bg)
        .r(size * 0.28)
        .grain(0.45)
        .rot(rot)
        .render(svgr!(<text
            x={cx}
            y={cy}
            text-anchor="middle"
            font-family={FONT.label}
            font-weight="800"
            font-size={size}
            letter-spacing={ls}
            fill={fg}
        >{text}</text>))
}

/// A cream paper card with grey "text" bars. `(x, y)` is the top-left corner, like [`Slab`].
pub fn card(
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    _lines: usize,
    rot: f64,
    color: &str,
    _seed: &str,
) -> Slab {
    Slab::new(x, y, w, h, color)
        .r(3.0)
        .rot(rot)
        .shade(C::CREAM_LO, 0.06)
}

/// The grey text bars for [`card`], drawn separately so a scene can place them.
pub fn card_lines(x: f64, y: f64, w: f64, h: f64, lines: usize, seed: &str) -> Svgr<'static> {
    let mut out: Vec<Svgr<'static>> = Vec::new();
    for i in 0..lines {
        let step = (26.0_f64).min(h * 0.66 / lines.max(1) as f64);
        let top = y + h * 0.2 + i as f64 * step;
        let bw = w * (0.55 + random(&format!("{seed}{i}")) * 0.3);
        out.push(
            svgr!(<rect x={x + w * 0.1} y={top} width={bw} height="9" rx="4" fill="#9B9184" />),
        );
    }
    svgr!(<g>{out}</g>)
}

/// A cream paper scrap with a torn edge: the banner, a sticky note, a torn label.
pub fn scrap(x: f64, y: f64, w: f64, h: f64, color: &str, rot: f64) -> Slab {
    Slab::new(x, y, w, h, color)
        .r(3.0)
        .rot(rot)
        .edge(Edge::Torn)
}

/// A strip of kraft tape, the kit's `<TapeCorners>` piece.
pub fn tape_strip(x: f64, y: f64, w: f64, h: f64, rot: f64, opacity: f64) -> Svgr<'static> {
    Slab::new(x, y, w, h, C::TAPE)
        .r(2.0)
        .rot(rot)
        .edge(Edge::Torn)
        .opacity(opacity)
        .grain(0.5)
        .render(Svgr::empty())
}

/// Tape over the two top corners of a card, as the kit's `<TapeCorners>` does.
pub fn tape_corners(x: f64, y: f64, w: f64, len: f64) -> Svgr<'static> {
    let a = tape_strip(x - len * 0.25, y - len * 0.18, len, len * 0.34, -35.0, 0.8);
    let b = tape_strip(
        x + w - len * 0.75,
        y - len * 0.18,
        len,
        len * 0.34,
        35.0,
        0.8,
    );
    svgr!(<g>{a}{b}</g>)
}

/// A paper wrapper for a whole prop: one rough edge and one shadow over several slabs.
/// `children` are already positioned.
pub fn paper<'a>(lift: f64, torn: bool, children: Svgr<'a>) -> Svgr<'a> {
    let filter = if torn { Edge::Torn } else { Edge::Cut };
    svgr!(<g
        filter={filter.filter()}
        style={format!("drop-shadow({0}px {1}px {2}px rgba(0,0,0,0.38))", (lift * 0.5).round(), lift.round(), (lift * 2.0).round())}
    >{children}</g>)
}

/// Pop-in: a spring scale from 0 with a small tilt that settles, invisible before `at`.
pub fn pop_in<'a>(
    frame: usize,
    at: usize,
    fps: usize,
    seed: &str,
    origin: (f64, f64),
    children: Svgr<'a>,
) -> Svgr<'a> {
    if frame < at {
        return Svgr::empty();
    }
    let s = crate::anim::pop_at(frame, at, fps);
    let tilt = (random(seed) - 0.5) * 8.0 * (1.0 - s);
    svgr!(<g
        transform={format!(
            "translate({} {}) scale({s}) rotate({tilt} {} {}) translate({} {})",
            origin.0,
            origin.1,
            origin.0,
            origin.1,
            -origin.0,
            -origin.1
        )}
    >{children}</g>)
}

/// Slide-in: eased translate from `(dx, dy)` to zero over `dur` frames.
pub fn slide_in<'a>(
    frame: usize,
    at: usize,
    dur: usize,
    dx: f64,
    dy: f64,
    children: Svgr<'a>,
) -> Svgr<'a> {
    if frame < at {
        return Svgr::empty();
    }
    let u = 1.0 - crate::anim::out_back(crate::anim::step(frame, at, at + dur, 1), 1.25);
    svgr!(<g transform={format!("translate({} {})", dx * u, dy * u)}>{children}</g>)
}

/// Stop-motion boil for screen-space paper overlays: hold 2 frames, then jump.
pub fn boil<'a>(frame: usize, amp: f64, rot: f64, seed: &str, children: Svgr<'a>) -> Svgr<'a> {
    let step = frame / 2;
    let tx = (random(&format!("{seed}x{step}")) - 0.5) * 2.0 * amp;
    let ty = (random(&format!("{seed}y{step}")) - 0.5) * 2.0 * amp;
    let r = (random(&format!("{seed}r{step}")) - 0.5) * 2.0 * rot;
    svgr!(<g transform={format!("translate({tx} {ty}) rotate({r} 540 960)")}>{children}</g>)
}

/// The full-frame grain at 7% over everything: the film's paper tooth.
pub fn grain_overlay(frame: usize, grain_href: &str) -> Svgr<'static> {
    let step = frame / 2;
    let tx = (random(&format!("g{step}")) - 0.5) * 60.0 - 216.0;
    let ty = (random(&format!("h{step}")) - 0.5) * 60.0 - 384.0;
    let href = grain_href.to_string();
    svgr!(<g opacity="0.07" style="mix-blend-mode:multiply">
        <image href={href} x={tx} y={ty} width="1512" height="2688" preserveAspectRatio="xMidYMid slice" />
    </g>)
}

/// A soft vignette, the film's edge falloff.
/// The kit's vignette at full strength. Prefer [`crate::look::vignette`] when you want to dial it.
pub fn plain_vignette() -> Svgr<'static> {
    svgr!(<rect x="0" y="0" width="1080" height="1920" fill="url(#pc-vignette)" />)
}

/// A sparkle burst: six four-point stars flying out from `(x, y)` over 12 frames from `at`.
pub fn sparkle_burst(
    frame: usize,
    at: usize,
    x: f64,
    y: f64,
    seed: &str,
    spread: f64,
) -> Svgr<'static> {
    if frame < at || frame > at + 12 {
        return Svgr::empty();
    }
    let f = (frame - at) as f64;
    let cols = [C::SPARK_YELLOW, C::SPARK_PINK, "#FFFFFF"];
    let mut out: Vec<Svgr<'static>> = Vec::new();
    for i in 0..6 {
        let a = (i as f64 / 6.0) * std::f64::consts::TAU + random(&format!("{seed}{i}"));
        let d = (34.0 + 34.0 * (f / 12.0)) * spread;
        let px = x + a.cos() * d;
        let py = y + a.sin() * d;
        let scale = (f / 12.0 * std::f64::consts::PI).sin();
        let color = cols[i % 3];
        let rot = f * 15.0;
        out.push(
            svgr!(<g transform={format!("translate({px} {py}) rotate({rot}) scale({scale})")}>
            <path d="M0-10 L2-2 L10 0 L2 2 L0 10 L-2 2 L-10 0 L-2-2Z" fill={color} />
        </g>),
        );
    }
    svgr!(<g>{out}</g>)
}

/// Radio waves: three arcs pulsing out from `(x, y)`.
pub fn radio_waves(
    frame: usize,
    x: f64,
    y: f64,
    dir: f64,
    color: &str,
    from: usize,
) -> Svgr<'static> {
    if frame < from {
        return Svgr::empty();
    }
    let f = frame - from;
    let mut out: Vec<Svgr<'static>> = Vec::new();
    for i in 0..3 {
        let p = ((f + i * 5) % 15) as f64 / 15.0;
        let r = 14.0 + p * 46.0;
        let x1 = r * (-0.7_f64).cos();
        let y1 = r * (-0.7_f64).sin();
        let x2 = r * 0.7_f64.cos();
        let y2 = r * 0.7_f64.sin();
        out.push(svgr!(<path
            d={format!("M {x1} {y1} A {r} {r} 0 0 1 {x2} {y2}")}
            stroke={color.to_string()}
            stroke-width="7"
            stroke-linecap="round"
            fill="none"
            opacity={1.0 - p}
        />));
    }
    svgr!(<g transform={format!("translate({x} {y}) rotate({dir})")}>{out}</g>)
}

/// A rubber-stamp mark: an inked double-bordered label that lands with a thump.
pub fn stamp_mark(
    frame: usize,
    at: usize,
    x: f64,
    y: f64,
    text: &str,
    color: &str,
    size: f64,
    rot: f64,
) -> Svgr<'static> {
    if frame < at {
        return Svgr::empty();
    }
    let s = crate::anim::lerp(
        1.5,
        1.0,
        crate::anim::out_back(crate::anim::step(frame, at, at + 5, 1), 2.0),
    );
    let w = text.chars().count() as f64 * size * 0.66 + size * 0.7;
    let h = size * 1.25;
    let color = color.to_string();
    let text = text.to_string();
    svgr!(<g
        transform={format!("translate({x} {y}) rotate({rot}) scale({s})")}
        opacity="0.92"
        filter="url(#pc-torn)"
    >
        <rect x={-w / 2.0} y={-h / 2.0} width={w} height={h} rx={size * 0.16} fill="none" stroke={color.clone()} stroke-width={size * 0.09} />
        <rect x={-w / 2.0 + size * 0.12} y={-h / 2.0 + size * 0.12} width={w - size * 0.24} height={h - size * 0.24} rx={size * 0.1} fill="none" stroke={color.clone()} stroke-width={size * 0.04} />
        <text x="0" y={size * 0.36} text-anchor="middle" font-family={FONT.banner} font-size={size} fill={color}>{text}</text>
    </g>)
}

/// A hand-drawn red marker ellipse, drawn on over `dur` frames (proof marks).
pub fn draw_on_ellipse(
    at: usize,
    frame: usize,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    color: &str,
    stroke: f64,
    dur: usize,
) -> Svgr<'static> {
    let p = crate::anim::out_cubic(crate::anim::step(frame, at, at + dur, 1));
    if p <= 0.0 {
        return Svgr::empty();
    }
    let l = std::f64::consts::PI * (w + h) * 0.55 * 1.08;
    let d = format!(
        "M {} 14 C {} 10, {} {}, {} {} C -8 {}, 2 8, {} 18",
        w * 0.62 + 12.0,
        w + 30.0,
        w + 22.0,
        h + 10.0,
        w / 2.0 + 12.0,
        h + 10.0,
        h + 12.0,
        w * 0.72 + 12.0
    );
    svgr!(<g transform={format!("translate({} {})", x - w / 2.0 - 12.0, y - h / 2.0 - 12.0)}>
        <path
            d={d}
            stroke={color.to_string()}
            stroke-width={stroke}
            fill="none"
            stroke-linecap="round"
            stroke-dasharray={l}
            stroke-dashoffset={l * (1.0 - p)}
        />
    </g>)
}

/// A simple paper arrow from `(x, y)` to `(tx, ty)`, drawn on over `dur`.
pub fn marker_arrow(
    at: usize,
    frame: usize,
    x: f64,
    y: f64,
    tx: f64,
    ty: f64,
    color: &str,
    stroke: f64,
    dur: usize,
    bend: f64,
) -> Svgr<'static> {
    let p = crate::anim::out_cubic(crate::anim::step(frame, at, at + dur, 1));
    if p <= 0.0 {
        return Svgr::empty();
    }
    let mx = (x + tx) / 2.0 + (ty - y) * bend;
    let my = (y + ty) / 2.0 - (tx - x) * bend;
    let len = ((tx - x).powi(2) + (ty - y).powi(2)).sqrt();
    let ang = (ty - y).atan2(tx - x).to_degrees();
    let head = svgr!(<path
        d="M0 0 L-24 -11 L-24 11 Z"
        fill={color.to_string()}
        transform={format!("translate({tx} {ty}) rotate({ang})")}
    />);
    svgr!(<g>
        <path
            d={format!("M {x} {y} Q {mx} {my} {tx} {ty}")}
            stroke={color.to_string()}
            stroke-width={stroke}
            fill="none"
            stroke-linecap="round"
            stroke-dasharray={len * 1.1}
            stroke-dashoffset={len * 1.1 * (1.0 - p)}
        />
        {if p > 0.85 { head } else { Svgr::empty() }}
    </g>)
}

/// A paper "screen glow": a soft radial light behind a monitor.
pub fn glow(x: f64, y: f64, r: f64, color: &str, opacity: f64) -> Svgr<'static> {
    svgr!(<circle cx={x} cy={y} r={r} fill={alpha(color, opacity)} />)
}
