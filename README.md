# material3-optimzer

The Material3 Theme Builder Parser for [Jetpack Compose Material3 Builder](https://m3.material.io/theme-builder#/custom) is a utility that automates the process of generating theme code based on the Material3 Builder output. It also allows you to inline the colors from the color scheme directly into the generated theme code, providing a seamless integration between the builder and the color scheme.

## About

Material Theme Builder is a new tool that helps you visualize Material You’s dynamic color and create a custom Material Design 3 theme. With built-in code export, it’s easy to migrate to Material's new color system and take advantage of dynamic color.

Material Design can be customized to represent a variety of stylistic choices. A theme is a set of style choices that define the visual appearance of a product. With Material Design 3, we are introducing design tokens—small, reusable design decisions that reflect the system’s visual style. By building with tokens instead of static values, design and code share a source of truth.

For more information about the Material 3 Builder and its capabilities, please refer to the [Material 3 Builder Blog](https://material.io/blog/material-theme-builder).


### Usage

To use the CLI program, follow these steps:

1. Compile the Rust code using the Rust compiler. Run the following command in your terminal:

```bash
    cargo build
```
2. After compiling the Rust code, you will have an executable file (material3_optimier.exe on Windows). Navigate to the directory where the compiled executable is located. To see the available arguments and usage information, enter the following command:
```bash
    ../material3_optimier.exe --help
```

### Error Handling

* If either the input directory or the output directory is not provided, the program will print an error message indicating that both input and output directories are required.
* If an error occurs while trying to create an`OptimizeResult` object from the input directory, the specific error message will be printed.

## Contributing

Contributions are welcome! If you find any issues or have suggestions for improvement, please open an issue or submit a pull request.
