use std::{env, fs, path::PathBuf};

fn main() {
    system_deps::Config::new().probe().unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let build_rs_path = manifest_dir.join("build.rs");
    let file_path = manifest_dir.join("src").join("vqsort.cpp");
    let highway_path = manifest_dir.join("third_party").join("highway");

    println!("cargo:rerun-if-changed={}", build_rs_path.display());
    println!("cargo:rerun-if-changed={}", file_path.display());

    cc::Build::new()
        .file(&file_path)
        .include(&highway_path)
        .cpp(true)
        .compile("vqsort");

    println!("cargo:rustc-link-lib=static=vqsort");
    println!("cargo:rustc-link-lib=static=hwy_contrib_sort");

    // Compile .cc files in third_party/highway/hwy/contrib/sort
    let contrib_sort_dir = manifest_dir
        .join("third_party")
        .join("highway")
        .join("hwy")
        .join("contrib")
        .join("sort");
    let mut sort_build = cc::Build::new();
    sort_build.include(&highway_path);
    sort_build.file(contrib_sort_dir.join("vqsort.cc"));

    for entry in fs::read_dir(&contrib_sort_dir).unwrap() {
        let path = entry.unwrap().path();
        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if file_name.starts_with("vqsort_")
                && path.extension().and_then(|ext| ext.to_str()) == Some("cc")
            {
                sort_build.file(path);
            }
        }
    }
    sort_build.file(highway_path.join("hwy").join("abort.cc"));
    sort_build.file(highway_path.join("hwy").join("aligned_allocator.cc"));
    sort_build.file(highway_path.join("hwy").join("per_target.cc"));
    sort_build.file(highway_path.join("hwy").join("perf_counters.cc"));
    sort_build.file(highway_path.join("hwy").join("targets.cc"));
    sort_build.cpp(true).compile("hwy_contrib_sort");
}
