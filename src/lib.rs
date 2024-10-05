pub mod colors;

use colors::load_colors;

// Fonction pour obtenir les valeurs RGB d'une couleur par son nom
pub async fn get_color_by_name(color_name: &str) -> Option<(u8, u8, u8)> {
    let color_data = load_colors().await.unwrap_or_default();
    let color_name = color_name.to_lowercase();  // Convertir en minuscule pour uniformiser la recherche
    if let Some(color) = color_data.get(&color_name) {
        let r = color["r"].as_u64().unwrap_or(0) as u8;
        let g = color["g"].as_u64().unwrap_or(0) as u8;
        let b = color["b"].as_u64().unwrap_or(0) as u8;
        return Some((r, g, b));
    }
    None
}