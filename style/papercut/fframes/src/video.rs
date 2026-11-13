//! `episode.json` and `plan.json` as typed Rust. The shapes match what the Remotion engine
//! reads, so one episode folder drives either engine.
//!
//! `plan.json` is produced by `core/tools/build_vo.py`; `episode.json` is written by hand and
//! `new_episode.sh` fills in `comp`, `out`, `style` and `engine`.

use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// One narration line in `script.json`.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Line {
    pub id: String,
    pub text: String,
}

/// The hook cover title, already resolved to frames by `build_vo.py`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HookSpec {
    /// Lines of the cover title, one `Vec` per line.
    #[serde(default)]
    pub lines: Vec<Vec<String>>,
    /// The words drawn in the accent colour.
    #[serde(default)]
    pub hot: Vec<String>,
    /// Frame the title leaves on.
    #[serde(default, rename = "outAt")]
    pub out_at: usize,
    /// Hold in seconds, kept for round-tripping; `build_vo.py` already folded it into `out_at`.
    #[serde(default)]
    pub hold: f64,
}

/// A cut between two scenes.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CutSpec {
    /// `tapePull`, `whip`, `cardFlip`, ... — see the style's transition series.
    #[serde(default)]
    pub kind: String,
    /// Where in the scene the cut lands, in seconds.
    #[serde(default)]
    pub at: f64,
}

/// `episode.json`: the episode-level config, shared by both engines.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Episode {
    /// Remotion composition id (`new_episode.sh` writes it).
    #[serde(default)]
    pub comp: String,
    /// Output file stem.
    #[serde(default)]
    pub out: String,
    /// Which style folder this episode renders with.
    #[serde(default)]
    pub style: String,
    /// `remotion` or `fframes`.
    #[serde(default = "default_engine")]
    pub engine: String,
    /// Music bed preset name from `core/audio/beds/`.
    #[serde(default)]
    pub bed: String,
    /// Relative path to the folder holding `script.json` and the takes.
    #[serde(default)]
    pub narration: String,
    /// Timing file name, when a custom one is used.
    #[serde(default)]
    pub timing: String,
    #[serde(default = "default_fps")]
    pub fps: usize,
    /// Silence before the first line, in seconds.
    #[serde(default)]
    pub start: f64,
    /// Extra gap between lines, in seconds.
    #[serde(default)]
    pub gap: f64,
    /// Tail hold after a line, in seconds.
    #[serde(default)]
    pub hold: f64,
    /// Per-line extra beat, keyed by narration line id.
    #[serde(default)]
    pub gaps: BTreeMap<String, f64>,
    /// The scene the whip transition lands on.
    #[serde(default, rename = "whipScene")]
    pub whip_scene: String,
    #[serde(default)]
    pub hook: Option<HookSpec>,
    /// `[scene id, narration line id, first word (index or exact word)]`.
    #[serde(default)]
    pub scenes: Vec<(String, String, WordRef)>,
    /// Taped banner text per scene id.
    #[serde(default)]
    pub banners: BTreeMap<String, String>,
    /// Cut per scene id.
    #[serde(default)]
    pub cuts: BTreeMap<String, CutSpec>,
    /// Sound effect name used at each cut.
    #[serde(default, rename = "cutSfx")]
    pub cut_sfx: String,
}

fn default_engine() -> String {
    "remotion".to_string()
}

const fn default_fps() -> usize {
    30
}

impl Default for Episode {
    fn default() -> Self {
        Episode {
            comp: String::new(),
            out: String::new(),
            style: String::new(),
            engine: default_engine(),
            bed: String::new(),
            narration: String::new(),
            timing: String::new(),
            fps: default_fps(),
            start: 0.0,
            gap: 0.0,
            hold: 0.0,
            gaps: BTreeMap::new(),
            whip_scene: String::new(),
            hook: None,
            scenes: Vec::new(),
            banners: BTreeMap::new(),
            cuts: BTreeMap::new(),
            cut_sfx: String::new(),
        }
    }
}

/// The third element of a `scenes` entry: a word index or the exact word. `episode.json` uses
/// both, and `0` means "the first word".
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WordRef {
    Index(usize),
    Word(String),
}

impl WordRef {
    /// The first word to cut on, as an index into the scene's `words`.
    #[must_use]
    pub fn index(&self) -> usize {
        match self {
            WordRef::Index(i) => *i,
            WordRef::Word(_) => 0,
        }
    }

