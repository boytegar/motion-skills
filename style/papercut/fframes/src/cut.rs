//! Scene transitions. A cut is a function of the transition's progress `t` (0 at the outgoing
//! frame, 1 when the incoming scene is whole) returning two SVG snippets: a mask/key over the
//! outgoing scene and a transform over the incoming one.
//!
//! In fframes, scenes are separate; the transition is drawn by the incoming scene over its own
//! first frames (an entrance veil) and by the outgoing scene over its last frames (an exit veil).
//! Both scenes call [`CutSeries::entrance`] / [`CutSeries::exit`] with the same kind, so the two
//! halves line up.

use fframes::svgr;

use crate::anim::{out_back, out_cubic, random};
use crate::tokens::palette as C;

/// The kit's cut kinds.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CutKind {
    Hard,
    Whip,
    ZoomThrough,
    PageTurn,
    TearAway,
    TapePull,
    StampCut,
    InfiniteZoom,
    PopUpBook,
    Keyhole,
    BurnAway,
}

impl CutKind {
    pub fn parse(s: &str) -> Self {
        match s {
            "whip" => Self::Whip,
            "zoomThrough" => Self::ZoomThrough,
            "pageTurn" => Self::PageTurn,
            "tearAway" => Self::TearAway,
            "tapePull" => Self::TapePull,
            "stampCut" => Self::StampCut,
            "infiniteZoom" => Self::InfiniteZoom,
            "popUpBook" => Self::PopUpBook,
            "keyhole" => Self::Keyhole,
            "burnAway" => Self::BurnAway,
            _ => Self::Hard,
        }
    }

    /// Frames the cut runs for (the kit's CUT_DEFAULTS).
    pub fn frames(self) -> usize {
        match self {
            Self::Hard => 0,
            Self::Whip => 8,
            Self::ZoomThrough => 10,
            Self::PageTurn => 14,
            Self::TearAway => 12,
            Self::TapePull => 12,
            Self::StampCut => 8,
            Self::InfiniteZoom => 16,
            Self::PopUpBook => 14,
            Self::Keyhole => 12,
            Self::BurnAway => 16,
        }
    }
}

/// The cut into each scene, from episode.json `"cuts": {"S04Name": {"kind": "tearAway"}}`.
pub struct CutSpec {
    pub kind: CutKind,
}

impl CutSpec {
    pub fn new(kind: &str) -> Self {
        CutSpec {
            kind: CutKind::parse(kind),
        }
    }
}

/// A scene's cut-in, with the kind and the frame count.
pub struct CutSeries {
    pub kind: CutKind,
    pub frames: usize,
}

impl CutSeries {
    pub fn new(kind: CutKind) -> Self {
        CutSeries {
            kind,
            frames: kind.frames(),
        }
    }

