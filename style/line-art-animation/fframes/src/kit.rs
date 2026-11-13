use crate::tokens::*;

pub fn banner_svg(text: &str) -> String {
    format!(
        r#"<svg width="{}" height="120" viewBox="0 0 {} 120" xmlns="http://www.w3.org/2000/svg">
  <line x1="100" y1="20" x2="{}" y2="20" stroke="{}" stroke-width="1.5"/>
  <line x1="100" y1="100" x2="{}" y2="100" stroke="{}" stroke-width="1.5"/>
  <text x="540" y="68" font-family="serif" font-size="34" letter-spacing="4" text-anchor="middle" fill="{}">{}</text>
</svg>"#,
        W, W, W - 100.0, palette::INK, W - 100.0, palette::INK, palette::INK, text
    )
}
