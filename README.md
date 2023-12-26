# material3-optimzer

The Material3 Theme Builder Parser for [Jetpack Compose Material3 Builder](https://m3.material.io/theme-builder#/custom) is a utility that automates the process of generating theme code based on the Material3 Builder output. It also allows you to inline the colors from the color scheme directly into the generated theme code, providing a seamless integration between the builder and the color scheme, eliminating generated boilerplate.

The package contains two components:

1. **Library:** A Rust library that provides the core functionality for parsing the Material3 Builder output and generating theme code.
2. **CLI:** A command-line interface (CLI) tool that wraps the library and provides a user-friendly interface for using the parser.

## About

Material Theme Builder is a new tool that helps you visualize Material You’s dynamic color and create a custom Material Design 3 theme. With built-in code export, it’s easy to migrate to Material's new color system and take advantage of dynamic color.

Material Design can be customized to represent a variety of stylistic choices. A theme is a set of style choices that define the visual appearance of a product. With Material Design 3, we are introducing design tokens—small, reusable design decisions that reflect the system’s visual style. By building with tokens instead of static values, design and code share a source of truth.

For more information about the Material 3 Builder and its capabilities, please refer to the [Material 3 Builder Blog](https://material.io/blog/material-theme-builder).

### Usage

**To use it as a library**, add the library to your project's `Cargo.toml` file:
```rust
[dependencies]
material3_optimizer = "0.1.1"
```

**To use the CLI program**, follow these steps:

1. Run the following command in your terminal:

```bash
    cargo install material3_optimizer_cli
```
2. Add the executable to your system's PATH environment variable. This will allow you to run the CLI from any directory.

To see the available arguments and usage information, enter the following command:
```bash
    material3_optimizer_cli --help
```

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
