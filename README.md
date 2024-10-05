# named_colors

`named_colors` is a Rust library that provides named colors with their RGB values. The library allows you to retrieve RGB values by color name or use these values in different applications.

## Features

- Retrieve RGB values by color name.
- Automatically fetches a JSON file containing color data from a remote URL.
- Supports a wide range of named colors (e.g., `red`, `blue`, `green`, etc.).

## Installation

You can include `named_colors` in your project by adding the following line to your `Cargo.toml` file.

### From GitHub

```toml
[dependencies]
named_colors = { git = "https://github.com/teamflp/named_colors" }
```

### From [Crates.io](http://Crates.io) (Version Specified)

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

## Usage

### Retrieving Colors by Name

You can use the `get_color_by_name` function to retrieve RGB values of a color by its name:

```rust
use named_colors::get_color_by_name;

let rgb = get_color_by_name("chartreuse");
if let Some((r, g, b)) = rgb {
    println!("RGB: {}, {}, {}", r, g, b);
} else {
    println!("Color not found");
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

The full list of colors is provided in the JSON file located [here](https://raw.githubusercontent.com/teamflp/named_colors/master/named_colors.json)

### Handling Errors

If the color name is invalid, the function will return `None`:

```rust
let invalid_color = get_color_by_name("invalid_color");
assert!(invalid_color.is_none());
```

## Contributing

Feel free to open issues or submit pull requests if you'd like to contribute to this project.

1. Fork the repository.
2. Create a new branch for your feature.
3. Make your changes and test thoroughly.
4. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.