# nxcloudnotes - a CLI text dumping tool for NextCloud

**This project was purely done as a learning exercise for Rust, and so is not production ready. Crates used have been purposely kept to a minimum for this reason.**

## Installing

The binaries for each release can be downloaded [here](https://github.com/CallumHoughton18/nextcloud-notes-rs/releases), currently only MacOS and linux binaries are generated on each release.

### MacOS

1. Download the binary ending in '-macos' for which version you wish to install.
2. Rename the binary to 'nxcloudnotes', then in the folder the binary was downloaded run: `chmod 700 ./nxcloudnotes`
3. Move the binary to somewhere that is currently in your $PATH variable, as specified in your .bashrc or .zshrc profile file.

### Linux

*SHOULD* follow a similar process to the MacOS install (but download the binary ending in -linux, obviously). I already spent longer than I really wanted to on this so I haven't actually tested it on a distro ¯\_(ツ)_/¯

### Windows

While cross-compilation with Rust does also support binary generation for Windows the OpenSSL dependency was making this more effort than it's worth. To run this on Windows you'll first have to have OpenSSL installed on your machine, then you will need to pull and compile the code yourself (sorry).
