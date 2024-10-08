pub mod colors;
pub mod errors;

/// Retrieves the RGB values of a color by its name from the provided color map.
/// This function is used to avoid loading colors repeatedly for each request.
/// It returns a tuple `(r, g, b)` if the color is found, otherwise `None`.
///
/// # Arguments
/// * `color_map` - A reference to the preloaded map of colors.
/// * `color_name` - The name of the color to retrieve.
///
/// # Example
/// ```rust
/// use named_colors::colors::{get_color_by_name, load_colors};
///
/// fn main() {
///     let colors_map = load_colors().unwrap();  // Load the colors
///     let red_rgb = get_color_by_name(&colors_map, "red");
///     if let Some((r, g, b)) = red_rgb {
///         println!("RGB for red is ({}, {}, {})", r, g, b);
///     }
/// }
/// ```
pub fn get_color_by_name(
    color_map: &std::collections::HashMap<String, colors::Color>,
    color_name: &str,
) -> Option<(u8, u8, u8)> {
    let color_name = color_name.to_lowercase();
    if let Some(color) = color_map.get(&color_name) {
        return Some((color.r, color.g, color.b));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::colors::load_colors_from_file;
    use crate::colors::{add_color, get_color_by_name, load_colors};

    // Example JSON data for testing.
    const SAMPLE_JSON: &str = r#"
    {
        "red": { "r": 255, "g": 0, "b": 0 },
        "green": { "r": 0, "g": 255, "b": 0 },
        "blue": { "r": 0, "g": 0, "b": 255 }
    }
    "#;

    /// Tests if the function returns the correct RGB values for the color "red".
    #[test]
    fn test_get_color_by_name() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "red");
        assert_eq!(color, Some((255, 0, 0))); // Expected RGB for "red"
    }

    /// Tests if the function returns `None` for an invalid color name.
    #[test]
    fn test_get_color_by_invalid_name() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "invalid_color");
        assert_eq!(color, None);
    }

    /// Tests case insensitivity: color names with different cases should work.
    #[test]
    fn test_get_color_case_insensitive() {
        let color_map = load_colors().unwrap();
        let color = get_color_by_name(&color_map, "Red");
        assert_eq!(color, Some((255, 0, 0)));

        let color_lowercase = get_color_by_name(&color_map, "red");
        assert_eq!(color_lowercase, Some((255, 0, 0)));
    }

    /// Tests if the `load_colors` function works correctly.
    #[test]
    fn test_load_colors_success() {
        let result = load_colors();
        assert!(result.is_ok(), "Failed to load colors.");
    }

    /// Tests the addition of a new color.
    #[test]
    fn test_add_new_color() {
        let mut color_map = load_colors().unwrap();
        let result = add_color(&mut color_map, "sunset_orange", 255, 94, 77);
        assert!(result.is_ok(), "Failed to add new color.");
        let color = get_color_by_name(&color_map, "sunset_orange");
        assert_eq!(color, Some((255, 94, 77)));
    }

    /// Tests that adding a duplicate color results in an error.
    #[test]
    fn test_add_duplicate_color() {
        let mut color_map = load_colors().unwrap();
        let result = add_color(&mut color_map, "red", 255, 0, 0);
        assert!(result.is_err(), "Added a duplicate color.");
        let color = get_color_by_name(&color_map, "red");
        assert_eq!(color, Some((255, 0, 0)));
    }

    /// Tests the loading of colors from `SAMPLE_JSON`.
    #[test]
    fn test_load_colors_from_sample_json() {
        let color_map =
            load_colors_from_file(SAMPLE_JSON).expect("Failed to load colors from sample JSON");

        let red = get_color_by_name(&color_map, "red");
        assert_eq!(red, Some((255, 0, 0)), "Expected RGB for 'red'");

        let green = get_color_by_name(&color_map, "green");
        assert_eq!(green, Some((0, 255, 0)), "Expected RGB for 'green'");

        let blue = get_color_by_name(&color_map, "blue");
        assert_eq!(blue, Some((0, 0, 255)), "Expected RGB for 'blue'");
    }

    /// Tests if the function returns None for an invalid color with `SAMPLE_JSON`.
    #[test]
    fn test_get_invalid_color_from_sample_json() {
        let color_map =
            load_colors_from_file(SAMPLE_JSON).expect("Failed to load colors from sample JSON");
        let color = get_color_by_name(&color_map, "invalid_color");
        assert_eq!(color, None, "Expected None for an invalid color");
    }

    /// Tests the  `SAMPLE_JSON`.
    #[test]
    fn test_get_color_case_insensitive_from_sample_json() {
        let color_map =
            load_colors_from_file(SAMPLE_JSON).expect("Failed to load colors from sample JSON");

        let red_upper = get_color_by_name(&color_map, "Red");
        assert_eq!(red_upper, Some((255, 0, 0)), "Expected RGB for 'Red'");

        let red_lower = get_color_by_name(&color_map, "red");
        assert_eq!(red_lower, Some((255, 0, 0)), "Expected RGB for 'red'");
    }

    /// Test adding a new color with `SAMPLE_JSON`.
    #[test]
    fn test_add_new_color_with_sample_json() {
        let mut color_map =
            load_colors_from_file(SAMPLE_JSON).expect("Failed to load colors from sample JSON");

        let result = add_color(&mut color_map, "sunset_orange", 255, 94, 77);
        assert!(result.is_ok(), "Failed to add new color to color map");

        let color = get_color_by_name(&color_map, "sunset_orange");
        assert_eq!(
            color,
            Some((255, 94, 77)),
            "Expected RGB for 'sunset_orange'"
        );
    }

    /// test function that verifies the failure of adding an existing color with `SAMPLE_JSON`.
    #[test]
    fn test_add_duplicate_color_with_sample_json() {
        let mut color_map =
            load_colors_from_file(SAMPLE_JSON).expect("Failed to load colors from sample JSON");

        // Essayer d'ajouter une couleur qui existe déjà.
        let result = add_color(&mut color_map, "red", 255, 0, 0);
        assert!(
            result.is_err(),
            "Expected an error when adding a duplicate color"
        );

        let color = get_color_by_name(&color_map, "red");
        assert_eq!(color, Some((255, 0, 0)), "Expected existing RGB for 'red'");
    }
}
