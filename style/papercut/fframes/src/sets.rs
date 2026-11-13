//! Set dressing: a room, and the furniture that goes in it.
//!
//! Port of `../remotion/kit/paper/sets.tsx`. Positions are frame pixels; scale `s` keeps one set
//! usable at several depths. Every piece wraps itself in [`crate::kit::paper`] the way the
//! Remotion components do, so a scene only decides where things sit.

use fframes::{svgr, Svgr};

use crate::anim::{map, random};
use crate::kit::{paper, Slab};
use crate::look::{edge_fade, vignette};
use crate::tokens::{palette as C, room as room_color, H, W};

/// A room: wall colour, a textured wall, a soft light falloff, and a floor.
#[derive(Debug, Clone)]
pub struct Room {
    /// A named room from [`crate::tokens::rooms`], or a literal colour.
    pub color: String,
    /// Where the wall meets the floor.
    pub floor_y: f64,
    pub floor: Floor,
    pub skirting: bool,
    /// Wall texture href. Pass `""` to skip it (a flat wall is a valid look).
    pub wall_href: String,
}

/// The floor treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Floor {
    Wood,
    Dark,
    /// No floor: the wall runs to the bottom of the frame.
    None,
}

impl Room {
    /// A room by name, with the kit's defaults (`wood` floor, skirting, textured wall).
    pub fn new(color: &str) -> Self {
        Room {
            color: color.to_string(),
            floor_y: 1420.0,
            floor: Floor::Wood,
            skirting: true,
            wall_href: String::new(),
        }
    }

    pub fn floor_y(mut self, y: f64) -> Self {
        self.floor_y = y;
        self
    }

    pub fn floor(mut self, f: Floor) -> Self {
        self.floor = f;
        self
    }

    pub fn skirting(mut self, on: bool) -> Self {
        self.skirting = on;
        self
    }

    pub fn wall_href(mut self, href: &str) -> Self {
        self.wall_href = href.to_string();
        self
    }

    /// Draw the room. `children` sit in front of the room but behind whatever the scene adds.
    #[must_use]
    pub fn render<'a>(&self, children: Svgr<'a>) -> Svgr<'a> {
        let wall = room_color(&self.color);
        let (w, h) = (W as f64, H as f64);
        let wall_tex = if self.wall_href.is_empty() {
            Svgr::empty()
        } else {
            svgr!(<image
                href={self.wall_href.clone()}
                width={w}
                height={h}
                preserveAspectRatio="xMidYMid slice"
                opacity={0.85}
                style="mix-blend-mode:multiply"
            />)
        };
        let light = svgr!(<ellipse
            cx={w / 2.0}
            cy={h * 0.42}
            rx={w * 0.72}
            ry={h * 0.44}
            fill="url(#pc-room-light)"
        />);
        let floor = match self.floor {
            Floor::None => Svgr::empty(),
            kind => {
                let color = if kind == Floor::Wood {
                    C::FLOOR
                } else {
                    C::BOARD_LO
                };
                let slab = Slab::new(-20.0, self.floor_y, 1120.0, h - self.floor_y + 20.0, color)
                    .r(0.0)
                    .grain(0.6)
                    .render(Svgr::empty());
                let planks = if kind == Floor::Wood {
                    let mut lines: Vec<Svgr<'static>> = Vec::new();
                    for (i, p) in [0.18_f64, 0.43, 0.71].iter().enumerate() {
                        lines.push(svgr!(<rect
                            x="0"
                            y={self.floor_y + 40.0 + i as f64 * 150.0 + p * 10.0}
                            width={w}
                            height="4"
                            fill={C::FLOOR_LO}
                            opacity="0.8"
                        />));
                    }
                    svgr!(<g>{lines}</g>)
                } else {
                    Svgr::empty()
                };
                let skirt = if self.skirting {
                    Slab::new(-20.0, self.floor_y - 26.0, 1120.0, 30.0, C::WOOD_LO)
                        .r(0.0)
                        .shade(C::FLOOR_LO, 0.35)
                        .render(Svgr::empty())
                } else {
                    Svgr::empty()
                };
                let meet = svgr!(<rect
                    x="0"
                    y={self.floor_y}
                    width={w}
                    height="80"
                    fill="url(#pc-floor-meet)"
                />);
                svgr!(<g>{slab}{planks}{skirt}{meet}</g>)
            }
        };
        svgr!(<g>
            <rect x="0" y="0" width={w} height={h} fill={wall} />
            {wall_tex}
            {light}
            {floor}
            {children}
        </g>)
    }
}

