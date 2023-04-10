#[cfg(not(target_os = "windows"))]
fn main() {
    cc::Build::new()
        .cpp(true)
        .include("cpp")
        .file("./cpp/binding.cc")
        .flag_if_supported("-std=c++11")
        .warnings_into_errors(true)
        .compile("counter");
}

#[cfg(target_os = "windows")]
fn main() {
    cc::Build::new()
        .cpp(true)
        .include("cpp")
        .file("./cpp/binding.cc")
        .flag_if_supported("/std:c++14")
        .flag_if_supported("/EHsc")
        .warnings_into_errors(true)
        .compile("counter");
}
