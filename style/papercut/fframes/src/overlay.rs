//! Global overlays: the taped banner, the word-by-word caption tags, the grain and the vignette.
//!
//! Port of `../remotion/kit/paper/overlays.tsx` and the root layers of `PapercutVideo.tsx`.
//! Everything here draws in frame coordinates; the caller places them in a [`crate::camera::Camera`].

use fframes::{svgr, Svgr};

use crate::anim::{pop_spring, random};
use crate::kit::{boil, grain_overlay, paper, pill, tape_corners, Edge, Slab};
use crate::tokens::{palette as C, FONT, H, W};

/// A caption word as `plan.json` and `captions.json` describe it.
#[derive(Debug, Clone, Default)]
pub struct Caption {
    pub text: String,
    /// Milliseconds from the start of the reel.
    pub start_ms: f64,
    pub end_ms: f64,
}

impl Caption {
    pub fn new(text: &str, start_ms: f64, end_ms: f64) -> Self {
        Caption {
            text: text.to_string(),
            start_ms,
            end_ms,
        }
    }
}

/// A page of caption tags: the words shown together, and when the page is up.
#[derive(Debug, Clone, Default)]
pub struct Page {
    pub words: Vec<Caption>,
    pub start_ms: f64,
    pub end_ms: f64,
}

/// Group captions into pages of at most `max` words, breaking early on punctuation or a pause.
///
/// This is the same rule as the Remotion `pageWords`: three words, or the end of a clause, or a
/// gap of more than 250 ms.
#[must_use]
pub fn page_words(caps: &[Caption], max: usize) -> Vec<Page> {
    let max = max.max(1);
    let mut pages = Vec::new();
    let mut cur: Vec<Caption> = Vec::new();
    for (i, w) in caps.iter().enumerate() {
        cur.push(w.clone());
        let next = caps.get(i + 1);
        let trimmed = w.text.trim();
        let punct = trimmed.ends_with(['.', ',', '!', '?', ':', ';', '…']);
        let gap = next.is_none_or(|n| n.start_ms - w.end_ms > 250.0);
        if cur.len() >= max || punct || gap {
            let end = next.map_or(w.end_ms + 400.0, |n| n.start_ms);
            let start_ms = cur.first().map_or(0.0, |w| w.start_ms);
            pages.push(Page {
                words: std::mem::take(&mut cur),
                start_ms,
                end_ms: end,
            });
        }
    }
    // `gap` only fires when we can see the next word, so a trailing run can
    // still be open when the list ends without punctuation. Close it.
    if !cur.is_empty() {
        let last_end = cur.last().map_or(0.0, |w| w.end_ms);
        let start_ms = cur.first().map_or(0.0, |w| w.start_ms);
        pages.push(Page {
            words: cur,
            start_ms,
            end_ms: last_end + 400.0,
        });
    }
    pages
}

/// Draw the taped banner for one scene: a cream paper strip with kraft tape at both corners.
///
/// `t` is the entrance progress (0 before it lands, 1 once settled) — pass
/// [`banner_spring`] so the scale matches the Remotion spring.
#[must_use]
pub fn tape_banner(text: &str, seed: &str, t: f64) -> Svgr<'static> {
    let (w, h) = (W as f64, H as f64);
    let bw = w * 0.88;
    let x = w * 0.06;
    let y = h * 0.14;
    // Fit the banner text to the strip. Archivo Black at 62 px fits ~13 characters in 820 px.
    let size = (820.0 / (text.chars().count().max(1) as f64 * 0.62)).min(62.0);
    let pad = 30.0;
    let bh = size * 1.05 + pad * 2.0;
    let rot = -1.0 + (random(seed) - 0.5) * 0.8;
    let scale = 0.94 + 0.06 * t;
    let label = text.to_uppercase();
    let cx = x + bw / 2.0;
    let cy = y + bh / 2.0;
    let inner = svgr!(<text
        x={cx}
        y={cy + size * 0.36}
        text-anchor="middle"
        font-family={FONT.banner}
        font-weight="400"
        font-size={size}
        letter-spacing={-0.01 * size}
        fill={C::INK}
    >{label}</text>);
    let strip = Slab::new(x, y, bw, bh, C::CREAM)
        .r(2.0)
        .edge(Edge::Torn)
        .grain(0.55)
        .lift(8.0)
        .render(inner);
    let tape = tape_corners(x, y + 16.0, bw, 124.0);
    // Boil the whole banner, then scale it in, both about the strip's centre.
    let boiled = boil(
        0,
        1.5,
        0.2,
        &format!("b{seed}"),
        svgr!(<g>{tape}{strip}</g>),
    );
    svgr!(<g
        transform={format!("translate({cx} {cy}) rotate({rot:.3}) scale({scale:.3}) translate({} {})", -cx, -cy)}
    >{boiled}</g>)
}

