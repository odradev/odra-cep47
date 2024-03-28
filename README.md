# CEP-47 implementation using using Odra Framework

This implementation is based on the original [CEP-47](https://github.com/casper-ecosystem/casper-nft-cep47) code.

## Usage
It's required to install 
[cargo-odra](https://github.com/odradev/cargo-odra) first.

### Build

```bash
cargo odra build
```
The result files will be placed in `${project-root}/wasm` directory.

### Tests
To run tests using OdraVM on your local machine, execute:

```bash
cargo odra test
```

To test actual wasm files against a CasperVM backend, 
you need to specify the backend passing -b argument to `cargo-odra`:

```bash
cargo odra test -b casper
```

To run livenet test suite, first set up the `.env` file, build wasm files and run:

```bash
cargo run --bin livenet --features livenet
```