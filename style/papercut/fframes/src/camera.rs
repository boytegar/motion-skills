//! Camera moves for paper sets: a fake camera moving through a paper diorama. Each move returns a
//! `transform` attribute string plus, for depth moves, a per-layer offset helper.
//!
//! The moves transform the whole frame, so the banner zone (y < 420) and the caption band
//! (y 1420..1580) move too: keep key props and faces out of both.
//!
//! The kit's hardware is emulated with SVG: a `Depth` layer is drawn at its normal screen position
//! and the camera adds a parallax offset in proportion to its `z`.

use crate::anim::{in_out, lerp, map, out_back, out_cubic, smooth, step};

/// A camera is a transform plus a depth function. Call [`Camera::focus`] to wrap content, or read
/// [`Camera::offset`] to place depth layers by hand.
pub struct Camera {
    pub scale: f64,
    pub tx: f64,
    pub ty: f64,
    pub rot: f64,
    /// Pivot the transform is applied around.
    pub origin: (f64, f64),
    /// Tint/blur a scene may want at this moment.
    pub blur: f64,
    /// Parallax strength for depth layers (0 = flat).
    pub parallax: f64,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            scale: 1.0,
            tx: 0.0,
            ty: 0.0,
            rot: 0.0,
            origin: (540.0, 960.0),
            blur: 0.0,
            parallax: 0.0,
        }
    }

    pub fn origin(mut self, x: f64, y: f64) -> Self {
        self.origin = (x, y);
        self
    }

    pub fn push(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn pan(mut self, x: f64, y: f64) -> Self {
        self.tx = x;
        self.ty = y;
        self
    }

    pub fn tilt(mut self, deg: f64) -> Self {
        self.rot = deg;
        self
    }

    pub fn blur(mut self, px: f64) -> Self {
        self.blur = px;
        self
    }

    pub fn parallax(mut self, p: f64) -> Self {
        self.parallax = p;
        self
    }

    /// The transform attribute for a layer at depth `z` (0 = backdrop, ~0.5 = props, 1 = foreground).
    pub fn transform(&self, z: f64) -> String {
        // Layers further back move less; the pivot is the camera origin.
        let grab = 1.0 + (1.0 - z) * self.parallax * 0.25;
        let s = 1.0 + (self.scale - 1.0) * grab;
        let tx = self.tx * lerp(0.6, 1.4, z);
        let ty = self.ty * lerp(0.6, 1.4, z);
        format!(
            "translate({} {}) scale({s}) rotate({} {} {}) translate({} {})",
            self.origin.0 + tx,
            self.origin.1 + ty,
            self.rot,
            self.origin.0,
            self.origin.1,
            -self.origin.0,
            -self.origin.1
        )
    }

    /// Wrap `children` in this camera at depth 0.5.
    pub fn focus<'a>(&self, children: fframes::Svgr<'a>) -> fframes::Svgr<'a> {
        let t = self.transform(0.5);
        fframes::svgr!(<g transform={t}>{children}</g>)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// A plain slow push: the baseline move, 1.0 to `to` over `dur`.
pub fn camera_push(
    frame: usize,
    dur: usize,
    to: f64,
    drift: (f64, f64),
    accent: Option<usize>,
) -> Camera {
    let base = map(frame as f64, 0.0, dur as f64, 1.0, to);
    let acc = accent.map_or(0.0, |a| {
        map(frame as f64, a as f64, a as f64 + 30.0, 0.0, 0.1)
    });
    let p = crate::anim::clamp01(frame as f64 / dur.max(1) as f64);
    Camera::new()
        .push(base + acc)
        .pan(drift.0 * p, drift.1 * p)
        .origin(540.0, 864.0)
}

/// Parallax orbit: the camera arcs `yaw` degrees around the focal plane while dollying to `push`.
pub fn orbit(
    frame: usize,
    dur: usize,
    yaw: f64,
    push: f64,
    drift: (f64, f64),
    origin: (f64, f64),
) -> Camera {
    let t = in_out(crate::anim::clamp01(frame as f64 / dur.max(1) as f64));
    let swing = (t - 0.5) * 2.0;
    Camera::new()
        .origin(origin.0, origin.1)
        .push(lerp(1.0, push, smooth(t)))
        .pan(swing * yaw * 7.0 + drift.0 * t, drift.1 * t)
        .tilt(swing * yaw * 0.12)
        .parallax(1.0)
}