/// The banner's entrance spring, as a 0..1 offset from its resting size.
#[must_use]
pub fn banner_spring(frame: usize, fps: usize) -> f64 {
    // damping 14, stiffness 220, mass 0.6 — the Remotion `TapeBanner` config.
    crate::anim::spring(frame as f64, fps as f64, 220.0, 14.0, 0.6)
}

/// The caption band for a frame, from a per-scene table. `bands` maps scene id to the band's
/// vertical fraction, matching the Remotion `bandY` callback.
#[must_use]
pub fn caption_tags(
    frame: usize,
    fps: usize,
    caps: &[Caption],
    band: f64,
    max_per_page: usize,
) -> Svgr<'static> {
    let ms = (frame as f64 / fps as f64) * 1000.0;
    let pages = page_words(caps, max_per_page);
    let Some(page) = pages
        .iter()
        .find(|p| ms >= p.start_ms - 40.0 && ms < p.end_ms)
    else {
        return Svgr::empty();
    };
    let size = 70.0;
    let pad = (12.0, 20.0, 18.0);
    let gap = 12.0;
    let visible: Vec<&Caption> = page
        .words
        .iter()
        .filter(|w| ms >= w.start_ms - 40.0)
        .collect();
    // Measure so the row can be centred.
    let widths: Vec<f64> = visible
        .iter()
        .map(|w| w.text.trim().chars().count().max(1) as f64 * size * 0.60 + pad.1 * 2.0)
        .collect();
    let total: f64 = widths.iter().sum::<f64>() + gap * visible.len().saturating_sub(1) as f64;
    let mut x = (W as f64 - total) / 2.0;
    let base_y = H as f64 * band;
    let mut tags: Vec<Svgr<'static>> = Vec::new();
    for (i, w) in visible.iter().enumerate() {
        let key = format!("{}-{}", w.start_ms, w.text);
        let at = ((w.start_ms / 1000.0) * fps as f64).round() as usize;
        let s = pop_spring(frame as f64 - at as f64, fps as f64);
        let word = w
            .text
            .trim()
            .trim_end_matches(['.', ',', ':', '…'])
            .to_string();
        let tag_w = widths[i];
        let tag_h = size + pad.0 + pad.2;
        let rot = (random(&key) - 0.5) * 10.0;
        let lift = (random(&format!("{key}y")) - 0.5) * 12.0
            + (1.0 - s) * -10.0
            + ((frame as f64 + random(&key) * 40.0) / 9.0).sin() * 1.2;
        let scale = 0.6 + 0.4 * s;
        let cx = x + tag_w / 2.0;
        let cy = base_y + tag_h / 2.0;
        let inner = svgr!(<text
            x={cx}
            y={cy + size * 0.32}
            text-anchor="middle"
            font-family={FONT.caption}
            font-weight="900"
            font-size={size}
            fill={C::TAG_TEXT}
        >{word}</text>);
        let slab = Slab::new(x, base_y, tag_w, tag_h, C::TAG)
            .r(2.0)
            .edge(Edge::Torn)
            .grain(0.55)
            .lift(6.0)
            .render(inner);
        tags.push(svgr!(<g
            transform={format!("translate(0 {lift:.2}) rotate({rot:.2} {cx} {cy}) translate({cx} {cy}) scale({scale:.3}) translate({} {})", -cx, -cy)}
        >{slab}</g>));
        x += tag_w + gap;
    }
    svgr!(<g>{tags}</g>)
}

/// The seven global layers, in the order `PapercutVideo.tsx` stacks them.
///
/// A scene supplies `content`; the kit supplies the walking grain, the caption tags, the banner
/// and the vignette. `banner` is drawn only when the storyboard gave the scene one.
pub struct Overlays<'a> {
    pub frame: usize,
    pub fps: usize,
    pub captions: &'a [Caption],
    pub band: f64,
    pub banner: Option<(&'a str, f64)>,
    pub grain_href: &'a str,
    pub vignette_opacity: f64,
}

impl Overlays<'_> {
    /// Draw the overlays over `content`.
    #[must_use]
    pub fn render<'a>(&self, content: Svgr<'a>) -> Svgr<'a> {
        let grain = grain_overlay(self.frame, self.grain_href);
        let caps = caption_tags(self.frame, self.fps, self.captions, self.band, 3);
        let banner = match self.banner {
            Some((text, t)) => tape_banner(text, text, t),
            None => Svgr::empty(),
        };
        let vig = crate::look::vignette(self.vignette_opacity);
        svgr!(<g>
            {content}
            {banner}
            {caps}
            {grain}
            {vig}
        </g>)
    }
}

/// A single taped note, for a scene that wants one without the full banner layout.
#[must_use]
pub fn note(x: f64, y: f64, w: f64, text: &str, rot: f64) -> Svgr<'static> {
    let h = w * 0.28;
    let size = (h * 0.42).min(48.0);
    let cx = x + w / 2.0;
    let cy = y + h / 2.0;
    let body = text.to_string();
    let inner = svgr!(<text
        x={cx}
        y={cy + size * 0.36}
        text-anchor="middle"
        font-family={FONT.label}
        font-weight="800"
        font-size={size}
        fill={C::INK}
    >{body}</text>);
    let strip = Slab::new(x, y, w, h, C::CREAM)
        .r(2.0)
        .rot(rot)
        .edge(Edge::Torn)
        .lift(6.0)
        .render(inner);
    let tape = paper(4.0, true, svgr!(<g>{strip}</g>));
    tape
}