    /// The exact word to cut on, when the entry gives one.
    #[must_use]
    pub fn word(&self) -> Option<&str> {
        match self {
            WordRef::Index(_) => None,
            WordRef::Word(w) => Some(w),
        }
    }
}

impl Default for WordRef {
    fn default() -> Self {
        WordRef::Index(0)
    }
}

/// One word with the frame it starts on, as `build_vo.py` writes it.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlanWord {
    pub text: String,
    /// Frame the word starts on, relative to the reel.
    pub f: usize,
    /// The narration line this word belongs to.
    #[serde(default)]
    pub line: String,
    /// Optional per-word timing in milliseconds, kept from the caption pass.
    #[serde(default, rename = "startMs")]
    pub start_ms: Option<u64>,
    #[serde(default, rename = "endMs")]
    pub end_ms: Option<u64>,
}

/// One scene in `plan.json`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlanScene {
    pub id: String,
    /// First frame of the scene in the reel.
    pub from: usize,
    #[serde(rename = "durationInFrames")]
    pub duration_in_frames: usize,
    #[serde(default)]
    pub words: Vec<PlanWord>,
}

/// `plan.json`: the frame-exact timeline `build_vo.py` writes.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Plan {
    #[serde(default = "default_fps")]
    pub fps: usize,
    #[serde(default, rename = "durationInFrames")]
    pub duration_in_frames: usize,
    #[serde(default)]
    pub scenes: Vec<PlanScene>,
    #[serde(default)]
    pub hook: Option<HookSpec>,
}

impl Plan {
    /// The scene with this id. Panics with a useful message, because a plan missing a scene the
    /// storyboard names is a build error, not something to paper over at render time.
    #[must_use]
    pub fn scene(&self, id: &str) -> &PlanScene {
        self.scenes
            .iter()
            .find(|s| s.id == id)
            .unwrap_or_else(|| panic!("no scene {id} in plan.json (have: {:?})", self.ids()))
    }

    /// Every scene id, in timeline order.
    #[must_use]
    pub fn ids(&self) -> Vec<&str> {
        self.scenes.iter().map(|s| s.id.as_str()).collect()
    }

    /// Frame the scene with this id starts on.
    #[must_use]
    pub fn from_of(&self, id: &str) -> usize {
        self.scene(id).from
    }

    /// Frames in the scene with this id.
    #[must_use]
    pub fn dur_of(&self, id: &str) -> usize {
        self.scene(id).duration_in_frames
    }

    /// The scene showing at `frame`, if any.
    #[must_use]
    pub fn at(&self, frame: usize) -> Option<&PlanScene> {
        self.scenes
            .iter()
            .find(|s| frame >= s.from && frame < s.from + s.duration_in_frames)
    }

    /// The taped banner active at `frame`, by scene id. Empty when the scene has no banner.
    #[must_use]
    pub fn banner_at<'a>(
        &'a self,
        frame: usize,
        banners: &'a BTreeMap<String, String>,
    ) -> Option<&'a str> {
        let s = self.at(frame)?;
        banners.get(&s.id).map(String::as_str)
    }
}

/// Read a JSON file into `T`. Panics with the path on failure, so a bad edit names itself.
pub fn load<T: for<'de> Deserialize<'de>>(path: impl AsRef<Path>) -> T {
    let path = path.as_ref();
    let src = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&src).unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
}

/// The episode pair every scene needs: the config and the timeline.
#[derive(Debug, Clone)]
pub struct Reel {
    pub ep: Episode,
    pub plan: Plan,
}

impl Reel {
    /// Load `episode.json` and `plan.json` from a project folder (the folder holding both).
    #[must_use]
    pub fn load(dir: impl AsRef<Path>) -> Self {
        let dir = dir.as_ref();
        Reel {
            ep: load(dir.join("episode.json")),
            plan: load(dir.join("plan.json")),
        }
    }

    /// The narration line for a scene, by the `scenes` table in `episode.json`.
    #[must_use]
    pub fn line_of(&self, scene_id: &str) -> Option<&str> {
        self.ep
            .scenes
            .iter()
            .find(|(s, _, _)| s == scene_id)
            .map(|(_, l, _)| l.as_str())
    }

    /// The banner for a scene, if it has one.
    #[must_use]
    pub fn banner_of(&self, scene_id: &str) -> Option<&str> {
        self.ep.banners.get(scene_id).map(String::as_str)
    }

