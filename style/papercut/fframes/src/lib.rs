//! Papercut style engine for the motion-explainer skills: builds an explainer reel from
//! `episode.json` + `plan.json` with fframes.
//!
//! The kit mirrors the Remotion one in `../remotion/kit/`, so the same storyboard renders with
//! either engine. Read `AGENT_BRIEF.md` in the style folder before adding scenes.

pub mod anim;
pub mod camera;
pub mod cast;
pub mod cut;
pub mod gen;
pub mod hook;
pub mod kit;
pub mod look;
pub mod overlay;
pub mod props;
pub mod sets;
pub mod svgtext;
pub mod testing;
pub mod tokens;
pub mod ui;
pub mod video;

pub use anim::{bob, hop, in_back, out_back, reach, Animation, Ease, Random};
pub use camera::*;
pub use cast::{Acc, Critter, CritterOpts, Expr};
pub use cut::{CutKind, CutSeries, CutSpec};
pub use hook::{HookTitle, TornTag};
pub use kit::*;
pub use look::*;
pub use sets::*;
pub use tokens::{palette as C, rooms, FONT, H, T, W};
pub use video::{Episode, HookSpec, Line, Plan, PlanScene, PlanWord, Reel, WordRef};

pub use fframes;
pub use fframes::svgr;
pub use fframes::{
    AudioMap, AudioTimestamp, FFramesContext, Frame, Overlap, Scene, SceneInfo, Scenes, Svgr, Video,
};

/// Everything a scene needs at render time: the episode config and the frame-exact timeline.
pub struct Ctx<'a> {
    pub reel: &'a Reel,
    pub scene: &'a PlanScene,
}

impl Ctx<'_> {
    /// The frame a word starts on, counted from the scene, not the reel.
    ///
    /// `plan.json` stores reel-absolute frames; scenes want scene-local ones so their entrances
    /// keep working when a neighbour's line changes length. Panics on a missing word.
    #[must_use]
    pub fn cue(&self, word: &str, n: usize) -> usize {
        self.reel.cue(&self.scene.id, word, n) - self.scene.from
    }

    /// Like [`cue`](Self::cue) but `None` when the word is missing.
    #[must_use]
    pub fn cue_opt(&self, word: &str, n: usize) -> Option<usize> {
        self.reel
            .cue_opt(&self.scene.id, word, n)
            .map(|f| f - self.scene.from)
    }

    /// Frames in this scene.
    #[must_use]
    pub const fn dur(&self) -> usize {
        self.scene.duration_in_frames
    }

    /// The frame's progress across the scene, 0..1.
    #[must_use]
    pub fn t(&self, frame: usize) -> f64 {
        anim::clamp01(frame as f64 / self.dur().max(1) as f64)
    }
}

/// Lowercase, alphanumerics only. `plan.json` words carry punctuation; cues do not.
pub fn norm(w: &str) -> String {
    w.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}