/// A label chip, thinner than [`crate::kit::pill`] and centred by default.
#[must_use]
pub fn chip(x: f64, y: f64, text: &str, bg: &str, fg: &str, size: f64) -> Svgr<'static> {
    pill(x, y, text, bg, fg, size, 0.0, true)
}

/// The frame the first caption word starts on, so a scene can gate its own entrances.
#[must_use]
pub fn first_word_frame(caps: &[Caption], fps: usize) -> Option<usize> {
    caps.first()
        .map(|c| ((c.start_ms / 1000.0) * fps as f64).round() as usize)
}

/// The default caption timings, so a scene can space beats without reading the plan.
#[must_use]
pub fn word_pop() -> usize {
    crate::tokens::T.word_pop
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{is_blank, markup};

    fn caps() -> Vec<Caption> {
        vec![
            Caption::new("Every", 0.0, 200.0),
            Caption::new("prop", 220.0, 400.0),
            Caption::new("is", 420.0, 500.0),
            Caption::new("paper.", 520.0, 800.0),
            Caption::new("Nothing", 1200.0, 1500.0),
            Caption::new("else.", 1520.0, 1700.0),
        ]
    }

    #[test]
    fn pages_break_on_punctuation_and_pauses() {
        let p = page_words(&caps(), 3);
        assert_eq!(
            p.len(),
            3,
            "got {:?}",
            p.iter().map(|x| x.words.len()).collect::<Vec<_>>()
        );
        assert_eq!(p[0].words.len(), 3);
        assert_eq!(p[1].words.first().map(|w| w.text.as_str()), Some("paper."));
        assert_eq!(p[0].start_ms, 0.0);
        assert_eq!(p[0].end_ms, 520.0);
    }

    #[test]
    fn empty_captions_make_no_pages() {
        assert!(page_words(&[], 3).is_empty());
        assert!(is_blank(caption_tags(0, 30, &[], 0.7, 3)));
    }

    #[test]
    fn captions_show_only_the_active_page() {
        // Words pop in one at a time (`WordTagCaptions` reveals a tag only from
        // its own `startMs`), so frame 0 shows just the first word and the
        // second has landed by frame 8.
        let first = markup(caption_tags(0, 30, &caps(), 0.7, 3));
        assert!(first.contains("Every"));
        assert!(!first.contains("prop"), "second word has not landed yet");

        let out = markup(caption_tags(8, 30, &caps(), 0.7, 3));
        assert!(out.contains("Every") && out.contains("prop"));
        assert!(!out.contains("paper"));
        // Frame 60 (2 s) is in the gap, so nothing is drawn.
        assert!(is_blank(caption_tags(70, 30, &caps(), 0.7, 3)));
    }

    #[test]
    fn caption_tags_draw_the_later_page() {
        let out = markup(caption_tags(40, 30, &caps(), 0.7, 3));
        assert!(out.contains("Nothing"), "got {out}");
    }

    #[test]
    fn banner_fits_long_text() {
        let long = "THIS IS A VERY LONG BANNER LINE";
        let out = markup(tape_banner(long, "b1", 1.0));
        assert!(out.contains("THIS IS A VERY LONG BANNER LINE"));
        assert!(out.contains(&C::CREAM), "banner paper colour");
    }

    #[test]
    fn banner_spring_settles_near_one() {
        assert!(banner_spring(0, 30) < 0.05);
        let settled = banner_spring(60, 30);
        assert!((settled - 1.0).abs() < 0.05, "got {settled}");
    }

    #[test]
    fn overlays_stack_without_a_banner() {
        let o = Overlays {
            frame: 10,
            fps: 30,
            captions: &[],
            band: 0.72,
            banner: None,
            grain_href: "grain.png",
            vignette_opacity: 0.8,
        };
        let out = markup(o.render(svgr!(<rect x="0" y="0" width="10" height="10" fill="#000" />)));
        assert!(out.contains("<rect"));
        assert!(out.contains("grain.png"));
    }

    #[test]
    fn overlays_draw_a_banner_when_given_one() {
        let o = Overlays {
            frame: 20,
            fps: 30,
            captions: &[],
            band: 0.72,
            banner: Some(("EVERY PROP IS PAPER", 1.0)),
            grain_href: "grain.png",
            vignette_opacity: 0.0,
        };
        assert!(markup(o.render(Svgr::empty())).contains("EVERY PROP IS PAPER"));
    }

    #[test]
    fn first_word_frame_rounds() {
        assert_eq!(first_word_frame(&caps(), 30), Some(0));
        assert_eq!(first_word_frame(&[], 30), None);
    }
}
