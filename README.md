# DroiDPI

DroiDPI is a command-line tool for resizing images to different screen densities commonly used in mobile application development. It simplifies the process of generating multiple sizes of icons for Flutter and native Android projects, helping to improve performance on various devices.

## Installation

Install DroiDPI using Cargo:

```bash
cargo install droidpi
```

Or install a specific version:

```bash
cargo install droidpi@0.1.0
```

## Features

- Resizes images to five different densities: mdpi, hdpi, xhdpi, xxhdpi, and xxxhdpi.
- Supports both Flutter and native Android projects.
- Automates the creation of directories and resizing of images for each density.
- Uses the image crate for image processing in Rust.

## Pattern directory to images

### For Flutter

Create `my_icon.png` for Flutter:

```bash
.../my_icon.png       (mdpi baseline)
.../1.5x/my_icon.png  (hdpi)
.../2.0x/my_icon.png  (xhdpi)
.../3.0x/my_icon.png  (xxhdpi)
.../4.0x/my_icon.png  (xxxhdpi)
```

### For Android

Create `my_icon.png` for Android (default: **mipmap** directories):

```bash
.../mipmap-mdpi/my_icon.png
.../mipmap-hdpi/my_icon.png
.../mipmap-xhdpi/my_icon.png
.../mipmap-xxhdpi/my_icon.png
.../mipmap-xxxhdpi/my_icon.png
```

**With `--use-drawable` flag:**  
Creates images in **drawable** directories instead:

```bash
.../drawable-mdpi/my_icon.png
.../drawable-hdpi/my_icon.png
.../drawable-xhdpi/my_icon.png
.../drawable-xxhdpi/my_icon.png
.../drawable-xxxhdpi/my_icon.png
```

## Usage

To resize an image using DroiDPI, use the following command:

```bash
droidpi --src <image_path> --outdir <directory_path> --name <image_name> --platform <flutter|android> [--use-drawable]
```

- `--src <image_path>`: The path to the input image file (.png, .jpg, or .jpeg).
- `--outdir <directory_path>`: The base directory where the resized images will be stored. The different densities will be created as subdirectories within this base directory, according to the selected platform.
- `--name <image_name>`: The desired name for the resized images. The resized images will be saved with this name.
- `--platform <platform>`: The target platform for which the images will be generated. Supported values: `flutter` or `android`.
- `--use-drawable`: *(Android only, optional)* If present, output images to `drawable-*dpi` directories instead of `mipmap-*dpi`.

### Examples

```bash
# For Flutter projects
droidpi --src logo.png --outdir ./assets --name logo --platform flutter

# For native Android projects (default: mipmap)
droidpi --src icon.png --outdir ./res --name ic_launcher --platform android

# For native Android projects (using drawable directories)
droidpi --src icon.png --outdir ./res --name ic_launcher --platform android --use-drawable
```

## What do I want to do with this?

I'm a mobile developer, and I always have to deal with performance on more humble devices. To collaborate with performance, I need to resize my icons in five different sizes (mdpi, hdpi, xhdpi, xxhdpi, and xxxhdpi).

It's always been a pain, but it's important, and that's why I want to create this!

For always enjoying low-level development, came the idea of combining business with pleasure: develop a binary that optimizes my work.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/davidgaspardev/droidpi.git
   cd droidpi
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Run the CLI locally:
   ```bash
   cargo run -- --src <image_path> --outdir <directory_path> --name <image_name> --platform <platform>
   ```

### Releasing

This project uses automated releases via GitHub Actions. See [`.github/RELEASE.md`](.github/RELEASE.md) for detailed instructions on publishing new versions.

## License

This project is licensed under the MIT License - see the [LICENSE-MIT](LICENSE-MIT) file for details.

## Author

David Gaspar - [davidgaspar.dev@gmail.com](mailto:davidgaspar.dev@gmail.com)

## Links

- [crates.io](https://crates.io/crates/droidpi)
- [Repository](https://github.com/davidgaspardev/droidpi)
- [Issues](https://github.com/davidgaspardev/droidpi/issues)
