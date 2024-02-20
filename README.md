Rust Art (imageproc)
====================

This is a simple imageproc project for people new to Rust or imageproc to start from.

The imageproc documentation is the best source of information.

imageproc home page: https://docs.rs/imageproc/latest/imageproc/

Building and Running
--------------------

We've tested this project on Windows 11, Ubuntu Linux 23.10, and MacOS Sonoma.

First, you need a Rust toolchain installed. rustup is an official tool for easily setting up a toolchain on various platforms. The rustup install process is guided and should setup external tooling too: https://rustup.rs/

Second, assuming you have git installed, clone this repository:
```
git clone https://github.com/BSJThomachot/rust_art_imageproc
```

If you don't have git and want a quick start, you can download the code as a zip file: https://github.com/BSJThomachot/rust_art_imageproc/archive/refs/heads/main.zip

Assuming that rustup installed a toolchain, you should now be able to run this command from the `rust_art_imageproc` directory:
```
cargo run
```

If everything went well it will display a window on screen!


Editing the code
----------------

It's recommended you install VSCode (https://code.visualstudio.com/) however various text editors support Rust.

Once VSCode is installed, open the `rust_art_imageproc` directory.

VSCode should prompt you to install the "Rust Analyzer" plugin, it's recommended you use this plugin (it provides syntax highlighting, code navigation, and various other nice to haves).

You should be able to make modifications to `src/main.rs`.

To test your modifications, save `main.rs`, then either execute `cargo run` from the terminal, or press Ctrl+Shift+B in VSCode.

