//! Serialise a [`Svgr`] back to standalone SVG markup.
//!
//! The kit builds *fragments*: they reference paint servers (`url(#pc-torn)`,
//! `url(#pc-grain-…)`) and font families that only resolve once the kit `<defs>`
//! are mounted and `media/fonts` is loaded. Handing a bare fragment to `usvgr`
//! drops everything it cannot resolve, which makes it useless for asserting on
//! what a scene actually emits.
//!
//! So this walks the compiled tree straight back to text, keeping every
//! attribute verbatim and splicing the kit `<defs>` into the root once. That is
//! the same document the renderer assembles for a real frame, minus the
//! normalisation pass — which is exactly what the tests want to look at.

use fframes::usvgr::roxmltree::StringStorage;
use fframes::usvgr::svgtree::svgrtypes::{Length, LengthUnit, PathSegment};
use fframes::usvgr::svgtree::{NestedNodeData, NestedNodeKind, SvgAttributeValue};
use fframes::Svgr;

const NS: &str = "http://www.w3.org/2000/svg";

/// Serialise `v` inside a `W`×`H` root that carries the kit `<defs>`.
///
/// Only the grain patterns the fragment actually references are mounted: the
/// full palette is ~170 patterns, which would bury every colour assertion in
/// the tests under `<defs>` noise.
pub fn markup(v: Svgr<'_>) -> String {
    let (w, h) = (crate::tokens::W, crate::tokens::H);
    let root = format!(r#"<svg xmlns="{NS}" width="{w}" height="{h}" viewBox="0 0 {w} {h}">"#);
    let mut body = String::with_capacity(8192);
    nodes(&v, &mut body);

    let mut out = String::with_capacity(body.len() + 4096);
    out.push_str(&root);
    nodes(&crate::kit::defs(), &mut out);
    nodes(&crate::sets::room_defs(), &mut out);
    let used = used_grain_keys(&body);
    if !used.is_empty() {
        nodes(
            &crate::kit::grain_patterns(&pairs_for(&used), "grain.png"),
            &mut out,
        );
    }
    out.push_str(&body);
    out.push_str("</svg>");
    out
}

/// Every `pc-grain-<hex>-<pct>` id referenced by `body`, as `(hex, pct)`.
fn used_grain_keys(body: &str) -> Vec<(String, f64)> {
    let mut out: Vec<(String, f64)> = Vec::new();
    let mut rest = body;
    while let Some(i) = rest.find("pc-grain-") {
        rest = &rest[i + "pc-grain-".len()..];
        let end = rest.find([')', '"', ' ', '\'']).unwrap_or(rest.len());
        let key = &rest[..end];
        rest = &rest[end..];
        if let Some((hex, pct)) = key.split_once('-') {
            let Ok(pct) = pct.parse::<f64>() else {
                continue;
            };
            let hex = format!("#{hex}");
            if !out.iter().any(|(h, p)| *h == hex && *p == pct) {
                out.push((hex, pct));
            }
        }
    }
    out
}

fn pairs_for(keys: &[(String, f64)]) -> Vec<(&str, f64)> {
    keys.iter().map(|(h, p)| (h.as_str(), *p)).collect()
}

/// True when `v` draws nothing but `<defs>`, `<title>`, `<desc>` or whitespace.
pub fn is_blank(v: Svgr<'_>) -> bool {
    !v.svg_tree.nodes.iter().flatten().any(visible)
}

/// True when `v` is the empty fragment (no nodes at all).
pub fn no_content(v: &Svgr<'_>) -> bool {
    v.svg_tree.nodes.iter().flatten().next().is_none()
}

fn visible(n: &NestedNodeData<'_>) -> bool {
    match &n.kind {
        NestedNodeKind::Root => false,
        NestedNodeKind::Text(t) => !str_of(t).trim().is_empty(),
        NestedNodeKind::Element { tag_name } => {
            !matches!(tag_name.to_str(), "defs" | "title" | "desc" | "metadata")
        }
    }
}

fn nodes(v: &Svgr<'_>, out: &mut String) {
    for node in &v.svg_tree.nodes {
        if let Some(node) = node {
            write_node(node, out);
        }
    }
}

fn write_node(n: &NestedNodeData<'_>, out: &mut String) {
    match &n.kind {
        NestedNodeKind::Root => {}
        NestedNodeKind::Text(t) => escape(str_of(t), out),
        NestedNodeKind::Element { tag_name } => {
            let tag = tag_name.to_str();
            out.push('<');
            out.push_str(tag);
            for attr in n.attrs.iter() {
                let Some(value) = attr_text(&attr.value) else {
                    continue;
                };
                if value.is_empty() {
                    continue;
                }
                out.push(' ');
                out.push_str(attr.name.to_str());
                out.push_str("=\"");
                escape(&value, out);
                out.push('"');
            }
            if n.children.iter().flatten().next().is_none() {
                out.push_str("/>");
                return;
            }
            out.push('>');
            for child in &n.children {
                if let Some(child) = child {
                    write_node(child, out);
                }
            }
            out.push_str("</");
            out.push_str(tag);
            out.push('>');
        }
    }
}

fn attr_text(v: &SvgAttributeValue<'_>) -> Option<String> {
    Some(match v {
        SvgAttributeValue::Float(f, storage) => match str_of(storage) {
            "" => num(*f),
            s => s.to_string(),
        },
        SvgAttributeValue::Length(l) => length(l),
        SvgAttributeValue::Transform(t) => {
            let default =
                t.a == 1.0 && t.b == 0.0 && t.c == 0.0 && t.d == 1.0 && t.e == 0.0 && t.f == 0.0;
            if default {
                String::new()
            } else {
                format!("matrix({} {} {} {} {} {})", t.a, t.b, t.c, t.d, t.e, t.f)
            }
        }
        SvgAttributeValue::Color(c) => format!("#{:02x}{:02x}{:02x}", c.red, c.green, c.blue),
        SvgAttributeValue::StringStorage(s) => str_of(s).to_string(),
        SvgAttributeValue::PathData(d) => path(d),
        SvgAttributeValue::StaticPath(p) => path(p.segments()),
        // `<image>` sources are resolved by the renderer, not by markup.
        SvgAttributeValue::ImageData(_) => return None,
    })
}

fn str_of<'a>(s: &'a StringStorage<'a>) -> &'a str {
    match s {
        StringStorage::Borrowed(s) => s,
        StringStorage::Owned(s) => s,
    }
}

fn num(f: f32) -> String {
    if f.is_finite() && f.fract() == 0.0 && f.abs() < 1e7 {
        (f as i64).to_string()
    } else {
        f.to_string()
    }
}

fn length(l: &Length) -> String {
    let n = l.number;
    let n = if n.fract() == 0.0 {
        (n as i64).to_string()
    } else {
        n.to_string()
    };
    let unit = match l.unit {
        LengthUnit::None => "",
        LengthUnit::Em => "em",
        LengthUnit::Ex => "ex",
        LengthUnit::Px => "px",
        LengthUnit::In => "in",
        LengthUnit::Cm => "cm",
        LengthUnit::Mm => "mm",
        LengthUnit::Pt => "pt",
        LengthUnit::Pc => "pc",
        LengthUnit::Percent => "%",
    };
    format!("{n}{unit}")
}

fn path(segments: &[PathSegment]) -> String {
    let mut d = String::new();
    for seg in segments {
        if !d.is_empty() && !matches!(seg, PathSegment::ClosePath { .. }) {
            d.push(' ');
        }
        let mut cmd = |abs: bool, upper: char, rest: String| {
            d.push(if abs {
                upper
            } else {
                upper.to_ascii_lowercase()
            });
            if !rest.is_empty() {
                d.push(' ');
                d.push_str(&rest);
            }
        };
        match *seg {
            PathSegment::MoveTo { abs, x, y } => cmd(abs, 'M', format!("{x} {y}")),
            PathSegment::LineTo { abs, x, y } => cmd(abs, 'L', format!("{x} {y}")),
            PathSegment::HorizontalLineTo { abs, x } => cmd(abs, 'H', x.to_string()),
            PathSegment::VerticalLineTo { abs, y } => cmd(abs, 'V', y.to_string()),
            PathSegment::CurveTo {
                abs,
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => cmd(abs, 'C', format!("{x1} {y1} {x2} {y2} {x} {y}")),
            PathSegment::SmoothCurveTo { abs, x2, y2, x, y } => {
                cmd(abs, 'S', format!("{x2} {y2} {x} {y}"))
            }
            PathSegment::Quadratic { abs, x1, y1, x, y } => {
                cmd(abs, 'Q', format!("{x1} {y1} {x} {y}"))
            }
            PathSegment::SmoothQuadratic { abs, x, y } => cmd(abs, 'T', format!("{x} {y}")),
            PathSegment::EllipticalArc {
                abs,
                rx,
                ry,
                x_axis_rotation,
                large_arc,
                sweep,
                x,
                y,
            } => cmd(
                abs,
                'A',
                format!(
                    "{rx} {ry} {x_axis_rotation} {} {} {x} {y}",
                    u8::from(large_arc),
                    u8::from(sweep)
                ),
            ),
            PathSegment::ClosePath { abs } => cmd(abs, 'Z', String::new()),
        }
    }
    d
}

fn escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
}