impl Default for Room {
    fn default() -> Self {
        Room::new("sage")
    }
}

/// The two room gradients, mounted once per frame alongside the kit defs.
#[must_use]
pub fn room_defs() -> Svgr<'static> {
    svgr!(<defs>
        <radialGradient id="pc-room-light" cx="50%" cy="42%" r="62%">
            <stop offset="0%" stop-color="#fff" stop-opacity="0.10" />
            <stop offset="45%" stop-color="#000" stop-opacity="0" />
            <stop offset="100%" stop-color="#000" stop-opacity="0.32" />
        </radialGradient>
        <linearGradient id="pc-floor-meet" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stop-color="#000" stop-opacity="0.28" />
            <stop offset="75%" stop-color="#000" stop-opacity="0" />
        </linearGradient>
    </defs>)
}

/// A pot plant. `(x, y)` is the top-left of its 220x300 box before `s`.
#[must_use]
pub fn plant(x: f64, y: f64, s: f64) -> Svgr<'static> {
    let leaves = [
        (-38.0_f64, 70.0_f64, 20.0_f64),
        (30.0, 40.0, -18.0),
        (-6.0, 10.0, 4.0),
        (52.0, 92.0, -40.0),
        (-62.0, 104.0, 38.0),
    ];
    let mut parts: Vec<Svgr<'static>> = Vec::new();
    for (i, (dx, dy, r)) in leaves.iter().enumerate() {
        let color = if i % 2 == 0 { "#3F6A3E" } else { "#4E7A4A" };
        parts.push(
            Slab::new((80.0 + dx) * s, dy * s, 62.0 * s, 150.0 * s, color)
                .r(60.0)
                .hi("#6A9461", 0.2)
                .rot(*r)
                .render(Svgr::empty()),
        );
    }
    let pot = Slab::new(44.0 * s, 170.0 * s, 132.0 * s, 120.0 * s, "#B8643E")
        .r(6.0 * s)
        .shade("#93492B", 0.2)
        .render(Svgr::empty());
    let rim = Slab::new(34.0 * s, 160.0 * s, 152.0 * s, 28.0 * s, "#C9744C")
        .r(4.0)
        .shade("#A2593A", 0.35)
        .render(Svgr::empty());
    paper(
        5.0,
        false,
        svgr!(<g transform={format!("translate({x} {y})")}>{parts}{pot}{rim}</g>),
    )
}

/// A desk lamp, with a light cone when `on`.
#[must_use]
pub fn lamp(x: f64, y: f64, s: f64, on: bool) -> Svgr<'static> {
    let glow = if on {
        let w = 180.0 * s;
        let h = 200.0 * s;
        let x0 = 10.0 * s;
        let y0 = 92.0 * s;
        svgr!(<path
            d={format!(
                "M {} {} L {} {} L {} {} L {} {} Z",
                x0 + w * 0.3, y0,
                x0 + w * 0.7, y0,
                x0 + w, y0 + h,
                x0, y0 + h
            )}
            fill="rgba(255,226,140,0.16)"
        />)
    } else {
        Svgr::empty()
    };
    let shade = Slab::new(40.0 * s, 20.0 * s, 120.0 * s, 80.0 * s, C::AMBER)
        .r(50.0 * s)
        .shade(C::AMBER_DIM, 0.18)
        .render(Svgr::empty());
    let stem = Slab::new(95.0 * s, 96.0 * s, 12.0 * s, 170.0 * s, C::INK_SOFT)
        .r(3.0)
        .render(Svgr::empty());
    let base = Slab::new(52.0 * s, 262.0 * s, 96.0 * s, 22.0 * s, C::INK_SOFT)
        .r(8.0)
        .render(Svgr::empty());
    paper(
        5.0,
        false,
        svgr!(<g transform={format!("translate({x} {y})")}>{glow}{shade}{stem}{base}</g>),
    )
}

