//! Deterministic helpers: hash-based randomness (the kit's `random(seed)`), easing, springs and
//! clamping. No state, so every frame renders the same value on every thread.

/// A stable pseudo-random number in 0..1 for a string seed, like Remotion's `random(seed)`.
pub fn random(seed: &str) -> f64 {
    // FNV-1a — cheap and stable across platforms and Rust versions.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in seed.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    // splitmix64 finaliser so neighbouring seeds land far apart
    h ^= h >> 30;
    h = h.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    h ^= h >> 27;
    h = h.wrapping_mul(0x94d0_49bb_1331_11eb);
    h ^= h >> 31;
    (h >> 11) as f64 / (u128::from(1u64) << 53) as f64
}

/// A seeded counter, for call sites that want `Random::next()` instead of free functions.
/// Each call mixes the base seed with the running index, so it stays deterministic per frame.
#[derive(Clone, Debug)]
pub struct Random {
    seed: String,
    n: u64,
}

impl Random {
    pub fn new(seed: &str) -> Self {
        Random {
            seed: seed.to_string(),
            n: 0,
        }
    }

    /// The next value in 0..1.
    pub fn next(&mut self) -> f64 {
        let v = random(&format!("{}{}", self.seed, self.n));
        self.n += 1;
        v
    }

    /// The next value in `lo..hi`.
    pub fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + self.next() * (hi - lo)
    }

    /// A value from `0..n` without advancing the counter, so parallel call sites stay stable.
    pub fn at(&self, i: u64) -> f64 {
        random(&format!("{}{}", self.seed, i))
    }
}

/// A stable integer in `lo..hi`.
pub fn rand_int(seed: &str, lo: i64, hi: i64) -> i64 {
    lo + (random(seed) * (hi - lo) as f64).floor() as i64
}

pub fn clamp01(x: f64) -> f64 {
    x.clamp(0.0, 1.0)
}

/// Progress of `frame` across `a..b`, clamped to 0..1 and stepped at `hold` frames (a stop-motion hold).
pub fn step(frame: usize, a: usize, b: usize, hold: usize) -> f64 {
    if b <= a {
        return f64::from((frame >= b) as u8);
    }
    let f = if hold > 1 {
        (frame / hold) * hold
    } else {
        frame
    };
    clamp01((f as f64 - a as f64) / (b - a) as f64)
}

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Map `frame` from `a..b` to `from..to`, clamped both ends (Remotion's `interpolate` with clamp).
pub fn map(frame: f64, a: f64, b: f64, from: f64, to: f64) -> f64 {
    if b <= a {
        return if frame >= b { to } else { from };
    }
    lerp(from, to, clamp01((frame - a) / (b - a)))
}

pub fn smooth(t: f64) -> f64 {
    let t = clamp01(t);
    t * t * (3.0 - 2.0 * t)
}

pub fn in_out(t: f64) -> f64 {
    let t = clamp01(t);
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
    }
}

pub fn out_cubic(t: f64) -> f64 {
    1.0 - (1.0 - clamp01(t)).powi(3)
}

pub fn out_back(t: f64, s: f64) -> f64 {
    let t = clamp01(t) - 1.0;
    t * t * ((s + 1.0) * t + s) + 1.0
}

/// Ease in with an overshoot, the mirror of [`out_back`]. `s` is the overshoot amount; 1.6 is a
/// noticeable fling, 1.0 a gentle one (Remotion's `Easing.in(Easing.back(s))`).
pub fn in_back(t: f64, s: f64) -> f64 {
    let t = clamp01(t);
    (s + 1.0) * t * t * t - s * t * t
}

pub fn out_elastic(t: f64) -> f64 {
    let t = clamp01(t);
    if t == 0.0 || t == 1.0 {
        return t;
    }
    2f64.powf(-10.0 * t) * ((t * 10.0 - 0.75) * (2.0 * std::f64::consts::PI / 3.0)).sin() + 1.0
}

/// A damped spring settling on 1 from 0 (the kit's `spring({frame, fps, config})`).
/// `frame` is frames since the spring started; negative frames give 0.
pub fn spring(frame: f64, fps: f64, stiffness: f64, damping: f64, mass: f64) -> f64 {
    if frame <= 0.0 {
        return 0.0;
    }
    let t = frame / fps;
    let w0 = (stiffness / mass).sqrt();
    let zeta = damping / (2.0 * (stiffness * mass).sqrt());
    if zeta < 1.0 {
        let wd = w0 * (1.0 - zeta * zeta).sqrt();
        1.0 - (-zeta * w0 * t).exp() * ((wd * t).cos() + (zeta * w0 / wd) * (wd * t).sin())
    } else {
        1.0 - (-w0 * t).exp() * (1.0 + w0 * t)
    }
}

