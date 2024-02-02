use cmake::Config;
use copy_to_output::copy_to_output;
use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=resources/*");
    println!("cargo:rerun-if-changed=oodlerelay/*");

    let dst = Config::new("oodlerelay").build_target("install").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=oodlerelay");

    copy_to_output("resources", &env::var("PROFILE").unwrap()).expect("Could not copy");
}
