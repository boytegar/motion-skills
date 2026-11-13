//! The cover title: torn paper tags, one per word, that slap in and clear on `outAt`.
//!
//! Port of `../remotion/kit/paper/hook.tsx`. The Remotion version lays out with flexbox; SVG has
//! no flexbox, so this measures the tags (approximate glyph metrics, like [`crate::kit::pill`])
//! and centres each line by hand.

use fframes::{svgr, Svgr};

use crate::anim::{map, random};
use crate::kit::{grain_fill, Edge, Slab};
use crate::tokens::{palette as C, FONT};

/// One torn paper tag. Sizes are derived from the font size so a scene only picks `size`.
#[derive(Debug, Clone)]
pub struct TornTag {
    pub word: String,
    /// Draw on the accent (spark yellow) paper instead of the tag red.
    pub hot: bool,
    pub size: f64,
    /// Stable per-tag seed for its tilt, offset and grain.
    pub seed: String,
    /// Extra degrees of rotation, for the stop-motion boil.
    pub jitter: f64,
}

impl TornTag {
    pub fn new(word: &str, hot: bool, size: f64, seed: &str) -> Self {
        TornTag {
            word: word.to_string(),
            hot,
            size,
            seed: seed.to_string(),
            jitter: 0.0,
        }
    }

    pub fn jitter(mut self, deg: f64) -> Self {
        self.jitter = deg;
        self
    }

    /// Width of the tag, from the caption font's rough metrics.
    #[must_use]
    pub fn width(&self) -> f64 {
        // Fraunces 900 is a wide serif. 0.60em per glyph plus the 0.52em of horizontal padding
        // tracks the fit of the rendered title closely enough to centre a line.
        self.word.chars().count() as f64 * self.size * 0.60 + self.size * 0.52
    }

    /// Height of the tag: one line of captions plus the top and bottom padding.
    #[must_use]
    pub fn height(&self) -> f64 {
        self.size * 1.0 + self.size * 0.40
    }

    /// Draw the tag with its top-left corner at `(x, y)`, returning the tag and its extent.
    pub fn render(&self, x: f64, y: f64) -> Svgr<'static> {
        let bg = if self.hot { C::SPARK_YELLOW } else { C::TAG };
        let fg = if self.hot { C::INK } else { C::TAG_TEXT };
        let rot = (random(&self.seed) - 0.5) * 9.0 + self.jitter;
        let dy = (random(&format!("{}y", self.seed)) - 0.5) * 14.0;
        let (w, h) = (self.width(), self.height());
        let text = self.word.clone();
        let size = self.size;
        let anchor = x + w / 2.0;
        let baseline = y + self.size * 0.16 + self.size * 0.78;
        Slab::new(x, y + dy, w, h, bg)
            .r(2.0)
            .rot(rot)
            .edge(Edge::Torn)
            .grain(0.55)
            .lift(8.0)
            .render(svgr!(<text
                x={anchor}
                y={baseline}
                text-anchor="middle"
                font-family={FONT.caption}
                font-weight="900"
                font-size={size}
                fill={fg}
            >{text}</text>))
    }
}

/// The cover title: `lines` of `TornTag`s, centred in a column `top` px from the top.
///
/// `frame` is scene-local. The title slaps in over the first 10 frames, holds, then exits upward
/// across the 7 frames before `out_at`.
#[derive(Debug, Clone)]
pub struct HookTitle {
    pub lines: Vec<Vec<String>>,
    /// Words drawn on the accent paper.
    pub hot: Vec<String>,
    /// Frame the title has fully left on.
    pub out_at: usize,
    pub top: f64,
    pub size: f64,
    pub seed: String,
}

impl HookTitle {
    pub fn new(lines: Vec<Vec<String>>, hot: Vec<String>, out_at: usize) -> Self {
        HookTitle {
            lines,
            hot,
            out_at,
            top: 220.0,
            size: 108.0,
            seed: "cover".to_string(),
        }
    }

