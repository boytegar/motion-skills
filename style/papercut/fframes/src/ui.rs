//! Interface props: monitors, laptops, terminals, buttons, pointers, cards, phones.
//!
//! Port of `../remotion/kit/ui/ui.tsx`. The Remotion version lays these out with CSS; SVG has no
//! flow layout, so text is placed by hand and measured with the same rough glyph metrics as
//! [`crate::kit::pill`] (Inter 800 at ~0.62em per glyph).

use fframes::{svgr, Svgr};

use crate::anim::{in_out, map, out_cubic, pop_spring, random};
use crate::kit::{paper, Slab};
use crate::tokens::{palette as C, FONT};

/// Rough width of a string in `FONT.label` at `size`, matching the Remotion layouts.
#[must_use]
pub fn text_w(text: &str, size: f64) -> f64 {
    text.chars().count().max(1) as f64 * size * 0.62
}

/// A monitor on a stand, with the screen contents drawn inside the bezel.
#[must_use]
pub fn monitor<'a>(
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    stand: bool,
    screen: &str,
    children: Svgr<'a>,
) -> Svgr<'a> {
    let stand_parts = if stand {
        let neck = Slab::new(x + w / 2.0 - 34.0, y + h - 10.0, 68.0, 110.0, "#3A3A40")
            .r(4.0)
            .shade("#28282C", 0.3)
            .render(Svgr::empty());
        let foot = Slab::new(x + w / 2.0 - 150.0, y + h + 92.0, 300.0, 30.0, "#3A3A40")
            .r(10.0)
            .shade("#28282C", 0.4)
            .render(Svgr::empty());
        svgr!(<g>{neck}{foot}</g>)
    } else {
        Svgr::empty()
    };
    let bezel = Slab::new(x, y, w, h, "#2B2B30")
        .r(22.0)
        .shade("#1E1E22", 0.06)
        .render(Svgr::empty());
    let glint = svgr!(<rect
        x={x + 22.0 + (w - 44.0) * 0.62}
        y={y + 22.0 - 40.0}
        width={46.0 * (w - 44.0) / 400.0}
        height={(h - 58.0) * 1.4}
        fill="rgba(255,255,255,0.06)"
        transform={format!("rotate(24 {} {})", x + 22.0 + (w - 44.0) * 0.62 + 23.0, y + 22.0)}
    />);
    let glass = Slab::new(x + 22.0, y + 22.0, w - 44.0, h - 58.0, screen)
        .r(10.0)
        .grain(0.3)
        .render(svgr!(<g>{children}{glint}</g>));
    let chin = svgr!(<circle cx={x + w / 2.0} cy={y + h - 18.0} r="5" fill="#55555C" />);
    paper(6.0, false, svgr!(<g>{stand_parts}{bezel}{glass}{chin}</g>))
}

/// A laptop: lid, screen, base.
#[must_use]
pub fn laptop(x: f64, y: f64, w: f64, h: f64, children: Svgr<'_>) -> Svgr<'_> {
    let lid = Slab::new(x, y, w, h, "#2B2B30")
        .r(18.0)
        .render(Svgr::empty());
    let screen = Slab::new(x + 18.0, y + 18.0, w - 36.0, h - 36.0, C::SCREEN)
        .r(8.0)
        .grain(0.3)
        .render(children);
    let base = Slab::new(x - 50.0, y + h - 4.0, w + 100.0, 34.0, C::METAL)
        .r(4.0)
        .shade(C::METAL_LO, 0.4)
        .render(Svgr::empty());
    let notch = Slab::new(x + w / 2.0 - 70.0, y + h - 4.0, 140.0, 12.0, C::METAL_LO)
        .r(8.0)
        .render(Svgr::empty());
    paper(6.0, false, svgr!(<g>{lid}{screen}{base}{notch}</g>))
}

/// The traffic-light bar and title of a desktop app window.
#[must_use]
pub fn app_window<'a>(w: f64, h: f64, title: &str, bg: &str, children: Svgr<'a>) -> Svgr<'a> {
    let bg = bg.to_string();
    let dots = ["#E5534B", "#E6A23C", "#4CAF6A"];
    let mut lights: Vec<Svgr<'static>> = Vec::new();
    for (i, c) in dots.iter().enumerate() {
        lights.push(svgr!(<circle cx={20.0 + 18.0 + i as f64 * 30.0} cy={26.0} r="9" fill={*c} />));
    }
    let bar = svgr!(<rect x="0" y="0" width={w} height="52" fill={C::SCREEN_HI} />);
    let label = svgr!(<text
        x={w / 2.0}
        y={36.0}
        text-anchor="middle"
        font-family={FONT.label}
        font-weight="800"
        font-size="24"
        fill={C::SCREEN_DIM}
    >{title.to_string()}</text>);
    let body = svgr!(<g transform="translate(0 52)">{children}</g>);
    svgr!(<g>
        <rect x="0" y="0" width={w} height={h} fill={bg} />
        {bar}{lights}{label}{body}
    </g>)
}

