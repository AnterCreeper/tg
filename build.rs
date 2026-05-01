fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rerun-if-changed=vendor/find_all_keys_macos.c");
            cc::Build::new()
                .file("vendor/find_all_keys_macos.c")
                .compile("tg_key_scanner");
        }
        "linux" => {
            println!("cargo:rerun-if-changed=vendor/find_all_keys_linux.c");
            cc::Build::new()
                .file("vendor/find_all_keys_linux.c")
                .compile("tg_key_scanner");
        }
        _ => {}
    }
}
