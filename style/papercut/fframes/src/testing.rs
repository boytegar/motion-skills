use fframes::Svgr;

pub use crate::svgtext::{is_blank, markup};

/// The markup for `v` as it would land in a file, for eyeballing a frame.
pub fn page(v: Svgr<'_>) -> String {
    markup(v)
}
