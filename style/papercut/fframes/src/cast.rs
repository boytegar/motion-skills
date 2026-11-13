//! The one base creature for the whole cast: a boxy paper critter with eyes only and stubby limbs.
//! Cast members differ by colour, accessory and name pill. `(x, y)` is the ground point under the
//! body.

use fframes::svgr;

use crate::anim::bob;
use crate::kit::{disc, paper, Slab};
use crate::tokens::palette as C;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Expr {
    Idle,
    Sleep,
    Shock,
    Happy,
    Worried,
    Blink,
    Squint,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Acc {
    None,
    PostCap,
    Monocle,
    Headphones,
    Visor,
    Cap,
    HardHat,
    Beret,
    Glasses,
    Headset,
    Bowtie,
}

impl Acc {
    fn is_hat(self) -> bool {
        matches!(
            self,
            Acc::PostCap | Acc::Headphones | Acc::Cap | Acc::HardHat | Acc::Beret
        )
    }
}

/// A critter's parameters. Build it with [`CritterOpts::new`] and the builder methods.
pub struct CritterOpts {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub color: String,
    pub shade: String,
    pub expr: Expr,
    pub acc: Acc,
    pub tag: Option<String>,
    pub tag_color: String,
    /// Right arm reach 0..1.
    pub arm_r: f64,
    pub arm_r_angle: f64,
    pub arm_l: f64,
    pub arm_l_angle: f64,
    pub lean: f64,
    pub lift: f64,
    pub face: f64,
    pub tape: bool,
    pub phase: f64,
    pub idle: bool,
    pub acc_color: Option<String>,
    pub acc_shade: Option<String>,
}

impl CritterOpts {
    pub fn new(x: f64, y: f64) -> Self {
        CritterOpts {
            x,
            y,
            size: 240.0,
            color: C::HERO.to_string(),
            shade: C::HERO_LO.to_string(),
            expr: Expr::Idle,
            acc: Acc::None,
            tag: None,
            tag_color: C::PURPLE.to_string(),
            arm_r: 0.0,
            arm_r_angle: 0.0,
            arm_l: 0.0,
            arm_l_angle: 0.0,
            lean: 0.0,
            lift: 0.0,
            face: 1.0,
            tape: false,
            phase: 0.0,
            idle: true,
            acc_color: None,
            acc_shade: None,
        }
    }

    pub fn size(mut self, s: f64) -> Self {
        self.size = s;
        self
    }

    pub fn color(mut self, c: &str, shade: &str) -> Self {
        self.color = c.to_string();
        self.shade = shade.to_string();
        self
    }

    pub fn expr(mut self, e: Expr) -> Self {
        self.expr = e;
        self
    }

    pub fn acc(mut self, a: Acc) -> Self {
        self.acc = a;
        self
    }

    pub fn acc_color(mut self, c: &str, shade: &str) -> Self {
        self.acc_color = Some(c.to_string());
        self.acc_shade = Some(shade.to_string());
        self
    }

    pub fn tag(mut self, t: &str) -> Self {
        self.tag = Some(t.to_string());
        self
    }

    pub fn tag_color(mut self, c: &str) -> Self {
        self.tag_color = c.to_string();
        self
    }

    /// Right arm reach 0..1 and angle in degrees (negative = up).
    pub fn arm_right(mut self, reach: f64, angle: f64) -> Self {
        self.arm_r = reach;
        self.arm_r_angle = angle;
        self
    }

    pub fn arm_left(mut self, reach: f64, angle: f64) -> Self {
        self.arm_l = reach;
        self.arm_l_angle = angle;
        self
    }

    pub fn lean(mut self, deg: f64) -> Self {
        self.lean = deg;
        self
    }

    pub fn lift(mut self, px: f64) -> Self {
        self.lift = px;
        self
    }

    /// Face left when negative.
    pub fn face(mut self, f: f64) -> Self {
        self.face = if f < 0.0 { -1.0 } else { 1.0 };
        self
    }

    pub fn taped(mut self) -> Self {
        self.tape = true;
        self
    }

    pub fn phase(mut self, p: f64) -> Self {
        self.phase = p;
        self
    }

    pub fn still(mut self) -> Self {
        self.idle = false;
        self
    }

    /// Render at `frame`.
    pub fn render(&self, frame: usize, fps: usize) -> fframes::Svgr<'static> {
        let u = self.size / 10.0;
        let b = if self.idle {
            bob(frame, 36.0, 3.0, self.phase)
        } else {
            0.0
        };
        let b_lag = if self.idle {
            bob(frame.saturating_sub(2), 36.0, 3.0, self.phase)
        } else {
            0.0
        };
        let blink = self.expr == Expr::Idle
            && ((frame as f64) + self.phase * 7.0).round() as usize % 83 < 3;
        let _ = fps;
        let body_w = 10.0 * u;
        let body_h = 7.0 * u;
        let leg_h = 2.2 * u;
        let top = -leg_h - body_h;
        let e = if self.expr == Expr::Blink || blink {
            Expr::Blink
        } else {
            self.expr
        };
        let eye_w = u * if e == Expr::Shock { 1.3 } else { 1.0 };
        let eye_h = u * if e == Expr::Shock { 2.1 } else { 1.6 };
        let eye_color = C::INK;

        let mut inner: Vec<fframes::Svgr<'static>> = Vec::new();
        // legs
        for (i, lx) in [1.2_f64, 3.3, 5.7, 7.8].iter().enumerate() {
            inner.push(
                Slab::new(lx * u, body_h - 0.3 * u, u, leg_h + 0.3 * u, &self.color)
                    .r(2.0)
                    .shade(&self.shade, 0.35)
                    .render(fframes::Svgr::empty()),
            );
            let _ = i;
        }
        // left arm
        inner.push(self.arm(-1.0, self.arm_l, self.arm_l_angle, body_w, u));
        // body
        inner.push(
            Slab::new(0.0, 0.0, body_w, body_h, &self.color)
                .r(4.0)
                .shade(&self.shade, 0.16)
                .render(fframes::Svgr::empty()),
        );
        // right arm
        inner.push(self.arm(1.0, self.arm_r, self.arm_r_angle, body_w, u));
        // eyes
        for (i, (ex, ey)) in [(3.0_f64, 2.0_f64), (6.0, 2.0)].iter().enumerate() {
            let eyes = match e {
                Expr::Sleep | Expr::Blink | Expr::Happy => {
                    let top = ey + if e == Expr::Happy { 0.3 * u } else { 0.8 * u };
                    svgr!(<rect x={ex * u - 0.15 * u} y={top} width={1.3 * u} height={0.28 * u} rx={if e == Expr::Happy { 6.0 } else { 2.0 }} fill={eye_color} />)
                }
                _ => {
                    let dx = if e == Expr::Worried {
                        (if i == 1 { -0.2 } else { 0.2 }) * u
                    } else {
                        0.0
                    };
                    let top = ey + if e == Expr::Squint { 0.5 * u } else { 0.0 };
                    let h = if e == Expr::Squint { 0.6 * u } else { eye_h };
                    svgr!(<rect x={ex * u + dx + (u - eye_w) / 2.0} y={top} width={eye_w} height={h} rx="2" fill={eye_color} />)
                }
            };
            inner.push(eyes);
        }
        if e == Expr::Shock {
            inner.push(svgr!(<circle cx={4.4 * u} cy={4.5 * u} r={0.55 * u} fill={eye_color} />));
        }
        if e == Expr::Happy {
            inner.push(svgr!(<rect x={4.0 * u} y={4.4 * u} width={2.0 * u} height={0.3 * u} rx="3" fill={eye_color} />));
        }
        if self.tape {
            inner.push(
                Slab::new(2.6 * u, 4.1 * u, 4.8 * u, 1.3 * u, C::TAPE)
                    .r(2.0)
                    .rot(-6.0)
                    .opacity(0.92)
                    .render(fframes::Svgr::empty()),
            );
        }
        inner.push(self.accessory(u));

        let body = Slab::new(-body_w / 2.0, top, body_w, body_h + leg_h, "transparent")
            .r(0.0)
            .edge(crate::kit::Edge::None)
            .lift(5.0);
        // The transparent slab is only the shadow carrier; draw the pieces directly.
        let _ = body;
        let body_svg = paper(5.0, false, svgr!(<g>{inner}</g>));

        let tag_svg = self.tag.as_ref().map_or_else(fframes::Svgr::empty, |tag| {
            let ty = top - if self.acc.is_hat() { 3.3 } else { 1.6 } * u + (b_lag - b);
            crate::kit::pill(
                0.0,
                ty,
                tag,
                &self.tag_color,
                C::CREAM,
                (0.13 * self.size).max(24.0),
                0.0,
                true,
            )
        });

        let lean_t = if self.lean.abs() > f64::EPSILON {
            format!("rotate({} 0 0)", self.lean)
        } else {
            String::new()
        };
        svgr!(<g transform={format!("translate({} {}) scale({} 1)", self.x, self.y, self.face)}>
            <g transform={format!("translate(0 {})", -self.lift + b)}>
                <g transform={lean_t}>
                    {body_svg}
                    {tag_svg}
                </g>
            </g>
        </g>)
    }

    fn arm(
        &self,
        side: f64,
        reach: f64,
        angle: f64,
        body_w: f64,
        u: f64,
    ) -> fframes::Svgr<'static> {
        let len = 2.3 * u + reach * 5.0 * u;
        let ax = if side > 0.0 {
            body_w - 0.4 * u
        } else {
            0.4 * u
        };
        let rot = if side > 0.0 { angle } else { -angle };
        let x = if side > 0.0 { 0.0 } else { -len };
        svgr!(<g transform={format!("translate({ax} {}) rotate({rot})", 3.4 * u)}>
            {Slab::new(x, -0.7 * u, len, 1.4 * u, &self.color).r(3.0).shade(&self.shade, 0.3).render(fframes::Svgr::empty())}
        </g>)
    }

    fn accessory(&self, u: f64) -> fframes::Svgr<'static> {
        let color = self.acc_color.as_deref();
        let shade = self.acc_shade.as_deref();
        let mut parts: Vec<fframes::Svgr<'static>> = Vec::new();
        match self.acc {
            Acc::PostCap => {
                parts.push(
                    Slab::new(1.2 * u, -1.9 * u, 7.6 * u, 2.1 * u, C::CAP)
                        .r(20.0)
                        .shade(C::CAP_LO, 0.3)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(5.4 * u, -0.1 * u, 4.6 * u, 0.7 * u, C::CAP_LO)
                        .r(3.0)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(4.3 * u, -1.5 * u, 1.3 * u, 1.1 * u, C::BRASS)
                        .r(3.0)
                        .shade(C::BRASS_LO, 0.3)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::Monocle => {
                parts.push(svgr!(<circle cx={6.5 * u} cy={2.5 * u} r={1.0 * u} fill="none" stroke={C::BRASS} stroke-width={0.35 * u} />));
                parts.push(svgr!(<line x1={6.4 * u} y1={3.5 * u} x2={6.6 * u} y2={5.9 * u} stroke={C::BRASS_LO} stroke-width={0.2 * u} />));
                parts.push(
                    Slab::new(3.4 * u, 6.3 * u, 3.2 * u, 1.3 * u, C::NOGO)
                        .r(6.0)
                        .shade(C::NOGO_DIM, 0.3)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::Headphones => {
                parts.push(svgr!(<path
                    d={format!("M {} {} A {} {} 0 0 1 {} {}", 0.6 * u, 2.4 * u, 4.4 * u, 4.0 * u, 9.4 * u, 2.4 * u)}
                    fill="none"
                    stroke={C::BOARD}
                    stroke-width={0.6 * u}
                />));
                parts.push(
                    Slab::new(-0.4 * u, 1.4 * u, 1.5 * u, 2.4 * u, C::BOARD)
                        .r(6.0)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(8.9 * u, 1.4 * u, 1.5 * u, 2.4 * u, C::BOARD)
                        .r(6.0)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::Visor => {
                parts.push(
                    Slab::new(0.8 * u, -0.8 * u, 9.6 * u, 1.2 * u, color.unwrap_or(C::GO))
                        .r(4.0)
                        .shade(shade.unwrap_or(C::GO_DIM), 0.4)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::Cap => {
                parts.push(
                    Slab::new(1.4 * u, -2.1 * u, 7.2 * u, 2.4 * u, color.unwrap_or(C::CAP))
                        .r(20.0)
                        .shade(shade.unwrap_or(C::CAP_LO), 0.28)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(6.2 * u, 0.0, 4.4 * u, 0.75 * u, shade.unwrap_or(C::CAP_LO))
                        .r(3.0)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    disc(5.0 * u, -2.0 * u, 0.9 * u, shade.unwrap_or(C::CAP_LO), None)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::HardHat => {
                parts.push(
                    Slab::new(
                        1.0 * u,
                        -2.3 * u,
                        8.0 * u,
                        2.6 * u,
                        color.unwrap_or(C::AMBER),
                    )
                    .r(18.0)
                    .shade(shade.unwrap_or(C::AMBER_DIM), 0.22)
                    .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(
                        0.2 * u,
                        0.0,
                        9.6 * u,
                        0.7 * u,
                        shade.unwrap_or(C::AMBER_DIM),
                    )
                    .r(3.0)
                    .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(
                        4.6 * u,
                        -2.2 * u,
                        0.8 * u,
                        2.2 * u,
                        shade.unwrap_or(C::AMBER_DIM),
                    )
                    .r(2.0)
                    .render(fframes::Svgr::empty()),
                );
            }
            Acc::Beret => {
                parts.push(
                    Slab::new(1.6 * u, -1.5 * u, 7.6 * u, 1.9 * u, color.unwrap_or(C::INK))
                        .r(14.0)
                        .rot(-6.0)
                        .shade(shade.unwrap_or("#000000"), 0.3)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(4.8 * u, -2.1 * u, 0.6 * u, 0.9 * u, color.unwrap_or(C::INK))
                        .r(3.0)
                        .render(fframes::Svgr::empty()),
                );
            }
            Acc::Glasses => {
                for gx in [2.4_f64, 5.6] {
                    parts.push(svgr!(<circle cx={(gx + 1.05) * u} cy={2.65 * u} r={1.05 * u} fill="none" stroke={C::INK} stroke-width={0.32 * u} />));
                }
                parts.push(svgr!(<line x1={4.5 * u} y1={2.6 * u} x2={5.7 * u} y2={2.6 * u} stroke={C::INK} stroke-width={0.3 * u} />));
            }
            Acc::Headset => {
                parts.push(svgr!(<path
                    d={format!("M {} {} A {} {} 0 0 1 {} {}", 0.4 * u, 2.2 * u, 4.6 * u, 3.4 * u, 9.6 * u, 2.2 * u)}
                    fill="none"
                    stroke={C::BOARD}
                    stroke-width={0.45 * u}
                />));
                parts.push(
                    Slab::new(
                        -0.3 * u,
                        1.6 * u,
                        1.3 * u,
                        2.0 * u,
                        color.unwrap_or(C::BOARD),
                    )
                    .r(6.0)
                    .render(fframes::Svgr::empty()),
                );
                parts.push(svgr!(<line x1={0.3 * u} y1={3.4 * u} x2={3.5 * u} y2={4.3 * u} stroke={C::BOARD} stroke-width={0.3 * u} />));
                parts.push(
                    disc(3.5 * u, 4.5 * u, 0.8 * u, C::BOARD, None).render(fframes::Svgr::empty()),
                );
            }
            Acc::Bowtie => {
                parts.push(
                    Slab::new(3.5 * u, 6.0 * u, 1.5 * u, 1.4 * u, color.unwrap_or(C::NOGO))
                        .r(3.0)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    Slab::new(5.0 * u, 6.0 * u, 1.5 * u, 1.4 * u, color.unwrap_or(C::NOGO))
                        .r(3.0)
                        .render(fframes::Svgr::empty()),
                );
                parts.push(
                    disc(
                        5.0 * u,
                        6.7 * u,
                        0.8 * u,
                        shade.unwrap_or(C::NOGO_DIM),
                        None,
                    )
                    .render(fframes::Svgr::empty()),
                );
            }
            Acc::None => {}
        }
        svgr!(<g>{parts}</g>)
    }
}

/// The shorthand most scenes use: `Critter::at(x, y).size(300).expr(Expr::Happy).render(frame)`.
pub struct Critter;

impl Critter {
    pub fn at(x: f64, y: f64) -> CritterOpts {
        CritterOpts::new(x, y)
    }

    /// The viewer's proxy: the hero colour with a blue cap on.
    pub fn you(x: f64, y: f64) -> CritterOpts {
        CritterOpts::new(x, y).acc(Acc::Cap)
    }
}