/// A terminal line: `(frame, text, kind)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TermKind {
    You,
    Out,
    Ok,
    Dim,
    Warn,
}

/// One scripted terminal line.
#[derive(Debug, Clone)]
pub struct TermLine {
    /// Frame the line appears, scene-local.
    pub at: usize,
    pub text: String,
    pub kind: TermKind,
}

impl TermLine {
    pub fn new(at: usize, text: &str, kind: TermKind) -> Self {
        TermLine {
            at,
            text: text.to_string(),
            kind,
        }
    }
}

/// A terminal window: a title bar and lines that appear at their frames.
#[must_use]
pub fn terminal(
    w: f64,
    h: f64,
    lines: &[TermLine],
    title: &str,
    size: f64,
    frame: usize,
) -> Svgr<'static> {
    let mut rows: Vec<Svgr<'static>> = Vec::new();
    let mut y = 24.0;
    for l in lines.iter().filter(|l| frame >= l.at) {
        let (glyph, color) = match l.kind {
            TermKind::You => (">", C::HERO),
            TermKind::Ok => ("✓", "#7BD88F"),
            TermKind::Warn => ("•", C::AMBER),
            TermKind::Dim => ("•", C::SCREEN_DIM),
            TermKind::Out => ("•", C::SCREEN_TEXT),
        };
        rows.push(svgr!(<text x="28" y={y + size * 0.8} font-family={FONT.mono} font-weight="700" font-size={size} fill={color}>{glyph.to_string()}</text>));
        let body_color = match l.kind {
            TermKind::You | TermKind::Out => C::SCREEN_TEXT,
            TermKind::Ok => "#7BD88F",
            TermKind::Warn => C::AMBER,
            TermKind::Dim => C::SCREEN_DIM,
        };
        // Typed lines reveal a character at a time; the rest land whole.
        let shown = match l.kind {
            TermKind::You => typewriter(&l.text, frame, l.at, 1.5),
            _ => l.text.clone(),
        };
        rows.push(svgr!(<text x={28.0 + size * 1.2} y={y + size * 0.8} font-family={FONT.mono} font-weight="700" font-size={size} fill={body_color}>{shown}</text>));
        y += size * 1.35;
    }
    let body = svgr!(<g>{rows}</g>);
    app_window(w, h, title, C::SCREEN, body)
}

/// The prefix of `text` revealed by frame `f`, one character every `fpc` frames from `at`.
#[must_use]
pub fn typewriter(text: &str, f: usize, at: usize, fpc: f64) -> String {
    if f < at {
        return String::new();
    }
    let n = (((f - at) as f64) / fpc.max(0.01)).floor().max(0.0) as usize;
    text.chars().take(n).collect()
}

/// A button, with an optional press dip.
#[must_use]
pub fn button(
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    text: &str,
    color: &str,
    size: f64,
    frame: usize,
    press_at: Option<usize>,
) -> Svgr<'static> {
    let color = color.to_string();
    let down = press_at.is_some_and(|p| frame >= p && frame < p + 5);
    let dy = if down { 4.0 } else { 0.0 };
    let label = text.to_string();
    let cx = x + w / 2.0;
    let cy = y + h / 2.0 + size * 0.36;
    let shadow = if down {
        String::new()
    } else {
        "filter:drop-shadow(0 5px 0 rgba(0,0,0,0.35))".to_string()
    };
    svgr!(<g style={shadow}>
        <rect x={x} y={y + dy} width={w} height={h} rx="16" fill={color} />
        <text x={cx} y={cy + dy} text-anchor="middle" font-family={FONT.label} font-weight="800" font-size={size} fill="#fff">{label}</text>
    </g>)
}

