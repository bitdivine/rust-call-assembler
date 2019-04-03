extern crate cc;

fn main() {
    cc::Build::new()
        .file("add.S")
        .compile("add");
}
