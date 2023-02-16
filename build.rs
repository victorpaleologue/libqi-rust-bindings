use cmake;

fn main() {
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    let cmake_install_dir = cmake::Config::new(".").build();
    let libqi_dir = cmake_install_dir.display().to_string();
    println!("cargo:rustc-link-search=native={}", libqi_dir.to_owned() + "/lib");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=static=qi");
    println!("cargo:rustc-link-lib=static=boost_chrono");
    println!("cargo:rustc-link-lib=static=boost_date_time");
    println!("cargo:rustc-link-lib=static=boost_filesystem");
    println!("cargo:rustc-link-lib=static=boost_program_options");
    println!("cargo:rustc-link-lib=static=boost_random");
    println!("cargo:rustc-link-lib=static=boost_regex");
    println!("cargo:rustc-link-lib=static=boost_thread");

    println!("cargo:rerun-if-changed=src/*.*pp");
    println!("cargo:rerun-if-changed=include/*");
    println!("cargo:rerun-if-changed=src/bridge.rs");
    cxx_build::bridge("src/bridge.rs")
        .file("src/qiri.cpp")
        .flag_if_supported("-std=c++14")
        .include(libqi_dir + "/include")
        // .object(libqi_dir.to_owned() + "/lib/libqi.a")
        .compile("qiri");
}
