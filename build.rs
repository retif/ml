fn main() {
    // Only add rpath if the user enabled our cfg flag
    if std::env::var("CARGO_CFG_BUILD_WITH_RPATH").is_ok() {
        let sysroot = std::process::Command::new("rustc")
            .args(["--print", "sysroot"])
            .output()
            .expect("failed to get rustc sysroot")
            .stdout;
        let sysroot = String::from_utf8(sysroot).unwrap().trim().to_owned();

        println!("cargo:rustc-link-arg=-Wl,-rpath={}/lib", sysroot);
    }
}
