//! Palette, fonts and timings. Every colour and type choice in the style lives here; a scene
//! module reads `C::hero` or `FONT.banner` and never writes a hex string of its own.

use std::collections::BTreeMap;
use std::sync::LazyLock;

/// The kit palette (paper, wood, board, screens, signals, sparks, brand).
pub mod palette {
    pub const CREAM: &str = "#DACEC0";
    pub const CREAM_HI: &str = "#E8DFD3";
    pub const CREAM_LO: &str = "#BFB2A2";
    pub const TAPE: &str = "#C3B284";
    pub const INK: &str = "#1C1B17";
    pub const INK_SOFT: &str = "#3A3832";

    pub const WOOD: &str = "#7F5231";
    pub const WOOD_LO: &str = "#5E3B22";
    pub const WOOD_HI: &str = "#95653F";
    pub const FLOOR: &str = "#4A3020";
    pub const FLOOR_LO: &str = "#36221A";

    pub const BRASS: &str = "#C89B4A";
    pub const BRASS_LO: &str = "#9C7433";
    pub const METAL: &str = "#B1B9BC";
    pub const METAL_LO: &str = "#8C9598";

    pub const BOARD: &str = "#2E3235";
    pub const BOARD_LO: &str = "#202326";
    pub const BOARD_HI: &str = "#3E4347";
    pub const SAGE: &str = "#8A9A86";
    pub const SAGE_LO: &str = "#667563";

    pub const SCREEN: &str = "#1B1726";
    pub const SCREEN_HI: &str = "#2A2438";
    pub const SCREEN_TEXT: &str = "#E9E2F2";
    pub const SCREEN_DIM: &str = "#8E86A0";

    /// The default hero critter (also the viewer's alias).
    pub const HERO: &str = "#D67354";
    pub const HERO_LO: &str = "#A9553B";

    pub const CAP: &str = "#3968AD";
    pub const CAP_LO: &str = "#284C80";
    pub const TAG: &str = "#C9573C";
    pub const TAG_TEXT: &str = "#F3E6D3";
    pub const PURPLE: &str = "#4E3881";

    pub const GO: &str = "#2DA34D";
    pub const GO_DIM: &str = "#165329";
    pub const NOGO: &str = "#D63A2B";
    pub const NOGO_DIM: &str = "#4D140D";
    pub const AMBER: &str = "#F3A53B";
    pub const AMBER_DIM: &str = "#6B4716";

    pub const SPARK_YELLOW: &str = "#F5C542";
    pub const SPARK_PINK: &str = "#F4A7B9";

    pub const BRAND: &str = "#2563EB";
    pub const BRAND_LO: &str = "#1A47A8";
}

/// Room background colours, rotated per set.
pub fn rooms() -> &'static BTreeMap<&'static str, &'static str> {
    static ROOMS: LazyLock<BTreeMap<&'static str, &'static str>> = LazyLock::new(|| {
        BTreeMap::from([
            ("sage", "#7E8C78"),
            ("navy", "#27324A"),
            ("khaki", "#8A7A60"),
            ("slate", "#4E6378"),
            ("purple", "#5B4A6E"),
            ("teal", "#2F5A5A"),
            ("terracotta", "#9A5A42"),
            ("mustard", "#A88A45"),
            ("grey", "#6E6A64"),
            ("dusk", "#5D7896"),
        ])
    });
    &ROOMS
}

/// One room colour by name, falling back to grey.
pub fn room(name: &str) -> &'static str {
    rooms().get(name).copied().unwrap_or(palette::SAGE)
}

/// Type. Every family has a real file in media/fonts; see `kit::fonts`.
pub struct Fonts {
    pub banner: &'static str,
    pub caption: &'static str,
    pub label: &'static str,
    pub mono: &'static str,
}

pub const FONT: Fonts = Fonts {
    banner: "Archivo Black",
    caption: "Fraunces",
    label: "Inter",
    mono: "JetBrains Mono",
};

/// Kit timings, in frames.
pub struct Timings {
    pub pop: usize,
    pub word_pop: usize,
    pub slide: usize,
    pub press: usize,
    pub blink_hz: f64,
}

pub const T: Timings = Timings {
    pop: 8,
    word_pop: 4,
    slide: 10,
    press: 6,
    blink_hz: 4.0,
};

pub const W: usize = 1080;
pub const H: usize = 1920;
pub const FPS: usize = 30;

/// The banner band (top edge as a fraction of the height) and the caption band.
pub const BANNER_TOP: f64 = 0.14;
/// Bottom of the banner block: keep key props out of y < 420.
pub const BANNER_BOTTOM: f64 = 420.0;
/// Keep faces and key labels out of y 1420..1580 (the caption band).
pub const CAPTIONS_TOP: f64 = 1420.0;
pub const CAPTIONS_BOTTOM: f64 = 1580.0;
/// The bottom 20% stays clear for platform UI.
pub const SAFE_BOTTOM: f64 = 1536.0;
