use std::io::Result;

extern crate cbindgen;

use std::env;

fn main() -> Result<()> {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        // .with_config("cbindgen.toml")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/rusty_cardano_wallet.h");

    // cxx_build::bridge("src/lib.rs")
    //     .file("src/blobstore.cc")
    //     .flag_if_supported("-std=c++14")
    //     .compile("rusty_cardano_wallet");

    Ok(())
}
