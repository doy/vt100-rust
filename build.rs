extern crate cc;
extern crate pkg_config;

fn libvt100() {
    let dir = std::env::current_dir()
        .unwrap_or_else(|e| { panic!("couldn't get cwd: {}", e) });;
    std::env::set_current_dir("libvt100")
        .unwrap_or_else(|e| { panic!("failed to chdir: {}", e) });
    let absdir = std::env::current_dir()
        .unwrap_or_else(|e| { panic!("couldn't get cwd: {}", e) });;
    let out = std::process::Command::new("make")
        .arg("static")
        .output()
        .unwrap_or_else(|e| { panic!("failed to exec: {}", e) });
    std::env::set_current_dir(dir)
        .unwrap_or_else(|e| { panic!("failed to chdir: {}", e) });
    if !out.status.success() {
        println!("{}", std::string::String::from_utf8_lossy(&out.stderr));
        std::process::exit(out.status.code().unwrap_or(255));
    }

    println!("cargo:rustc-link-search=native={}", absdir.to_str().unwrap());
    println!("cargo:rustc-link-lib=static=vt100");
}

fn glib() {
    let lib_def = pkg_config::probe_library("glib-2.0")
        .unwrap_or_else(|e| {
            panic!("Couldn't find required dependency glib-2.0: {}", e);
        });
    for dir in lib_def.link_paths {
        println!("cargo:rustc-link-search=native={}", dir.to_str().unwrap());
    }
}

fn libvt100_wrappers() {
    cc::Build::new()
        .file("src/ffi.c")
        .compile("vt100wrappers");
}

fn main() {
    libvt100();
    glib();
    libvt100_wrappers();
}
