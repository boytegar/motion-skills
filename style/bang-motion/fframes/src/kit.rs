use crate::tokens::*;

pub fn banner_svg(text: &str) -> String {
    format!(
        r#"<svg width="{}" height="140" viewBox="0 0 {} 140" xmlns="http://www.w3.org/2000/svg">
  <rect x="70" y="20" width="{}" height="100" rx="16" fill="{}" stroke="{}" stroke-width="1.5"/>
  <text x="120" y="80" font-family="Inter, sans-serif" font-size="32" font-weight="bold" fill="{}">{}</text>
  <rect x="{}" y="45" width="80" height="40" rx="8" fill="rgba(99, 102, 241, 0.25)"/>
  <text x="{}" y="71" font-family="monospace" font-size="18" font-weight="bold" fill="{}" text-anchor="middle">BANG</text>
</svg>"#,
        W, W, W - 140.0, palette::SURFACE, palette::ACCENT_LIT, palette::TEXT, text, W - 220.0, W - 180.0, palette::GLOW
    )
}
