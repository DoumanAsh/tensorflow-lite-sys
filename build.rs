//Reference: https://www.tensorflow.org/lite/guide/build_cmake#build_tensorflow_lite_c_library
//cmake -S ../tensorflow_src/tensorflow/lite/c -DTFLITE_C_BUILD_SHARED_LIBS:BOOL=OFF
//cmake --build . -j

fn build() {
    use std::env;
    use std::process::Command;

    const MSVC_UTF8: &str = "/utf-8";
    const CMAKE_TARGET: &str = "tensorflow/tensorflow/lite/c";

    let generator = match Command::new("ninja").arg("--version").status() {
        Ok(_) => Some("Ninja"),
        Err(_) => None,
    };
    let target = env::var("TARGET").expect("to set TARGET");

    let mut cmake = cmake::Config::new(CMAKE_TARGET);
    cmake.define("TFLITE_C_BUILD_SHARED_LIBS", "OFF")
         .profile("Release")
         .pic(true)
         .uses_cxx11();

    #[cfg(windows)]
    cmake.define("CMAKE_OBJECT_PATH_MAX", "500");

    if let Some(generator) = generator {
        cmake.generator(generator);
    }
    if target.contains("msvc") {
        cmake.cflag(MSVC_UTF8)
             .cxxflag(MSVC_UTF8);
    }
    let output = cmake.build();
    println!("cargo:rustc-link-search=native={}/build", output.display());
    println!("cargo:rustc-link-lib=static=tensorflowlite_c");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=CXX");
}

#[cfg(not(feature = "bindgen"))]
fn generate_lib() {
}

#[cfg(feature = "bindgen")]
fn generate_lib() {
    use std::path::PathBuf;

    #[derive(Debug)]
    struct ParseCallbacks;

    impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
        fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
            if name.starts_with("Tf") {
                Some(bindgen::callbacks::IntKind::Int)
            } else {
                None
            }
        }
    }

    let out = PathBuf::new().join("src").join("bindings.rs");
    let bindings = bindgen::Builder::default().header("tensorflow/tensorflow/lite/c/c_api.h")
                                              .ctypes_prefix("core::ffi")
                                              .use_core()
                                              .generate_comments(true)
                                              .layout_tests(false)
                                              .size_t_is_usize(true)
                                              .allowlist_type("Tf.+")
                                              .allowlist_function("Tf.+")
                                              .allowlist_var("Tf.+")
                                              .clang_arg("-Itensorflow/")
                                              .default_enum_style(bindgen::EnumVariation::Rust {
                                                  non_exhaustive: false
                                              })
                                              .merge_extern_blocks(true)
                                              .parse_callbacks(Box::new(ParseCallbacks))
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");
}

fn main() {
    generate_lib();
    build();
}