    /// The entrance veil for the incoming scene, over its first `frames` frames.
    /// Returns (overlay, incoming transform).
    pub fn entrance(&self, frame: usize, seed: &str) -> (fframes::Svgr<'static>, String) {
        if frame >= self.frames || self.frames == 0 {
            return (fframes::Svgr::empty(), String::new());
        }
        let t = out_cubic(frame as f64 / self.frames as f64);
        match self.kind {
            CutKind::Hard => (fframes::Svgr::empty(), String::new()),
            CutKind::Whip => {
                let dx = (1.0 - t) * 1080.0;
                let veil = svgr!(<rect x="0" y="0" width="1080" height="1920" fill={C::INK} opacity={0.25 * (1.0 - t)} />);
                (veil, format!("translate({dx} 0)"))
            }
            CutKind::ZoomThrough => {
                let s = 0.6 + 0.4 * t;
                let veil = svgr!(<circle cx="540" cy="960" r={700.0 * (1.0 - t) + 900.0} fill={C::INK} opacity={0.3 * (1.0 - t)} />);
                (
                    veil,
                    format!("translate(540 960) scale({s}) translate(-540 -960)"),
                )
            }
            CutKind::PageTurn => {
                let w = 1080.0 * (1.0 - t);
                let veil = svgr!(<g>
                    <rect x="0" y="0" width={w} height="1920" fill={C::CREAM_LO} />
                    <rect x={w - 18.0} y="0" width="18" height="1920" fill={C::CREAM_HI} opacity="0.7" />
                </g>);
                (veil, String::new())
            }
            CutKind::TearAway => {
                let y = 1920.0 * t;
                let mut path = String::from("M 0 1920 L 0 1920 ");
                let steps = 14;
                for i in 0..=steps {
                    let x = 1080.0 * i as f64 / steps as f64;
                    let yy = y - random(&format!("{seed}t{i}")) * 40.0;
                    path.push_str(&format!("L {x} {yy} "));
                }
                path.push_str("L 1080 1920 Z");
                let veil = svgr!(<g>
                    <path d={path} fill={C::CREAM} />
                    <rect x="0" y={y - 40.0} width="1080" height="40" fill={C::CREAM_HI} opacity="0.8" />
                </g>);
                (veil, String::new())
            }
            CutKind::TapePull => {
                let x = 1080.0 * (1.0 - t);
                let veil = svgr!(<g>
                    <rect x={x} y="0" width={1080.0 - x} height="1920" fill={C::INK} opacity={0.45 * (1.0 - t)} />
                    <rect x={x - 40.0} y="0" width="40" height="1920" fill={C::TAPE} opacity="0.85" transform={format!("rotate({} {} 960)", -2.0 * (1.0 - t), x)} />
                </g>);
                (veil, format!("translate({} 0)", -120.0 * (1.0 - t)))
            }
            CutKind::StampCut => {
                let s = out_back(t, 2.0);
                let veil = svgr!(<rect x="0" y="0" width="1080" height="1920" fill={C::INK} opacity={0.6 * (1.0 - t)} />);
                (
                    veil,
                    format!("translate(540 960) scale({s}) translate(-540 -960)"),
                )
            }
            CutKind::InfiniteZoom => {
                let s = 1.0 + 0.7 * (1.0 - t);
                let mut frames = Vec::new();
                for i in 0..4 {
                    let k = 1.0 + i as f64 * 0.9;
                    let w = 300.0 / k;
                    let op = 0.8 - i as f64 * 0.18;
                    frames.push(svgr!(<rect
                        x={540.0 - w / 2.0}
                        y={960.0 - w * 0.9}
                        width={w}
                        height={w * 1.8}
                        rx={w * 0.12}
                        fill="none"
                        stroke={C::CREAM}
                        stroke-width={6.0 / k}
                        opacity={(op * (1.0 - t)).max(0.0)}
                    />));
                }
                (
                    svgr!(<g>{frames}</g>),
                    format!("translate(540 960) scale({s}) translate(-540 -960)"),
                )
            }
            CutKind::PopUpBook => {
                let s = out_back(t, 1.4);
                let veil = svgr!(<rect x="0" y="0" width="1080" height="1920" fill={C::WOOD} opacity={0.55 * (1.0 - t)} />);
                (
                    veil,
                    format!("translate(540 1560) scale({s}) translate(-540 -1560)"),
                )
            }
            CutKind::Keyhole => {
                let r = 200.0 + 1400.0 * t;
                let veil = svgr!(<mask id="pc-keyhole-in">
                    <rect x="0" y="0" width="1080" height="1920" fill="#fff" />
                    <circle cx="540" cy="900" r={r} fill="#000" />
                </mask>);
                let dark = svgr!(<rect x="0" y="0" width="1080" height="1920" fill="#000" mask="url(#pc-keyhole-in)" />);
                (svgr!(<g>{veil}{dark}</g>), String::new())
            }
            CutKind::BurnAway => {
                let y = 1920.0 * t;
                let mut path = String::from("M 0 1920 ");
                let steps = 16;
                for i in 0..=steps {
                    let x = 1080.0 * i as f64 / steps as f64;
                    let yy = y + (random(&format!("{seed}b{i}")) - 0.5) * 90.0;
                    path.push_str(&format!("L {x} {yy} "));
                }
                path.push_str("L 1080 1920 Z");
                let inner = svgr!(<g>
                    <path d={path.clone()} fill={C::INK} />
                    <path d={path} fill={C::AMBER} opacity="0.85" transform="translate(0 14)" />
                </g>);
                (svgr!(<g opacity={1.0 - t}>{inner}</g>), String::new())
            }
        }
    }

    /// The exit veil for the outgoing scene, over its last `frames` frames.
    pub fn exit(&self, frame: usize, dur: usize, seed: &str) -> (fframes::Svgr<'static>, String) {
        if self.frames == 0 || frame + self.frames < dur {
            return (fframes::Svgr::empty(), String::new());
        }
        // replay the entrance backwards over the tail: same shape, mirrored progress
        let into = dur.saturating_sub(frame);
        let (veil, tf) = self.entrance(
            self.frames
                .saturating_sub(into)
                .min(self.frames.saturating_sub(1)),
            seed,
        );
        (veil, tf)
    }

    /// The SFX name for this cut, for the cue sheet (episode.json "cutSfx").
    pub fn sfx(&self) -> &'static str {
        match self.kind {
            CutKind::Hard => "",
            CutKind::Whip => "whip",
            CutKind::ZoomThrough => "zoom",
            CutKind::PageTurn => "page-flip",
            CutKind::TearAway => "rip",
            CutKind::TapePull => "tape-peel",
            CutKind::StampCut => "stamp",
            CutKind::InfiniteZoom => "zoom",
            CutKind::PopUpBook => "whoosh",
            CutKind::Keyhole => "whoosh",
            CutKind::BurnAway => "burn-away",
        }
    }
}

/// A curved paper divider landing in the middle of a cut scene, for the two-scene case.
pub fn transition_badge(kind: CutKind, x: f64, y: f64) -> fframes::Svgr<'static> {
    let label = match kind {
        CutKind::Whip => "WHIP",
        CutKind::ZoomThrough => "ZOOM",
        CutKind::PageTurn => "TURN",
        CutKind::TearAway => "TEAR",
        CutKind::TapePull => "TAPE",
        CutKind::StampCut => "STAMP",
        CutKind::InfiniteZoom => "DROSTE",
        CutKind::PopUpBook => "POP-UP",
        CutKind::Keyhole => "POV",
        CutKind::BurnAway => "BURN",
        CutKind::Hard => "",
    };
    if label.is_empty() {
        return fframes::Svgr::empty();
    }
    crate::kit::pill(x, y, label, C::INK, C::CREAM, 26.0, -4.0, true)
}
