use reqwest::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;

const COLORS_JSON_URL: &str = "https://raw.githubusercontent.com/teamflp/named_colors/master/named_colors.json";
const CACHE_FILE_PATH: &str = "cache/named_colors.json"; // Path for the local cache file
const CACHE_EXPIRATION_DURATION: Duration = Duration::from_secs(60 * 60 * 24); // Cache expiration after 24 hours

/// Checks if the cache is still valid based on the file's last modification time.
/// If the cache was modified less than 24 hours ago, it is considered valid.
/// 
/// # Returns:
/// * `true` if the cache is valid.
/// * `false` if the cache is expired or there is an error accessing the file.
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

/// ```rust
/// use named_colors::colors::load_colors;
/// use tokio;  // Necessary to run async
///
/// #[tokio::main]  // Create an async function for testing
/// async fn main() {
///     let colors = load_colors().await.unwrap();
///     if let Some(color) = colors.get("red") {
///         println!("RGB for red: {:?}", color);
///     }
/// }
/// ```

pub async fn load_colors() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    // If the cache file exists and is valid, use it.
    if Path::new(CACHE_FILE_PATH).exists() && is_cache_valid() {
        // Load the cache file
        let cache_content = fs::read_to_string(CACHE_FILE_PATH)?;
        let colors: HashMap<String, Value> = serde_json::from_str(&cache_content)
            .map_err(|err| Box::new(err) as Box<dyn Error>)?;
        return Ok(colors);
    } else {
        // If the cache is absent or expired, download the JSON file.
        let response = get(COLORS_JSON_URL).await?.text().await?;
        
        // Save the JSON file to the cache for future use.
        fs::create_dir_all("cache")?;  // Create the "cache" folder if it doesn't exist
        fs::write(CACHE_FILE_PATH, &response)?;  // Write the downloaded content to the cache
        
        // Load the downloaded JSON file
        let colors: HashMap<String, Value> = serde_json::from_str(&response)
            .map_err(|err| Box::new(err) as Box<dyn Error>)?;
        return Ok(colors);
    }
}