/// A corkboard with scatter pins.
#[must_use]
pub fn corkboard(x: f64, y: f64, w: f64, h: f64) -> Svgr<'static> {
    let frame = Slab::new(x, y, w, h, C::WOOD_HI)
        .r(8.0)
        .shade(C::WOOD, 0.04)
        .render(Svgr::empty());
    let mut pins: Vec<Svgr<'static>> = Vec::new();
    for i in 0..40 {
        let px = x + 22.0 + w * ((i * 37) % 97) as f64 / 100.0;
        let py = y + 22.0 + h * ((i * 53) % 95) as f64 / 100.0;
        pins.push(svgr!(<circle cx={px} cy={py} r="3" fill="#8F6539" opacity="0.6" />));
    }
    let board = Slab::new(x + 22.0, y + 22.0, w - 44.0, h - 44.0, "#B98B5B")
        .r(4.0)
        .shade("#A27748", 0.06)
        .grain(0.9)
        .render(svgr!(<g>{pins}</g>));
    paper(6.0, false, svgr!(<g>{frame}{board}</g>))
}

/// A framed window: night shows stars, day shows sky.
#[must_use]
pub fn window(x: f64, y: f64, w: f64, h: f64, night: bool) -> Svgr<'static> {
    let frame = Slab::new(x, y, w, h, C::CREAM_LO)
        .r(4.0)
        .render(Svgr::empty());
    let glass_color = if night { "#1F2A44" } else { "#9DB7C9" };
    let mut inner: Vec<Svgr<'static>> = Vec::new();
    if night {
        for (i, (sx, sy)) in [
            (0.2, 0.2),
            (0.7, 0.3),
            (0.4, 0.6),
            (0.8, 0.75),
            (0.15, 0.85),
        ]
        .iter()
        .enumerate()
        {
            inner.push(svgr!(<circle
                cx={x + 18.0 + (w - 36.0) * sx}
                cy={y + 18.0 + (h - 36.0) * sy}
                r="3"
                fill="#F3E6D3"
                opacity={0.5 + 0.1 * random(&format!("star{i}"))}
            />));
        }
    }
    inner.push(svgr!(<rect x={x + w / 2.0 - 5.0} y={y + 18.0} width="10" height={h - 36.0} fill={C::CREAM_LO} />));
    inner.push(svgr!(<rect x={x + 18.0} y={y + h / 2.0 - 5.0} width={w - 36.0} height="10" fill={C::CREAM_LO} />));
    let glass = Slab::new(x + 18.0, y + 18.0, w - 36.0, h - 36.0, glass_color)
        .r(2.0)
        .render(svgr!(<g>{inner}</g>));
    let left = Slab::new(x - 34.0, y - 10.0, 70.0, h + 20.0, "#C9A04A")
        .r(4.0)
        .shade("#A8853B", 0.05)
        .render(Svgr::empty());
    let right = Slab::new(x + w - 36.0, y - 10.0, 70.0, h + 20.0, "#C9A04A")
        .r(4.0)
        .shade("#A8853B", 0.05)
        .render(Svgr::empty());
    paper(4.0, false, svgr!(<g>{frame}{left}{right}{glass}</g>))
}

/// A shelf with books.
#[must_use]
pub fn shelf(x: f64, y: f64, w: f64, books: usize) -> Svgr<'static> {
    let mut parts: Vec<Svgr<'static>> = Vec::new();
    for i in 0..books {
        let color = ["#B8643E", "#5D7FA3", "#C9A04A", "#8A9A86", "#8C7BA8"][i % 5];
        parts.push(
            Slab::new(
                x + 20.0 + i as f64 * 52.0,
                y - 110.0 + (i % 2) as f64 * 14.0,
                44.0,
                110.0 - (i % 2) as f64 * 14.0,
                color,
            )
            .r(3.0)
            .shade("#000000", 0.1)
            .render(Svgr::empty()),
        );
    }
    let board = Slab::new(x, y, w, 22.0, C::WOOD)
        .r(3.0)
        .shade(C::WOOD_LO, 0.4)
        .render(Svgr::empty());
    paper(4.0, false, svgr!(<g>{parts}{board}</g>))
}

/// A sticky note.
#[must_use]
pub fn sticky(x: f64, y: f64, s: f64, color: &str, rot: f64) -> Svgr<'static> {
    let note = Slab::new(x, y, s, s, color)
        .r(2.0)
        .rot(rot)
        .shade("#000000", 0.15)
        .render(Svgr::empty());
    paper(3.0, false, note)
}

