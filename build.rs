fn main() {
    cc::Build::new()
        .file("sys/RE_stub.c")
        .compile("RE_stub");
}
