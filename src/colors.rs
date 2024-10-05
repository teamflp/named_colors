use reqwest::blocking::get;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

const COLORS_JSON_URL: &str = "https://github.com/teamflp/colorsjson/blob/master/named_colors.json"; // URL de ton fichier GitHub

pub fn load_colors() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let response = get(COLORS_JSON_URL)?.text()?;
    let colors: HashMap<String, Value> = serde_json::from_str(&response)?;
    Ok(colors)
}