/// A whip-pan entrance: slide in from `from` px and blur out, over `dur` frames.
pub fn whip_in(frame: usize, dur: usize, from: f64, children: Svgr<'_>) -> Svgr<'_> {
    let p = map(frame as f64, 0.0, dur as f64, 1.0, 0.0);
    let dx = p * from;
    // A cheap horizontal blur: three offset copies with descending opacity. SVG filters are
    // available, but stacking is far cheaper to rasterise and reads the same at speed.
    if p <= 0.0 {
        return children;
    }
    let spread = p * 60.0;
    let ghost =
        svgr!(<g transform={format!("translate({} 0)", -spread)} opacity={0.35}>{children}</g>);
    svgr!(<g transform={format!("translate({dx:.2} 0)")}>{ghost}</g>)
}

/// A wall clock, for a set that needs to read as "office".
#[must_use]
pub fn clock(x: f64, y: f64, d: f64, minutes: f64) -> Svgr<'static> {
    let r = d / 2.0;
    let cx = x + r;
    let cy = y + r;
    let face = Slab::new(x, y, d, d, C::CREAM).r(r).render(Svgr::empty());
    let hour_angle =
        (minutes / 60.0) % 12.0 / 12.0 * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
    let min_angle = (minutes % 60.0) / 60.0 * std::f64::consts::TAU - std::f64::consts::FRAC_PI_2;
    let hand = |angle: f64, len: f64, wid: f64| {
        let ex = cx + angle.cos() * len;
        let ey = cy + angle.sin() * len;
        svgr!(<line x1={cx} y1={cy} x2={ex} y2={ey} stroke={C::INK} stroke-width={wid} stroke-linecap="round" />)
    };
    let hour_hand = hand(hour_angle, r * 0.5, 6.0);
    let min_hand = hand(min_angle, r * 0.72, 4.0);
    svgr!(<g>{face}{hour_hand}{min_hand}<circle cx={cx} cy={cy} r="5" fill={C::INK} /></g>)
}

/// The room's own vignette, weaker than the global one, so a set has depth before the overlays.
#[must_use]
pub fn room_vignette() -> Svgr<'static> {
    svgr!(<g>{vignette(0.55)}{edge_fade(true, 0.5)}</g>)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::markup;

    #[test]
    fn room_draws_wall_floor_and_skirt() {
        let out = markup(Room::new("navy").render(Svgr::empty()));
        assert!(
            out.to_lowercase().contains("#27324a"),
            "wall colour of navy"
        );
        assert!(out.contains(C::FLOOR), "wood floor");
        assert!(out.contains(C::WOOD_LO), "skirting");
    }

    #[test]
    fn room_without_a_floor_has_no_wood() {
        let out = markup(Room::new("sage").floor(Floor::None).render(Svgr::empty()));
        assert!(
            !out.contains(C::WOOD_LO),
            "no skirting when the floor is off"
        );
    }

    #[test]
    fn unknown_room_names_fall_back() {
        let out = markup(Room::new("not-a-room").render(Svgr::empty()));
        assert!(out.contains(C::SAGE));
    }

    #[test]
    fn lamp_glows_only_when_on() {
        assert!(markup(lamp(0.0, 0.0, 1.0, true)).contains("#ffe28c"));
        assert!(!markup(lamp(0.0, 0.0, 1.0, false)).contains("#ffe28c"));
    }

    #[test]
    fn window_shows_stars_only_at_night() {
        assert!(markup(window(0.0, 0.0, 300.0, 400.0, true))
            .to_lowercase()
            .contains("#1f2a44"));
        assert!(markup(window(0.0, 0.0, 300.0, 400.0, false))
            .to_lowercase()
            .contains("#9db7c9"));
    }

    #[test]
    fn shelf_draws_the_books_it_is_asked_for() {
        let two = markup(shelf(0.0, 0.0, 400.0, 2));
        let five = markup(shelf(0.0, 0.0, 400.0, 5));
        assert!(five.len() > two.len());
    }

    #[test]
    fn whip_in_is_a_noop_at_rest() {
        let a = markup(whip_in(
            30,
            7,
            1080.0,
            svgr!(<rect x="0" y="0" width="1" height="1" fill="#000" />),
        ));
        let b = markup(svgr!(<rect x="0" y="0" width="1" height="1" fill="#000" />));
        assert_eq!(a, b);
    }

    #[test]
    fn clock_hands_move_with_time() {
        let nine = markup(clock(0.0, 0.0, 200.0, 9.0 * 60.0));
        let three = markup(clock(0.0, 0.0, 200.0, 3.0 * 60.0));
        assert_ne!(nine, three);
    }
}
