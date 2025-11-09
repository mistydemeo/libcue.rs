use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = Config::new("vendor/libcue")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
}