/// A progress bar, filled to `t` (0..1).
#[must_use]
pub fn progress(x: f64, y: f64, w: f64, t: f64, color: &str) -> Svgr<'static> {
    let color = color.to_string();
    let h = 26.0;
    let filled = w * crate::anim::clamp01(t);
    svgr!(<g>
        <rect x={x} y={y} width={w} height={h} rx={h / 2.0} fill="#3A3348" />
        <rect x={x} y={y} width={filled} height={h} rx={h / 2.0} fill={color} />
    </g>)
}

/// A count-up number, rounded and zero-free, as `CountUp` renders it.
///
/// `map` already eases (it goes through `lerp` + `clamp01`), so this must not
/// apply `out_cubic` a second time — that compresses the ramp and the counter
/// finishes early.
#[must_use]
pub fn count_at(frame: usize, at: usize, dur: usize, to: f64, from: f64) -> f64 {
    map(frame as f64, at as f64, (at + dur) as f64, from, to)
}

/// Format a count the way the kit does: rounded, thousands separated.
#[must_use]
pub fn fmt(n: f64) -> String {
    let n = n.round() as i64;
    let neg = n < 0;
    let digits = n.abs().to_string();
    let mut out = String::new();
    for (i, c) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    if neg {
        format!("-{out}")
    } else {
        out
    }
}

/// A repository card: owner/repo, a description, and a star button that counts up.
#[derive(Debug, Clone)]
pub struct RepoCard {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub owner: String,
    pub repo: String,
    pub desc: String,
    pub stars_at: usize,
    pub stars: f64,
    pub stars_from: f64,
    pub forks: Option<f64>,
    pub star_dur: usize,
    pub rot: f64,
}

impl RepoCard {
    pub fn new(
        x: f64,
        y: f64,
        owner: &str,
        repo: &str,
        desc: &str,
        stars_at: usize,
        stars: f64,
    ) -> Self {
        RepoCard {
            x,
            y,
            w: 820.0,
            owner: owner.to_string(),
            repo: repo.to_string(),
            desc: desc.to_string(),
            stars_at,
            stars,
            stars_from: 0.0,
            forks: None,
            star_dur: 36,
            rot: -1.5,
        }
    }

    pub fn forks(mut self, n: f64) -> Self {
        self.forks = Some(n);
        self
    }

    /// Draw at `frame`.
    #[must_use]
    pub fn render(&self, frame: usize) -> Svgr<'static> {
        let h = 430.0;
        let pressed = frame >= self.stars_at && frame < self.stars_at + 4;
        // GitHub's `owner / repo` heading is one text run; splitting it across
        // two `<text>` nodes drops the space and the tests/eyeball both notice.
        let head = format!("{} / {}", self.owner, self.repo);
        let name_size = 40.0;
        let desc_size = 34.0;
        let star = count_at(
            frame,
            self.stars_at,
            self.star_dur,
            self.stars,
            self.stars_from,
        );
        let btn_fill = if pressed { "#E3DED3" } else { "#EAE5DB" };
        let btn_scale = if pressed { 0.95 } else { 1.0 };
        let star_fill = if frame >= self.stars_at {
            "#E3B341"
        } else {
            "none"
        };
        let bx = self.x + 44.0;
        let by = self.y + 40.0;

