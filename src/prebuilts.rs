use crate::{LConfig, finalize::finalize_build};

/// Default build script for prebuilt binaries
pub fn build_prebuilt_default(lcfg: LConfig) {    
    let current_dir = std::env::current_dir().unwrap().display().to_string();
    let target = std::env::var("TARGET").unwrap();

    println!(
        "Current directory: {}",
        current_dir
    );

    let static_libs_path = format!("{}/prebuilts-git-build/prebuilts/{}/build/staticlibs", current_dir, target);
    println!("Static libs path: {}", static_libs_path);
    let slp = std::path::Path::new(&static_libs_path);
    if !slp.exists() {
        panic!("No prebuilt libs found in repo?");
    }

    println!("cargo:rustc-link-search=native={}", static_libs_path);
    for entry in std::fs::read_dir(&static_libs_path).expect("Failed to read source directory") {  
        let entry = entry.expect("Failed to read entry");
        let src_path = entry.path();

        if src_path.display().to_string().contains("part") && !src_path.display().to_string().contains("part1") {
            // Skip part files that are not part1
            continue;
        }

        if src_path.is_file() && src_path.extension().map_or(false, |ext| ext == "part1") {
            let dst_path = src_path.display().to_string().split(".part").next().unwrap().to_string();

            let mut part_number = 1;
            let mut contents = Vec::new();
            loop {
                let part_file = format!("{}.part{}", dst_path, part_number);
                if std::path::Path::new(&part_file).exists() {
                    // Append the part to the destination file
                    contents.extend(
                        std::fs::read(&part_file).expect("Failed to read part file")
                    );
                    part_number += 1;
                } else {
                    break; // No more parts found
                }
            }

            // Write the combined contents to the destination file
            std::fs::write(&dst_path, contents).expect("Failed to write combined file");
            continue;
        }
    }

    finalize_build(lcfg, true);
}