unsafe extern "C" {
    fn add(left: u64, right: u64) -> u64;
}

fn main() {
    println!("sum: {:?}", unsafe { add(1, 2) });
}
