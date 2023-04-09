fn main() {
    cc::Build::new()
        .cpp(true)
        .include("cpp")
        .file("./cpp/binding.cc")
        // .cpp_link_stdlib("stdc++")
        .flag_if_supported("-std=c++11")
        .warnings_into_errors(true)
        .compile("counter");
}
