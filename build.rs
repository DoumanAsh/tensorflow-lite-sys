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
}

fn main() {
    build()
}
