#[link(name = "add")]
extern "C" {
    fn my_adder(a: u64, b: u64) -> u64;
}

fn main() {
    let sum = unsafe { my_adder(1, 2) };
    println!("{}", sum);
}
