use crate::errors::NamedColorsError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a color using RGB values.
#[derive(Deserialize, Serialize, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Loads predefined colors from the library's JSON data.
/// This function is intended to load a static set of colors provided by the library.
///
/// # Returns
/// * `Result<HashMap<String, Color>, NamedColorsError>` - A `HashMap` mapping color names to their RGB values,
/// or a `NamedColorsError` in case of failure.
///
/// # Example
/// ```rust
/// use named_colors::colors::{get_color_by_name, load_colors};
/// let color_map = load_colors().expect("Failed to load library colors");
/// if let Some((r, g, b)) = get_color_by_name(&color_map, "blue") {
///     println!("Blue RGB: {}, {}, {}", r, g, b);
/// }
/// ```
pub fn load_colors() -> Result<HashMap<String, Color>, NamedColorsError> {
    let data = include_str!("../assets/named_colors.json"); // Load the library's predefined colors.
    let color_map: HashMap<String, Color> =
        serde_json::from_str(data).map_err(NamedColorsError::ParseError)?;
    Ok(color_map)
}

/// Loads colors from a JSON string provided by the user.
/// This allows the user to manage the storage method (e.g., file, database).
///
/// # Arguments
/// * `json_data` - The JSON string containing the colors.
///
/// # Returns
/// * `Result<HashMap<String, Color>, NamedColorsError>` - A `HashMap` mapping color names to their RGB values,
/// or a `NamedColorsError` in case of failure.
///
/// # Example
/// ```rust
/// use named_colors::colors::{get_color_by_name, load_colors_from_file};
/// let json_data = r#"{"blue": {"r": 0, "g": 0, "b": 255}}"#;
/// let color_map = load_colors_from_file(json_data).expect("Failed to load user-defined colors");
/// if let Some((r, g, b)) = get_color_by_name(&color_map, "blue") {
///     println!("Blue RGB: {}, {}, {}", r, g, b);
/// }
/// ```
pub fn load_colors_from_file(json_data: &str) -> Result<HashMap<String, Color>, NamedColorsError> {
    let color_map: HashMap<String, Color> =
        serde_json::from_str(json_data).map_err(NamedColorsError::ParseError)?;
    Ok(color_map)
}

/// Adds a new color to the color map.
/// The user is responsible for saving the data through their chosen method.
///
/// # Arguments
/// * `color_map` - The existing color map.
/// * `color_name` - The name of the new color to add.
/// * `r`, `g`, `b` - RGB values of the new color.
///
/// # Returns
/// * `Result<(), NamedColorsError>` - `Ok(())` if the addition is successful, `Err` otherwise.
///
/// # Validation Messages
/// * Returns `Err` if the color name already exists in the map:
///   `"The color 'color_name' already exists."`
///
/// # Example
/// ```rust
/// use named_colors::colors::{add_color, load_colors};
/// let mut color_map = load_colors().expect("Failed to load library colors");
/// add_color(&mut color_map, "sky_blue", 135, 206, 235)
///     .expect("Failed to add color");
/// ```
pub fn add_color(
    color_map: &mut HashMap<String, Color>,
    color_name: &str,
    r: u8,
    g: u8,
    b: u8,
) -> Result<(), NamedColorsError> {
    let color_name = color_name.to_lowercase();
    if color_map.contains_key(&color_name) {
        return Err(NamedColorsError::Custom(format!(
            "The color '{}' already exists.",
            color_name
        )));
    }

    color_map.insert(color_name.clone(), Color { r, g, b });
    Ok(())
}

/// Retrieves the RGB values of a color by its name from the provided color map.
///
/// # Arguments
/// * `color_map` - A reference to the map of colors.
/// * `color_name` - The name of the color to retrieve.
///
/// # Returns
/// * `Option<(u8, u8, u8)>` - The RGB values if the color is found, or `None` if not.
///
/// # Example
/// ```rust
/// use named_colors::colors::{get_color_by_name, load_colors};
/// let color_map = load_colors().expect("Failed to load library colors");
///
/// if let Some((r, g, b)) = get_color_by_name(&color_map, "emerald") {
///     println!("Emerald RGB: {}, {}, {}", r, g, b);
/// } else {
///     println!("Color not found");
/// }
/// ```
pub fn get_color_by_name(
    color_map: &HashMap<String, Color>,
    color_name: &str,
) -> Option<(u8, u8, u8)> {
    let color_name = color_name.to_lowercase();
    color_map
        .get(&color_name)
        .map(|color| (color.r, color.g, color.b))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_color_by_name() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "red");
        assert_eq!(color, Some((255, 0, 0))); // Expected RGB for "red"
    }

    #[test]
    fn test_get_color_by_invalid_name() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "invalid_color");
        assert_eq!(color, None); // No valid color found
    }

    #[test]
    fn test_get_color_case_insensitive() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "Red");
        assert_eq!(color, Some((255, 0, 0))); // Function should be case insensitive

        let color_lowercase = get_color_by_name(&color_map, "red");
        assert_eq!(color_lowercase, Some((255, 0, 0))); // Check with a lowercase name
    }

    #[test]
    fn test_add_new_color() {
        let mut color_map = load_colors().unwrap();
        let result = add_color(&mut color_map, "sunset_orange", 255, 94, 77);
        assert!(result.is_ok(), "Failed to add the new color.");
        let color = get_color_by_name(&color_map, "sunset_orange");
        assert_eq!(color, Some((255, 94, 77))); // Verify the added color
    }

    #[test]
    fn test_load_colors_from_file() {
        let json_data = r#"{"blue": {"r": 0, "g": 0, "b": 255}}"#;
        let color_map = load_colors_from_file(json_data).unwrap();
        let color = get_color_by_name(&color_map, "blue");
        assert_eq!(color, Some((0, 0, 255))); // Verify loading from user-defined JSON
    }

    #[test]
    fn test_add_duplicate_color() {
        let mut color_map = load_colors().unwrap();
        let result = add_color(&mut color_map, "blue", 0, 0, 255);
        assert!(result.is_ok(), "Failed to add the new color.");

        let result = add_color(&mut color_map, "blue", 255, 0, 0);
        assert!(result.is_err(), "Failed to detect duplicate color.");

        assert_eq!(
            result.err().unwrap().to_string(),
            "The color 'blue' already exists."
        );
    }
}