/// Punch-in to a focal point: 1 to `to` in 4 frames with a 3-frame shake and a little roll, then
/// settles at `settle`. Use `at` = the frame the thing lands.
pub fn snap_zoom(frame: usize, at: usize, x: f64, y: f64, to: f64, settle: f64) -> Camera {
    if frame < at {
        return Camera::new().origin(x, y);
    }
    let f = frame - at;
    let punch = map(f as f64, 0.0, 4.0, 1.0, to);
    let back = map(f as f64, 4.0, 12.0, to, settle);
    let s = if f < 4 { punch } else { back };
    let shake = if f < 5 {
        (1.0 - f as f64 / 5.0) * 18.0
    } else {
        0.0
    };
    let wob = ((f as f64) * 2.1).sin() * shake;
    let wob2 = ((f as f64) * 2.7).cos() * shake;
    let rot = if f < 5 {
        (1.0 - f as f64 / 5.0) * 1.4
    } else {
        0.0
    };
    Camera::new()
        .origin(x, y)
        .push(s)
        .pan(wob, wob2)
        .tilt(rot)
        .blur(if f < 3 { 3.0 } else { 0.0 })
}

/// Pull-back reveal: starts `from`x tight on the detail at `(x, y)`, eases out to the whole set.
pub fn pull_back(frame: usize, at: usize, dur: usize, x: f64, y: f64, from: f64) -> Camera {
    let t = out_cubic(step(frame, at, at + dur, 1));
    Camera::new()
        .origin(x, y)
        .push(lerp(from, 1.0, t))
        .blur(if frame < at + 3 { 2.5 } else { 0.0 })
}

/// Dolly zoom: the camera pushes in while the set widens, the vertigo move.
pub fn dolly_zoom(
    frame: usize,
    dur: usize,
    at: usize,
    strength: f64,
    origin: (f64, f64),
) -> Camera {
    let t = in_out(step(frame, at, at + dur, 1));
    Camera::new()
        .origin(origin.0, origin.1)
        .push(lerp(1.0, 1.0 + 0.18 * strength, t))
        .parallax(-1.6 * strength)
}

/// Crab track: step sideways to each stop and hold, with parallax.
pub fn crab(frame: usize, stops: &[f64], hold: usize, move_dur: usize, at: usize) -> Camera {
    let f = frame.saturating_sub(at);
    let mut pan = stops.first().copied().unwrap_or(0.0);
    let mut i = 0;
    let mut cursor = hold;
    while i + 1 < stops.len() {
        if f < cursor {
            pan = stops[i];
            break;
        }
        if f < cursor + move_dur {
            let t = in_out((f - cursor) as f64 / move_dur as f64);
            pan = lerp(stops[i], stops[i + 1], t);
            break;
        }
        cursor += move_dur + hold;
        i += 1;
        pan = stops[i];
    }
    Camera::new().pan(pan, 0.0).parallax(0.8)
}

/// Crane: rise or drop to a higher viewpoint over `dur`.
pub fn crane(
    frame: usize,
    dur: usize,
    at: usize,
    height: f64,
    rise: bool,
    origin: (f64, f64),
) -> Camera {
    let t = out_cubic(step(frame, at, at + dur, 1));
    let dy = if rise {
        lerp(0.0, -height, t)
    } else {
        lerp(-height, 0.0, t)
    };
    Camera::new()
        .origin(origin.0, origin.1)
        .pan(0.0, dy)
        .push(lerp(1.0, 1.06, t))
        .parallax(0.4)
}

/// Handheld: a slow organic drift, for the struggling beat.
pub fn handheld(frame: usize, amp: f64) -> Camera {
    let f = frame as f64;
    Camera::new()
        .pan((f * 0.31).sin() * amp, (f * 0.23).cos() * amp * 0.7)
        .tilt((f * 0.11).sin() * 0.4)
}

/// A freeze frame with an optional label: holds the child at `at`, desaturates and darkens, then
/// slaps on a torn paper label with a stamp bounce.
/// Returns the camera (a punch in on the label) and a flag the caller uses to draw the label.
pub fn freeze_frame(frame: usize, at: usize, x: f64, y: f64) -> (Camera, bool) {
    if frame < at {
        return (Camera::new(), false);
    }
    let f = frame - at;
    let bounce = out_back(map(f as f64, 0.0, 6.0, 0.0, 1.0), 2.4);
    let cam = Camera::new().origin(x, y).push(lerp(1.0, 1.04, bounce));
    (cam, true)
}
