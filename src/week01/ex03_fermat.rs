#[path = "ex01_fast_exp.rs"]
mod ex01_fast_exp;

pub fn inverse(elem: u32, modulus: u32) -> u32 {
    // a^(p-1) = 1 mod p => a^(p-2) = a^(-1) mod p
    return ex01_fast_exp::fast_power(elem, modulus - 2, modulus);
}

fn main() {
    assert_eq!(inverse(5, 11), 9);
    assert_eq!(inverse(4, 11), 3);
    assert_eq!(inverse(7, 11), 8);
    assert_eq!(inverse(128, 307), 12);
    assert_eq!(inverse(987, 1009), 321);

    println!("success");
}
