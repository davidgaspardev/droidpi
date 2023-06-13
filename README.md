# DroiDPI

DroiDPI is a command-line tool for resizing images to different screen densities commonly used in mobile application development. It simplifies the process of generating multiple sizes of icons for Flutter and native Android projects, helping to improve performance on various devices.

## Features

- Resizes images to five different densities: mdpi, hdpi, xhdpi, xxhdpi, and xxxhdpi.
- Supports both Flutter and native Android projects.
- Automates the creation of directories and resizing of images for each density.
- Uses the image crate for image processing in Rust.

## Flutter pattern directory to images

```bash
.../my_icon.png       (mdpi baseline)
.../1.5x/my_icon.png  (hdpi)
.../2.0x/my_icon.png  (xhdpi)
.../3.0x/my_icon.png  (xxhdpi)
.../4.0x/my_icon.png  (xxxhdpi)
```

## Usage

To resize an image using droiDPI, use the following command:

```bash
droiDPI --image <image_path> --flutter <flutter_path> --name <image_name>
```

- `<image_path>`: The path to the input image file.
- `<flutter_path>`: The base directory where the resized images will be stored for Flutter. The different densities will be created as subdirectories within this base directory.
- `<image_name>`: The desired name for the resized images. The resized images will be saved with this name and the appropriate density suffix.

## What do I want to do with this ?

I'm a mobile developer, and I always have to deal with performance on more humble devices, to collaborate with performance I need to resize my icons in five different sizes (mdpi, hdpi, xhdpi, xxhdpi and xxxhdpi).

It's always been a pain in the ass, but it's important, that's why I want to create this!

For always enjoying low-level development, came the idea of combining business with pleasure. Develop a binary that optimizes my work.