    pub fn top(mut self, top: f64) -> Self {
        self.top = top;
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    pub fn seed(mut self, seed: &str) -> Self {
        self.seed = seed.to_string();
        self
    }

    /// The tags of one line, already positioned.
    fn line_tags(
        &self,
        line: &[String],
        row: usize,
        top: f64,
        step: usize,
    ) -> (Vec<Svgr<'static>>, f64) {
        let gap = self.size * 0.14;
        let tags: Vec<TornTag> = line
            .iter()
            .enumerate()
            .map(|(j, w)| {
                let seed = format!("{}-{row}-{j}-{w}", self.seed);
                let jitter = (random(&format!("{seed}-{step}")) - 0.5) * 1.4;
                TornTag::new(w, self.hot.iter().any(|h| h == w), self.size, &seed).jitter(jitter)
            })
            .collect();
        let total: f64 = tags.iter().map(TornTag::width).sum::<f64>()
            + gap * (tags.len().saturating_sub(1)) as f64;
        let mut x = (crate::tokens::W as f64 - total) / 2.0;
        let out = tags
            .iter()
            .map(|t| {
                let svg = t.render(x, top);
                x += t.width() + gap;
                svg
            })
            .collect();
        (out, total)
    }

    /// Draw at `frame`, or nothing once the title has left.
    ///
    /// `out_at == 0` is a real value, not a sentinel: it means the title is
    /// never shown, so frame 0 is already gone.
    #[must_use]
    pub fn render(&self, frame: usize) -> Svgr<'static> {
        if frame >= self.out_at {
            return Svgr::empty();
        }
        // Slap-in: a decaying sine squeeze over the first 10 frames.
        let slap = if frame < 10 {
            (frame as f64 * 1.7).sin() * (1.0 - frame as f64 / 10.0) * 0.045
        } else {
            0.0
        };
        // Exit: back-eased upward across the last 7 frames.
        let tail = self.out_at.saturating_sub(7).max(1);
        let out = crate::anim::in_back(
            map(
                frame as f64,
                tail as f64,
                self.out_at.max(tail + 1) as f64,
                0.0,
                1.0,
            ),
            1.6,
        );

        let step = frame / 3;
        let gap = self.size * 0.2;
        let mut y = self.top;
        let mut rows: Vec<Svgr<'static>> = Vec::new();
        for (i, line) in self.lines.iter().enumerate() {
            let (tags, _) = self.line_tags(line, i, y, step);
            rows.extend(tags);
            y += self.size + gap;
        }

        // Transform about 40% height, matching the Remotion `transformOrigin: '50% 40%'`.
        let cx = crate::tokens::W as f64 / 2.0;
        let cy = self.top
            + (self.size * (self.lines.len() as f64)
                + gap * self.lines.len().saturating_sub(1) as f64)
                * 0.4;
        let scale = 1.0 + slap;
        let ty = -out * 640.0;
        let rot = out * -4.0;
        let opacity = 1.0 - out * 0.4;
        svgr!(<g
            transform={format!("translate(0 {ty:.2}) rotate({rot:.3} {cx} {cy}) translate({cx} {cy}) scale({scale:.4}) translate({} {})", -cx, -cy)}
            opacity={opacity}
        >{rows}</g>)
    }

    /// Build from the plan's hook block, if the episode has one.
    #[must_use]
    pub fn from_spec(spec: &crate::video::HookSpec) -> Self {
        HookTitle::new(spec.lines.clone(), spec.hot.clone(), spec.out_at)
    }
}

/// The grain fill for a tag colour, exposed so a scene can match the title's paper.
#[must_use]
pub fn tag_fill(hot: bool) -> String {
    grain_fill(if hot { C::SPARK_YELLOW } else { C::TAG }, 0.55)
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{is_blank, markup};

    #[test]
    fn tag_size_follows_the_font() {
        let a = TornTag::new("CUT", false, 100.0, "a");
        let b = TornTag::new("CUT", false, 200.0, "a");
        assert!(b.width() > a.width());
        assert!(b.height() > a.height());
    }

    #[test]
    fn hot_words_use_the_accent_paper() {
        let t = TornTag::new("FREE", true, 100.0, "x");
        let out = markup(t.render(0.0, 0.0));
        assert!(
            out.to_lowercase().contains(&C::SPARK_YELLOW.to_lowercase()),
            "expected spark yellow in {out}"
        );
    }

    #[test]
    fn title_is_gone_from_its_out_frame_on() {
        let t = HookTitle::new(vec![vec!["ALL".into(), "CUT".into()]], vec![], 40);
        // Frame 0 is the cover: Remotion draws it and bounds it with a
        // `Sequence` of `outAt + 1`, so it must be visible.
        assert!(!is_blank(t.render(0)));
        assert!(!is_blank(t.render(39)));
        assert!(is_blank(t.render(40)));
        assert!(is_blank(t.render(400)));
    }

    #[test]
    fn title_renders_every_word_while_holding() {
        let t = HookTitle::new(vec![vec!["ALL".into()], vec!["CUT".into()]], vec![], 100);
        let out = markup(t.render(30));
        assert!(out.contains("ALL") && out.contains("CUT"), "got {out}");
    }

    #[test]
    fn title_is_deterministic() {
        let t = HookTitle::new(vec![vec!["PAPER".into()]], vec!["PAPER".into()], 90);
        assert_eq!(markup(t.render(17)), markup(t.render(17)));
    }

    #[test]
    fn builds_from_a_hook_spec() {
        let spec = crate::video::HookSpec {
            lines: vec![vec!["$180".into()], vec!["PER".into(), "YEAR".into()]],
            hot: vec!["$180".into()],
            out_at: 78,
            hold: 2.6,
        };
        let t = HookTitle::from_spec(&spec);
        assert_eq!(t.out_at, 78);
        assert_eq!(t.lines.len(), 2);
        assert!(markup(t.render(10)).contains("$180"));
    }
}
