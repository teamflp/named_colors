pub mod colors;

use colors::load_colors;


/// ```rust
/// use named_colors::get_color_by_name;
/// use tokio;  // Nécessary to run async
///
/// #[tokio::main]  // Create an async function for testing
/// async fn main() {
///     let red_rgb = get_color_by_name("red").await;
///     if let Some((r, g, b)) = red_rgb {
///         println!("RGB for red is ({}, {}, {})", r, g, b);
///     }
/// }
/// ```

pub async fn get_color_by_name(color_name: &str) -> Option<(u8, u8, u8)> {
    // Load the colors from the cache/download function
    let color_data = load_colors().await.unwrap_or_default();
    
    // Convert the color name to lowercase to standardize the search
    let color_name = color_name.to_lowercase();
    
    if let Some(color) = color_data.get(&color_name) {
        // Extract RGB values from the JSON
        let r = color["r"].as_u64().unwrap_or(0) as u8;
        let g = color["g"].as_u64().unwrap_or(0) as u8;
        let b = color["b"].as_u64().unwrap_or(0) as u8;
        return Some((r, g, b));
    }
    None
}


/// Unit tests for the `get_color_by_name` function.
/// These tests check various conditions to ensure that the function works as expected.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the function returns the correct RGB values for the color "red".
    #[tokio::test]
    async fn test_get_color_by_name() {
        let color = get_color_by_name("red").await;
        assert_eq!(color, Some((255, 0, 0)));  // Expected RGB for "red"
    }

    /// Tests if the function returns `None` for an invalid color.
    #[tokio::test]
    async fn test_get_color_by_invalid_name() {
        let color = get_color_by_name("invalid_color").await;
        assert_eq!(color, None);  // No valid color found
    }

    /// Tests case insensitivity: color names with different cases should work.
    #[tokio::test]
    async fn test_get_color_case_insensitive() {
        let color = get_color_by_name("Red").await;
        assert_eq!(color, Some((255, 0, 0)));  // The function should be case-insensitive
        
        let color_lowercase = get_color_by_name("red").await;
        assert_eq!(color_lowercase, Some((255, 0, 0)));  // Check with lowercase name
    }

    /// Tests if the `load_colors` function works correctly.
    /// In this test, we simply check that loading the colors is successful.
    #[tokio::test]
    async fn test_load_colors_failure() {
        let result = load_colors().await;
        assert!(result.is_ok(), "Failed to load colors.");  // Ensure colors load correctly
    }
}