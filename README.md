# Named Colors : version 0.1.1

`named_colors` is a Rust library that provides named colors with their RGB values. The library allows you to retrieve RGB values by color name or use these values in different applications.

## Features

- Retrieve RGB values by color name.
- Supports loading colors from a pre-defined JSON file or a user-provided JSON string.
- Handles a wide range of named colors (e.g., `red`, `blue`, `green`, etc.).
- Error handling with custom error types for smooth integration.

## Installation

You can include `named_colors` in your project by adding the following line to your `Cargo.toml` file.

### From [crates.io](http://Crates.io) (Version Specified)

If the library is published on [Crates.io](https://crates.io/), you can specify a version:

```toml
[dependencies]
named_colors = "0.1.0"
```

Or you can specify a more specific version like this:

```toml
[dependencies]
named_colors = { version = "0.1.0" }
```

### Example

Here is an example of how to use the library:

```rust
extern crate named_colors;
use named_colors::get_color_by_name;

fn main() {
    let color_name = "navy";
    
    match get_color_by_name(color_name) {
        Some((r, g, b)) => {
            println!("RGB for {}: ({}, {}, {})", color_name, r, g, b);
        }
        None => {
            println!("Color '{}' not found.", color_name);
        }
    }
}
```

This will output:

```shell
RGB for navy: (0, 0, 128)
```

## Synchronous Example: Retrieve RGB Values by Color Name

The library can be used to retrieve RGB values synchronously:

```rust
extern crate named_colors;
use named_colors::{get_color_by_name, load_colors};

fn main() {
    // Load the color map once.
    let color_map = load_colors().unwrap();

    // Get the RGB values for a color by its name.
    let color_name = "navy";

    match get_color_by_name(&color_map, color_name) {
        Some((r, g, b)) => {
            println!("RGB for {}: ({}, {}, {})", color_name, r, g, b);
        }
        None => {
            println!("Color '{}' not found.", color_name);
        }
    }
}
```

### Asynchronous Example: Retrieve RGB Values

If you need to work asynchronously, for instance when downloading the color data from a remote source, here's how to use the library:

```rust
use named_colors::get_color_by_name;
use tokio;  // Necessary to run async

#[tokio::main]
async fn main() {
    let color_name = "red";
    let red_rgb = get_color_by_name(color_name).await;

    if let Some((r, g, b)) = red_rgb {
        println!("RGB for {}: ({}, {}, {})", color_name, r, g, b);
    }
}
```

This will output:

```shell
RGB for red is (255, 0, 0)
```

### Handling Errors

The `load_colors` function returns a `Result` type with a custom error (`NamedColorsError`), allowing you to handle potential parsing errors gracefully:

```rust
use named_colors::colors::load_colors;

fn main() {
    match load_colors() {
        Ok(colors_map) => {
            println!("Colors loaded successfully.");
        }
        Err(err) => {
            eprintln!("Failed to load colors: {}", err);
        }
    }
}
```

### Available Colors

The library fetches color data from a JSON file hosted at a remote URL. It includes common colors such as:

- Red
- Green
- Blue
- Navy
- Magenta
- And many more...

The colors data is stored in a JSON file included in the library, ensuring quick access to color values at runtime.

### Retrieving Colors by Name

You can use the `get_color_by_name` function to retrieve RGB values of a color by its name:

```rust
use named_colors::colors::{get_color_by_name, load_colors};

let color_map = load_colors().unwrap();
let rgb = get_color_by_name(&color_map, "chartreuse");
if let Some((r, g, b)) = rgb {
    println!("RGB: {}, {}, {}", r, g, b);
} else {
    println!("Color not found");
}
```

### Case Insensitivity

The `get_color_by_name` function is case-insensitive, meaning that "Red" and "red" will yield the same result:

```rust
let color_map = load_colors().unwrap();
assert_eq!(get_color_by_name(&color_map, "Red"), Some((255, 0, 0)));
assert_eq!(get_color_by_name(&color_map, "red"), Some((255, 0, 0)));
```

## Contributing

Feel free to open issues or submit pull requests if you'd like to contribute to this project.

1. Fork the repository.
2. Create a new branch for your feature.
3. Make your changes and test thoroughly.
4. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.