        let mut parts: Vec<Svgr<'static>> = Vec::new();
        parts.push(svgr!(<text
            x={bx + 56.0}
            y={by + name_size * 0.85}
            font-family={FONT.label}
            font-weight="800"
            font-size={name_size}
            fill="#2B5CC4"
        >{head}</text>));
        // Two lines of description, wrapped on word boundaries.
        let max_w = self.w - 88.0;
        for (i, line) in wrap(&self.desc, max_w, desc_size)
            .into_iter()
            .take(2)
            .enumerate()
        {
            parts.push(svgr!(<text
                x={bx}
                y={by + 26.0 + name_size + desc_size * (0.9 + i as f64 * 1.25)}
                font-family={FONT.label}
                font-weight="800"
                font-size={desc_size}
                fill={C::INK_SOFT}
            >{line}</text>));
        }
        let row_y = by + 34.0 + name_size + desc_size * 2.9;
        let btn_w = 200.0;
        let btn_h = 70.0;
        let cx = bx + btn_w / 2.0;
        let cy = row_y + btn_h / 2.0;
        parts.push(svgr!(<g
            transform={format!("translate({cx} {cy}) scale({btn_scale:.3}) translate({} {})", -cx, -cy)}
        >
            <rect x={bx} y={row_y} width={btn_w} height={btn_h} rx="12" fill={btn_fill} stroke="#CFC8BA" stroke-width="3" />
            <path d="M12 2l2.9 6.6 7.1.6-5.4 4.7 1.6 7L12 17.3 5.8 21l1.6-7L2 9.2l7.1-.6z"
                  transform={format!("translate({} {}) scale(1.15)", bx + 22.0, row_y + 18.0)}
                  fill={star_fill} stroke="#8A8274" stroke-width="2" />
            <text x={bx + 78.0} y={row_y + 46.0} font-family={FONT.label} font-weight="800" font-size="34" fill={C::INK}>{"Star"}</text>
            <text x={bx + 148.0} y={row_y + 46.0} font-family={FONT.label} font-weight="800" font-size="30" fill={C::INK}>{fmt(star)}</text>
        </g>));
        if let Some(fork_to) = self.forks {
            let fv = count_at(frame, self.stars_at + 8, self.star_dur, fork_to, 0.0);
            parts.push(svgr!(<text
                x={bx + btn_w + 22.0}
                y={row_y + 46.0}
                font-family={FONT.label}
                font-weight="800"
                font-size="30"
                fill="#6E6A64"
            >{format!("⑂ {} forks", fmt(fv))}</text>));
        }

        let card = Slab::new(self.x, self.y, self.w, h, "#F1EDE4")
            .r(14.0)
            .rot(self.rot)
            .shade(C::CREAM_LO, 0.03)
            .render(svgr!(<g>{parts}</g>));
        paper(7.0, false, card)
    }
}

/// Greedy word wrap to `max_w` at `size`, for the few places SVG needs it.
#[must_use]
pub fn wrap(text: &str, max_w: f64, size: f64) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        let candidate = if cur.is_empty() {
            word.to_string()
        } else {
            format!("{cur} {word}")
        };
        if text_w(&candidate, size) > max_w && !cur.is_empty() {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        } else {
            cur = candidate;
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// A pointer keyframe: where the cursor is, and when.
#[derive(Debug, Clone, Copy)]
pub struct PointerKey {
    pub f: usize,
    pub x: f64,
    pub y: f64,
    pub click: bool,
}

impl PointerKey {
    pub const fn new(f: usize, x: f64, y: f64) -> Self {
        PointerKey {
            f,
            x,
            y,
            click: false,
        }
    }

    pub const fn click(mut self) -> Self {
        self.click = true;
        self
    }
}

/// A cursor that eases between keyframes and shows a click ripple.
#[must_use]
pub fn pointer(frame: usize, keys: &[PointerKey], size: f64) -> Svgr<'static> {
    let Some(first) = keys.first() else {
        return Svgr::empty();
    };
    if frame < first.f {
        return Svgr::empty();
    }
    let at = keys.iter().rev().find(|k| frame >= k.f).unwrap_or(first);
    let next = keys.iter().find(|k| k.f > frame);
    let (x, y) = match next {
        Some(n) if n.f > at.f => {
            let t = in_out(map(frame as f64, at.f as f64, n.f as f64, 0.0, 1.0));
            (
                crate::anim::lerp(at.x, n.x, t),
                crate::anim::lerp(at.y, n.y, t),
            )
        }
        _ => (at.x, at.y),
    };
    let click = keys
        .iter()
        .find(|k| k.click && frame >= k.f && frame < k.f + 14);
    let dt = click.map_or(99, |c| frame - c.f);
    let ripple = match click {
        Some(c) => {
            let grow = dt as f64 * 8.0;
            svgr!(<circle
                cx={c.x - 10.0 - dt as f64 * 4.0 + (20.0 + grow) / 2.0}
                cy={c.y - 10.0 - dt as f64 * 4.0 + (20.0 + grow) / 2.0}
                r={(20.0 + grow) / 2.0}
                fill="none"
                stroke={C::SPARK_YELLOW}
                stroke-width="5"
                opacity={1.0 - dt as f64 / 14.0}
            />)
        }
        None => Svgr::empty(),
    };
    let scale = if dt < 4 { 0.85 } else { 1.0 };
    let cursor = svgr!(<g
        transform={format!("translate({x} {y}) scale({scale})")}
        style="filter:drop-shadow(2px 4px 4px rgba(0,0,0,0.4))"
    >
        <path d="M3 2l7.5 19 2.6-7.4L21 11z" fill="#F7F3EC" stroke={C::INK} stroke-width="1.6" stroke-linejoin="round" />
    </g>);
    let _ = size;
    svgr!(<g>{ripple}{cursor}</g>)
}

