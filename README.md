# pachy-decrypt
`pachy-decrypt` is a small asynchronous multithreaded script used to decrypt .pachy files found during the Forensic course.

![GitHub top language](https://img.shields.io/github/languages/top/QuaeroEtTego/file-adder)
![mit-badge](https://img.shields.io/badge/license-MIT-blue.svg)

## Usage
1. Clone the repository
2. Remove the extension `.example` present in files on `./data`
3. Complete the file `./data/key-iv.json` (make sure to keep the same JSON structure)
4. Place all `.pachy` files in `./data/file`
5. Build and run `cargo run --release`

## Notes
Make sure to have OpenSSL installed on your operating system.\
The script has been developed and tested on a Linux architecture.

## Supported Rust Versions
Pachy-decrypt supports a MSRV (minimum supported rust version) of Rust 1.73.
Pachy-decrypt version is not guaranteed to build on Rust versions earlier than the minimum supported version.

## License
This project is licensed under the [MIT](LICENSE.md) license.