/// The kit's PopIn spring: damping 12, stiffness 180, mass 0.6.
pub fn pop_spring(frame: f64, fps: f64) -> f64 {
    spring(frame, fps, 180.0, 12.0, 0.6)
}

pub fn pop_at(frame: usize, at: usize, fps: usize) -> f64 {
    if frame < at {
        0.0
    } else {
        pop_spring((frame - at) as f64, fps as f64)
    }
}

/// A timeline of keyframes in seconds, sampled per frame.
pub struct Animation {
    keys: Vec<(f32, f64)>,
    easing: Ease,
}

#[derive(Clone, Copy)]
pub enum Ease {
    Linear,
    In,
    Out,
    InOut,
    Spring,
}

impl Animation {
    pub fn new(keys: &[(f32, f64)], easing: Ease) -> Self {
        let mut k = keys.to_vec();
        k.sort_by(|a, b| a.0.total_cmp(&b.0));
        Animation { keys: k, easing }
    }

    /// The value at `seconds`.
    pub fn at(&self, seconds: f64) -> f64 {
        let Some(first) = self.keys.first() else {
            return 0.0;
        };
        if seconds <= f64::from(first.0) {
            return first.1;
        }
        for w in self.keys.windows(2) {
            let (a, av) = w[0];
            let (b, bv) = w[1];
            if seconds <= f64::from(b) {
                let span = f64::from(b - a);
                let t = if span <= 0.0 {
                    1.0
                } else {
                    (seconds - f64::from(a)) / span
                };
                let e = match self.easing {
                    Ease::Linear => clamp01(t),
                    Ease::In => clamp01(t).powi(2),
                    Ease::Out => out_cubic(t),
                    Ease::InOut => in_out(t),
                    Ease::Spring => spring(t * 30.0, 30.0, 120.0, 18.0, 1.0),
                };
                return lerp(av, bv, e);
            }
        }
        self.keys.last().map_or(0.0, |k| k.1)
    }
}

/// Idle bob: a sine wave in frames (the kit's `bob`).
pub fn bob(frame: usize, period: f64, amp: f64, phase: f64) -> f64 {
    ((frame as f64 + phase) / period * std::f64::consts::TAU).sin() * amp
}

/// A single hop that peaks `h` pixels `dur` frames after `at`.
pub fn hop(frame: usize, at: usize, dur: usize, h: f64) -> f64 {
    if frame < at || dur == 0 || frame > at + dur {
        return 0.0;
    }
    let u = (frame - at) as f64 / dur as f64;
    (u * std::f64::consts::PI).sin() * h
}

/// An arm reach that springs out at `at` and (if `back` is given) retracts at `back`, both 0..1.
pub fn reach(frame: usize, at: usize, back: Option<usize>) -> f64 {
    let out = out_back(map(frame as f64, at as f64, at as f64 + 6.0, 0.0, 1.0), 1.6);
    match back {
        None => out,
        Some(b) => out * map(frame as f64, b as f64, b as f64 + 5.0, 1.0, 0.0),
    }
}

/// Colour maths on `#rrggbb` strings: mix two colours by `t`, and darken/lighten by a factor.
pub fn mix(a: &str, b: &str, t: f64) -> String {
    let (ar, ag, ab) = rgb(a);
    let (br, bg, bb) = rgb(b);
    let f = |x: f64, y: f64| (lerp(x, y, clamp01(t))).round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", f(ar, br), f(ag, bg), f(ab, bb))
}

/// Multiply a colour (light < 1 darkens, > 1 lightens).
pub fn shade(color: &str, k: f64) -> String {
    let (r, g, b) = rgb(color);
    let f = |x: f64| (x * k).round().clamp(0.0, 255.0) as u8;
    format!("#{:02x}{:02x}{:02x}", f(r), f(g), f(b))
}

/// The colour with alpha `a` (0..1) as `#rrggbbaa`.
pub fn alpha(color: &str, a: f64) -> String {
    let (r, g, b) = rgb(color);
    let r = r as u8;
    let g = g as u8;
    let b = b as u8;
    let a = (clamp01(a) * 255.0).round() as u8;
    format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
}

fn rgb(color: &str) -> (f64, f64, f64) {
    let h = color.trim_start_matches('#');
    let n = u32::from_str_radix(&h[..h.len().min(6)], 16).unwrap_or(0);
    (
        f64::from((n >> 16) & 0xff),
        f64::from((n >> 8) & 0xff),
        f64::from(n & 0xff),
    )
}