/// A tool integration card: a coloured glyph tile and a name.
#[must_use]
pub fn tool_card(
    x: f64,
    y: f64,
    name: &str,
    color: &str,
    glyph: Option<&str>,
    w: f64,
    rot: f64,
) -> Svgr<'static> {
    let h = 150.0;
    let label = glyph.unwrap_or(&name[..1.min(name.len())]).to_string();
    let size = ((w - 140.0) / (name.chars().count().max(1) as f64 * 0.62)).min(40.0);
    let tile = Slab::new(22.0, 33.0, 84.0, 84.0, color)
        .r(18.0)
        .render(svgr!(<text
        x={22.0 + 42.0}
        y={33.0 + 42.0 + 44.0 * 0.36}
        text-anchor="middle"
        font-family={FONT.banner}
        font-weight="400"
        font-size="44"
        fill="#fff"
    >{label}</text>));
    let name = name.to_string();
    let body = svgr!(<g>
        {tile}
        <text x={124.0} y={h / 2.0 + size * 0.36} font-family={FONT.label} font-weight="800" font-size={size} fill={C::INK}>{name}</text>
    </g>);
    let slab = Slab::new(x, y, w, h, C::CREAM_HI)
        .r(14.0)
        .rot(rot)
        .shade(C::CREAM_LO, 0.05)
        .render(body);
    paper(6.0, false, slab)
}

