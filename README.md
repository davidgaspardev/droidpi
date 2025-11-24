# DroiDPI

DroiDPI is a command-line tool for resizing images to different screen densities commonly used in mobile application development. It simplifies the process of generating multiple sizes of icons for Flutter and native Android projects, helping to improve performance on various devices.

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

Create `my_icon.png` for Android (with mipmap directories by default):

```bash
.../mipmap-mdpi/my_icon.png
.../mipmap-hdpi/my_icon.png
.../mipmap-xhdpi/my_icon.png
.../mipmap-xxhdpi/my_icon.png
.../mipmap-xxxhdpi/my_icon.png
```

Or with `--use-drawable` flag (uses drawable directories):

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
droidpi --src <image_path> --outDir <directory_path> --name <image_name> --platform <flutter|android>
```

### Options

- `--src <image_path>`: Path to the input image file (.png, .jpg, or .jpeg)
- `--outDir <directory_path>`: Base directory where resized images will be stored
- `--name <image_name>`: Desired name for the resized images
- `--platform <platform>`: Target platform (flutter or android)
  - `flutter`: Flutter platform - generates images in 1.5x, 2.0x, 3.0x, 4.0x directories
  - `android`: Android platform - generates images in mipmap-* or drawable-* directories
- `--use-drawable`: Use drawable directories instead of mipmap for Android (only applies to android platform)
- `--help`: Display help message
- `--version`: Show version information

### Examples

```bash
# For Flutter projects
droidpi --src logo.png --outDir ./assets --name logo --platform flutter

# For native Android projects (using mipmap)
droidpi --src icon.png --outDir ./res --name ic_launcher --platform android

# For native Android projects (using drawable)
droidpi --src icon.png --outDir ./res --name ic_logo --platform android --use-drawable

# Show help
droidpi --help

# Show version
droidpi --version
```

## What do I want to do with this?

I'm a mobile developer, and I always have to deal with performance on more humble devices. To collaborate with performance, I need to resize my icons in five different sizes (mdpi, hdpi, xhdpi, xxhdpi, and xxxhdpi).

It's always been a pain, but it's important, and that's why I want to create this!

For always enjoying low-level development, came the idea of combining business with pleasure: develop a binary that optimizes my work.
