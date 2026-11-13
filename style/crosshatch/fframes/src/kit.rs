use crate::tokens::*;

pub fn banner_svg(text: &str) -> String {
    format!(
        r#"<svg width="{}" height="120" viewBox="0 0 {} 120" xmlns="http://www.w3.org/2000/svg">
  <rect x="40" y="10" width="{}" height="100" fill="{}" rx="6"/>
  <text x="540" y="72" text-anchor="middle" font-family="sans-serif" font-size="36" font-weight="bold" fill="#ffffff">{}</text>
</svg>"#,
        W, W, W - 80.0, palette::HERO, text
    )
}