    /// The cut that starts a scene, if the storyboard gave one.
    #[must_use]
    pub fn cut_of(&self, scene_id: &str) -> Option<&CutSpec> {
        self.ep.cuts.get(scene_id)
    }

    /// The frame a word starts on, within a scene. `n` counts repeats of the same word.
    ///
    /// Panics when the word is not in the scene: a cue that does not exist is a storyboard bug,
    /// and failing at build time is better than an element that silently never appears.
    #[must_use]
    pub fn cue(&self, scene_id: &str, word: &str, n: usize) -> usize {
        self.cue_opt(scene_id, word, n)
            .unwrap_or_else(|| panic!("cue \"{word}\"#{n} not in {scene_id}"))
    }

    /// Like [`cue`](Self::cue) but `None` when the word is missing, for optional beats.
    #[must_use]
    pub fn cue_opt(&self, scene_id: &str, word: &str, n: usize) -> Option<usize> {
        let want = crate::norm(word);
        let mut k = 0;
        for w in &self.plan.scene(scene_id).words {
            if crate::norm(&w.text).starts_with(&want) {
                if k == n {
                    return Some(w.f);
                }
                k += 1;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EP: &str = r#"{
        "comp": "Example", "out": "example", "style": "papercut", "engine": "fframes",
        "fps": 30, "scenes": [["S01Hello", "l1_hook", 0], ["S02Cta", "l2_cta", "For"]],
        "banners": {"S01Hello": "EVERY PROP IS PAPER"},
        "cuts": {"S02Cta": {"kind": "tapePull"}}
    }"#;

    const PLAN: &str = r#"{
        "fps": 30, "durationInFrames": 120,
        "scenes": [
            {"id": "S01Hello", "from": 0, "durationInFrames": 90,
             "words": [{"text": "Hello", "f": 6}, {"text": "paper", "f": 20}]},
            {"id": "S02Cta", "from": 90, "durationInFrames": 30,
             "words": [{"text": "For", "f": 95}]}
        ],
        "hook": {"lines": [["ALL", "CUT"]], "hot": ["PAPER"], "outAt": 40}
    }"#;

    fn reel() -> Reel {
        Reel {
            ep: serde_json::from_str(EP).expect("episode.json"),
            plan: serde_json::from_str(PLAN).expect("plan.json"),
        }
    }

    #[test]
    fn parses_both_files() {
        let r = reel();
        assert_eq!(r.ep.comp, "Example");
        assert_eq!(r.ep.engine, "fframes");
        assert_eq!(r.plan.scenes.len(), 2);
        assert_eq!(r.plan.duration_in_frames, 120);
    }

    #[test]
    fn word_ref_takes_an_index_or_a_word() {
        let r = reel();
        assert_eq!(r.ep.scenes[0].2.index(), 0);
        assert_eq!(r.ep.scenes[0].2.word(), None);
        assert_eq!(r.ep.scenes[1].2.index(), 0);
        assert_eq!(r.ep.scenes[1].2.word(), Some("For"));
    }

    #[test]
    fn cue_finds_words_and_panics_on_misses() {
        let r = reel();
        assert_eq!(r.cue("S01Hello", "paper", 0), 20);
        assert_eq!(r.cue_opt("S01Hello", "nope", 0), None);
    }

    #[test]
    fn timeline_lookups_work() {
        let r = reel();
        assert_eq!(r.plan.from_of("S02Cta"), 90);
        assert_eq!(r.plan.dur_of("S01Hello"), 90);
        assert_eq!(r.plan.at(95).map(|s| s.id.as_str()), Some("S02Cta"));
        assert!(r.plan.at(500).is_none());
        assert_eq!(r.banner_of("S01Hello"), Some("EVERY PROP IS PAPER"));
        assert_eq!(
            r.cut_of("S02Cta").map(|c| c.kind.as_str()),
            Some("tapePull")
        );
        assert!(r.ep.hook.is_none());
        assert_eq!(r.plan.hook.as_ref().map(|h| h.out_at), Some(40));
    }

    #[test]
    fn line_of_reads_the_scenes_table() {
        let r = reel();
        assert_eq!(r.line_of("S01Hello"), Some("l1_hook"));
        assert_eq!(r.line_of("S03Nope"), None);
    }
}
