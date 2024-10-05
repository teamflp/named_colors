use reqwest::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

const COLORS_JSON_URL: &str = "https://raw.githubusercontent.com/teamflp/named_colors/refs/heads/master/named_colors.json";  // Utilise le lien vers le fichier brut


// Cette fonction télécharge et charge les couleurs à partir du fichier JSON hébergé.
pub async fn load_colors() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // Requête HTTP pour récupérer le fichier JSON brut
    let response = get(COLORS_JSON_URL).await?.text().await?;
    
    // Parse le contenu JSON et le mappe dans un HashMap
    let colors: HashMap<String, Value> = serde_json::from_str(&response)
        .map_err(|err| Box::new(err) as Box<dyn Error>)?;
    
    Ok(colors)
}

