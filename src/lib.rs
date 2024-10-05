pub mod colors;

use colors::load_colors;

// Fonction pour obtenir les valeurs RGB d'une couleur par son nom

pub async fn get_color_by_name(color_name: &str) -> Option<(u8, u8, u8)> {
    // Charger les couleurs depuis la fonction de cache/téléchargement
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


// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_color_by_name() {
        // Tester une couleur valide
        let color = get_color_by_name("red").await;
        assert_eq!(color, Some((255, 0, 0)));
    }

    #[tokio::test]
    async fn test_get_color_by_invalid_name() {
        // Tester une couleur invalide
        let color = get_color_by_name("invalid_color").await;
        assert_eq!(color, None);
    }

    #[tokio::test]
    async fn test_get_color_case_insensitive() {
        // Tester la sensibilité à la casse
        let color = get_color_by_name("Red").await;
        assert_eq!(color, Some((255, 0, 0)));
        
        let color_lowercase = get_color_by_name("red").await;
        assert_eq!(color_lowercase, Some((255, 0, 0)));
    }

    #[tokio::test]
    async fn test_load_colors_failure() {
        // Simuler un cas où le chargement des couleurs échoue (si possible)
        // Exemple fictif, pourrait nécessiter des ajustements dans load_colors()
        let result = load_colors().await;
        assert!(result.is_ok(), "Échec du chargement des couleurs.");
    }
}
