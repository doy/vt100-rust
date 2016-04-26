fn main() {
    std::env::set_current_dir("libvt100")
        .unwrap_or_else(|e| { panic!("failed to chdir: {}", e) });
    let out = std::process::Command::new("make")
        .arg("static")
        .output()
        .unwrap_or_else(|e| { panic!("failed to exec: {}", e) });
    if !out.status.success() {
        println!("{}", std::string::String::from_utf8_lossy(&out.stderr));
        std::process::exit(out.status.code().unwrap_or(255));
    }
}
