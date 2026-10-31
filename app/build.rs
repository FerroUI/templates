//! Compiles the XAML documents of the crate to Rust with `ferroui-build`.
//!
//! Every `*.xaml` file below `src/` is a document. A document with `x:Class` is compiled into
//! the module `compiled_<file name in snake case>` of the crate (`MainWindow.xaml` is
//! `crate::compiled_main_window`), whose `populate` function the constructor of the class calls:
//! the `InitializeComponent()` of .NET. A document without a class (a resource dictionary, a
//! set of styles) is compiled into a module of its own as well, and is reached by its URI
//! (`StyleInclude`, `ResourceInclude`) through the loader table of the crate, which
//! `compiled_markup::register()` registers.
//!
//! The documents are compiled against the type models: the scan of the sources of this crate
//! and the models the FerroUI crates export, so the build script links no crate for the types
//! the documents name.

use std::fs;
use std::path::Path;

use ferroui_build::{Build, TypeSystem, XamlGroup};

fn main() {
    let mut documents = Vec::new();
    collect(Path::new("src"), Path::new("src"), &mut documents);
    documents.sort();

    let all: Vec<(&str, &str)> = documents.iter().map(|(name, text)| (name.as_str(), text.as_str())).collect();
    let mut modules: Vec<(String, &str)> = Vec::new();
    let mut build = Build::from_env().type_system(TypeSystem::Model).default_compile_bindings(true);
    for (name, text) in &all {
        let module = module_name(name);
        if let Some((_, other)) = modules.iter().find(|(existing, _)| *existing == module) {
            panic!("src/{name} and src/{other} would both be the module {module}: rename one of the files");
        }
        modules.push((module.clone(), name));
        build = build.input(&format!("src/{name}"));
        let group = XamlGroup::new(&module);
        // The group of a class is its document with the documents of the crate it includes.
        let group = if text.contains("x:Class=") { group.documents(&all).class_document(name) } else { group.documents(&[(name, text)]) };
        build = build.compile_group(group);
    }
    // A document added later runs the build again.
    println!("cargo::rerun-if-changed=src");
    build.run();
}

/// The documents below `directory`, by their path below `root` with `/` as the separator.
fn collect(root: &Path, directory: &Path, found: &mut Vec<(String, String)>) {
    let Ok(entries) = fs::read_dir(directory) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, found);
        } else if path.extension().is_some_and(|extension| extension == "xaml") {
            let name = path.strip_prefix(root).expect("below the root").components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/");
            let text = fs::read_to_string(&path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()));
            found.push((name, text));
        }
    }
}

/// `views/MainWindow.xaml` is the module `compiled_main_window`.
fn module_name(document: &str) -> String {
    let stem = document.rsplit('/').next().unwrap_or(document).trim_end_matches(".xaml");
    let mut name = String::from("compiled_");
    let mut previous_lower = false;
    for c in stem.chars() {
        if c.is_ascii_uppercase() {
            if previous_lower {
                name.push('_');
            }
            name.push(c.to_ascii_lowercase());
            previous_lower = false;
        } else if c.is_ascii_alphanumeric() {
            name.push(c);
            previous_lower = true;
        } else {
            name.push('_');
            previous_lower = false;
        }
    }
    name
}
