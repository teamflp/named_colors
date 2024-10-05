pub mod colors;

use colors::load_colors;

pub fn get_color_by_name(color_name: &str) -> Option<(u8, u8, u8)> {
    let color_data = load_colors().unwrap_or_default();
    if let Some(color) = color_data.get(color_name) {
        let r = color["r"].as_u64().unwrap_or(0) as u8;
        let g = color["g"].as_u64().unwrap_or(0) as u8;
        let b = color["b"].as_u64().unwrap_or(0) as u8;
        return Some((r, g, b));
    }
    None
}