/// A phone with a comment box: types `keyword`, then posts it.
#[allow(clippy::too_many_arguments)]
pub fn phone_comment(
    frame: usize,
    x: f64,
    y: f64,
    keyword: &str,
    type_at: usize,
    fpc: f64,
    post_at: Option<usize>,
    scale: f64,
) -> Svgr<'static> {
    let (w, h) = (520.0_f64, 1000.0_f64);
    let typed = typewriter(keyword, frame, type_at, fpc);
    let posted = post_at.is_some_and(|p| frame >= p);
    let pop = post_at.map_or(0.0, |p| pop_spring(frame as f64 - p as f64, 30.0));
    let mut parts: Vec<Svgr<'static>> = Vec::new();

    parts.push(svgr!(<text x={w / 2.0} y="56" text-anchor="middle" font-family={FONT.label} font-weight="800" font-size="30" fill={C::INK}>{"Comments"}</text>));
    // Three placeholder comment rows.
    for i in 0..3 {
        let cy = 130.0 + i as f64 * 92.0;
        let color = ["#C9A04A", "#5D7FA3", "#8C7BA8"][i];
        parts.push(svgr!(<circle cx="60" cy={cy} r="26" fill={color} />));
        let w1 = 180.0 + random(&format!("c{i}")) * 140.0;
        let w2 = 120.0 + random(&format!("d{i}")) * 120.0;
        parts.push(
            svgr!(<rect x="100" y={cy - 16.0} width={w1} height="14" rx="7" fill="#B9B0A2" />),
        );
        parts.push(
            svgr!(<rect x="100" y={cy + 8.0} width={w2} height="12" rx="6" fill="#D3CABD" />),
        );
    }
    if posted {
        let cx = 100.0;
        let cy = 420.0;
        parts.push(svgr!(<g transform={format!("translate({cx} {cy}) scale({pop:.3}) translate({} {})", -cx, -cy)}>
            <circle cx="60" cy="20" r="26" fill={C::CAP} />
            <text x="100" y="34" font-family={FONT.label} font-weight="800" font-size="34" fill={C::INK}>{keyword.to_string()}</text>
        </g>));
    }
    // Input row.
    let placeholder = if posted {
        "Add a comment…".to_string()
    } else if typed.is_empty() {
        "Add a comment…".to_string()
    } else {
        typed.clone()
    };
    let caret = if !typed.is_empty() && !posted && (frame / 8).is_multiple_of(2) {
        "|"
    } else {
        ""
    };
    let input_size = if !typed.is_empty() && !posted {
        38.0
    } else {
        30.0
    };
    let input_color = if !typed.is_empty() && !posted {
        C::INK
    } else {
        "#9B9184"
    };
    parts.push(svgr!(<rect x="24" y="510" width={w - 48.0} height="76" rx="38" fill="#E3DDD2" stroke="#CFC8BA" stroke-width="3" />));
    parts.push(svgr!(<text x="54" y={510.0 + 40.0 + input_size * 0.3} font-family={FONT.label} font-weight="800" font-size={input_size} fill={input_color}>{format!("{placeholder}{caret}")}</text>));
    let post_ready = typed.chars().count() == keyword.chars().count() && !posted;
    let post_bg = if post_ready { C::BRAND } else { "#B9B0A2" };
    parts
        .push(svgr!(<rect x={w - 138.0} y="519" width="120" height="58" rx="26" fill={post_bg} />));
    parts.push(svgr!(<text x={w - 78.0} y="557" text-anchor="middle" font-family={FONT.label} font-weight="800" font-size="26" fill="#fff">{"Post"}</text>));

    // Keyboard.
    parts.push(svgr!(<rect x="0" y="620" width={w} height={h - 620.0} fill="#D6CFC3" />));
    let rows = ["QWERTYUIOP", "ASDFGHJKL", "ZXCVBNM"];
    for (r, row) in rows.iter().enumerate() {
        let key_w = 40.0;
        let gap = 6.0;
        let total = row.chars().count() as f64 * key_w
            + gap * (row.chars().count().saturating_sub(1)) as f64;
        let start = (w - total) / 2.0;
        let top = 620.0 + if r == 0 { 18.0 } else { 18.0 + r as f64 * 68.0 };
        for (k, key) in row.chars().enumerate() {
            let lit = typed.ends_with(key) && !posted && frame >= type_at;
            let bg = if lit { C::BRAND } else { "#F4F0E8" };
            let fg = if lit { "#fff" } else { C::INK };
            let kx = start + k as f64 * (key_w + gap);
            parts.push(svgr!(<rect x={kx} y={top} width={key_w} height="58" rx="8" fill={bg} />));
            parts.push(svgr!(<text x={kx + key_w / 2.0} y={top + 38.0} text-anchor="middle" font-family={FONT.label} font-weight="800" font-size="28" fill={fg}>{key.to_string()}</text>));
        }
    }

    let bezel = Slab::new(0.0, 0.0, w, h, "#26232C")
        .r(64.0)
        .render(Svgr::empty());
    let screen = Slab::new(18.0, 18.0, w - 36.0, h - 36.0, "#F1EDE4")
        .r(50.0)
        .grain(0.3)
        .render(svgr!(<g>{parts}</g>));
    let phone = paper(8.0, false, svgr!(<g>{bezel}{screen}</g>));
    svgr!(<g transform={format!("translate({x} {y}) scale({scale:.3})")}>{phone}</g>)
}

