fn main() {
    cxx_build::bridge("src/main.rs")
        .files(["include/test.cpp"])
        .flag_if_supported("-std=c++14")
        .compile("demo");
}
