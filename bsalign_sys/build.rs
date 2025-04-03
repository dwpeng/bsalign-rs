fn complie() {
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=rt");

    let debug = std::env::var("CARGO_CFG_DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

    let mut b = cc::Build::new();
    b.file("./wrapper.c")
        .include("./")
        .flag("-std=c99")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-but-set-variable")
        .flag("-DNDEBUG")
        .flag("-D_GNU_SOURCE")
        .flag("-D_FILE_OFFSET_BITS=64")
        .flag("-DVERSION=\"1.2.1\"")
        .static_flag(true)
        .shared_flag(true)
        .warnings(false)
        .extra_warnings(false)
        .cargo_warnings(false);

    if debug {
        b.debug(true);
        b.opt_level(0);
    } else {
        b.debug(false);
        b.opt_level(3);
    }

    #[cfg(target_arch = "x86_64")]
    {
        b.flag("-msse4.2");
        b.flag("-mpopcnt");
    }

    #[cfg(target_arch = "aarch64")]
    {
        b.flag("-march=armv8-a+simd");
        b.flag("-mfpu=neon");
        b.flag("-mfloat-abi=softfp");
    }

    b.compile("bsalign");
}

fn check_bsalign_repo() {
    let project_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // check if bsalign repo is exist
    let bsalign_header_path = format!("{}/bsalign/bsalign.h", project_dir);
    if std::path::Path::new(&bsalign_header_path).exists() {
        return;
    }
    let status = std::process::Command::new("git")
        .args(&["submodule", "update", "--init", "--recursive"])
        .current_dir(&project_dir)
        .status()
        .expect("Failed to update submodule");
    if !status.success() {
        panic!("Failed to update submodule");
    }
    println!("cargo:rerun-if-changed=.gitmodules");
}

fn generate() {
    bindgen::Builder::default()
        .header("./wrapper.h")
        .clang_arg("-std=gnu89")
        .blocklist_var("FP_NAN")
        .blocklist_var("FP_INFINITE")
        .blocklist_var("FP_ZERO")
        .blocklist_var("FP_SUBNORMAL")
        .blocklist_var("FP_NORMAL")
        .derive_default(true)
        .no_default("BSPOAPar")
        .derive_debug(true)
        .wrap_static_fns(false)
        .wrap_unsafe_ops(true)
        .fit_macro_constants(true)
        .dynamic_link_require_all(true)
        .disable_header_comment()
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    check_bsalign_repo();
    complie();
    generate();
}
