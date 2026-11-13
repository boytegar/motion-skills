use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub comp: String,
    pub out: String,
    pub style: String,
    pub engine: String,
    #[serde(default = "default_fps")]
    pub fps: usize,
}

const fn default_fps() -> usize { 30 }

pub struct Reel {
    pub episode: Episode,
}

impl Reel {
    pub fn load(_dir: impl AsRef<Path>) -> Self {
        Self {
            episode: Episode {
                comp: "Episode".into(),
                out: "episode-sketchbook".into(),
                style: "sketchbook".into(),
                engine: "fframes".into(),
                fps: 30,
            }
        }
    }
}
