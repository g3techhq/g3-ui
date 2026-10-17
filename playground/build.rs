//! Extract component props on the host; the browser receives static metadata.
mod props_model;

use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let components = root.join("../src/components");
    println!("cargo:rerun-if-changed={}", components.display());
    println!("cargo:rerun-if-changed=props_model.rs");
    let mut files = fs::read_dir(components)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .collect::<Vec<_>>();
    files.sort();
    let theme = root.join("../src/theme.rs");
    println!("cargo:rerun-if-changed={}", theme.display());
    files.push(theme);
    let sources = files
        .iter()
        .map(|path| {
            let source = fs::read_to_string(path).unwrap();
            syn::parse_file(&source).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        })
        .collect::<Vec<_>>();
    let model = props_model::components(&sources);
    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("component_props.rs");
    fs::write(output, props_model::generate(&model)).unwrap();
}