/// A button-shaped call to action, used by the CTA scene template.
#[must_use]
pub fn cta_button(frame: usize, x: f64, y: f64, w: f64, text: &str, at: usize) -> Svgr<'static> {
    let t = out_cubic(map(frame as f64, at as f64, (at + 8) as f64, 0.0, 1.0));
    let h = 110.0;
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;
    let label = text.to_string();
    svgr!(<g transform={format!("translate({cx} {cy}) scale({t:.3}) translate({} {})", -cx, -cy)}>
        <rect x={x} y={y} width={w} height={h} rx="24" fill={C::BRAND} style="filter:drop-shadow(0 6px 0 rgba(0,0,0,0.3))" />
        <text x={cx} y={cy + 18.0} text-anchor="middle" font-family={FONT.label} font-weight="800" font-size="44" fill="#fff">{label}</text>
    </g>)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{is_blank, markup};

    #[test]
    fn typewriter_reveals_one_char_per_fpc() {
        assert_eq!(typewriter("hello", 0, 10, 2.0), "");
        assert_eq!(typewriter("hello", 10, 10, 2.0), "");
        assert_eq!(typewriter("hello", 14, 10, 2.0), "he");
        assert_eq!(typewriter("hello", 30, 10, 2.0), "hello");
    }

    #[test]
    fn fmt_groups_thousands() {
        assert_eq!(fmt(0.0), "0");
        assert_eq!(fmt(999.4), "999");
        assert_eq!(fmt(1000.0), "1,000");
        assert_eq!(fmt(1_234_567.0), "1,234,567");
        assert_eq!(fmt(-1200.0), "-1,200");
    }

    #[test]
    fn wrap_keeps_lines_under_the_width() {
        let lines = wrap(
            "a fairly long description that needs to wrap over lines",
            400.0,
            34.0,
        );
        assert!(lines.len() > 1);
        for l in &lines {
            assert!(text_w(l, 34.0) <= 400.0 + 1.0, "line too wide: {l}");
        }
    }

    #[test]
    fn count_at_clamps_both_ends() {
        assert_eq!(count_at(0, 10, 20, 100.0, 0.0), 0.0);
        assert!((count_at(40, 10, 20, 100.0, 0.0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn terminal_only_shows_lines_that_have_landed() {
        let lines = vec![
            TermLine::new(0, "first", TermKind::Out),
            TermLine::new(50, "second", TermKind::Out),
        ];
        let early = markup(terminal(800.0, 600.0, &lines, "t", 30.0, 10));
        assert!(early.contains("first"));
        assert!(!early.contains("second"));
        let late = markup(terminal(800.0, 600.0, &lines, "t", 30.0, 60));
        assert!(late.contains("second"));
    }

    #[test]
    fn pointer_is_hidden_before_its_first_key() {
        let keys = vec![PointerKey::new(20, 100.0, 100.0)];
        assert!(is_blank(pointer(0, &keys, 64.0)));
        assert!(!is_blank(pointer(30, &keys, 64.0)));
    }

    #[test]
    fn pointer_eases_between_keys() {
        let keys = vec![
            PointerKey::new(0, 0.0, 0.0),
            PointerKey::new(20, 200.0, 0.0),
        ];
        let mid = markup(pointer(10, &keys, 64.0));
        assert!(mid.contains("translate("));
        assert_ne!(
            markup(pointer(0, &keys, 64.0)),
            markup(pointer(20, &keys, 64.0))
        );
    }

    #[test]
    fn repo_card_counts_stars_up() {
        let card = RepoCard::new(0.0, 0.0, "acme", "widget", "a thing", 30, 1000.0);
        let early = markup(card.render(0));
        let late = markup(card.render(90));
        assert!(early.contains("acme / widget"));
        assert!(late.contains("1,000"), "got {late}");
        assert!(!early.contains("1,000"));
    }

    #[test]
    fn phone_types_then_posts() {
        let early = markup(phone_comment(0, 0.0, 0.0, "PAPER", 30, 5.0, Some(90), 1.0));
        assert!(early.contains("Add a comment"));
        let typed = markup(phone_comment(60, 0.0, 0.0, "PAPER", 30, 5.0, Some(90), 1.0));
        assert!(typed.contains("PAPER"));
    }
}
