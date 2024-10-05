use reqwest::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;

const COLORS_JSON_URL: &str = "https://raw.githubusercontent.com/teamflp/named_colors/master/named_colors.json";
const CACHE_FILE_PATH: &str = "cache/named_colors.json"; // Chemin pour le fichier cache local
const CACHE_EXPIRATION_DURATION: Duration = Duration::from_secs(60 * 60 * 24); // Expiration du cache après 24 heures

// Vérifie si le cache est encore valide (non expiré)
fn is_cache_valid() -> bool {
    if let Ok(metadata) = fs::metadata(CACHE_FILE_PATH) {
        if let Ok(modified_time) = metadata.modified() {
            if let Ok(duration_since_modified) = modified_time.elapsed() {
                return duration_since_modified < CACHE_EXPIRATION_DURATION;
            }
        }
    }
    false
}

// Télécharge et charge les couleurs à partir du cache ou du fichier en ligne
pub async fn load_colors() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // Si le fichier de cache existe et est valide, l'utiliser
    if Path::new(CACHE_FILE_PATH).exists() && is_cache_valid() {
        // Charger le fichier cache
        let cache_content = fs::read_to_string(CACHE_FILE_PATH)?;
        let colors: HashMap<String, Value> = serde_json::from_str(&cache_content)
            .map_err(|err| Box::new(err) as Box<dyn Error>)?;
        return Ok(colors);
    } else {
        // Si le cache est absent ou expiré, télécharger le fichier JSON
        let response = get(COLORS_JSON_URL).await?.text().await?;
        
        // Sauvegarder le fichier JSON dans le cache
        fs::create_dir_all("cache")?;  // Créer le dossier "cache" s'il n'existe pas
        fs::write(CACHE_FILE_PATH, &response)?;  // Écrire dans le fichier cache
        
        // Charger le fichier JSON téléchargé
        let colors: HashMap<String, Value> = serde_json::from_str(&response)
            .map_err(|err| Box::new(err) as Box<dyn Error>)?;
        return Ok(colors);
    }
}