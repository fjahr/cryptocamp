pub fn fast_power(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut res = 1u32;
    let mut mask = 1u32;
    let mut a = base;

    for i in 0..32 {
        if exp & mask != 0 {
            res = (res * a) % modulus;
        }
        mask <<= 1;
        a = a*a % modulus;
    }

    res
}

fn main() {
    let g: u32 = 3;
    let MOD: u32 = 1000;
    let A: u32 = 218;

    assert_eq!(fast_power(g, A, MOD), 489);

    // Some additional test vectors
    assert_eq!(fast_power(5, 3, 13), 8);
    assert_eq!(fast_power(4, 13, 497), 445);
    assert_eq!(fast_power(34, 3994, 793), 558);

    println!("success");
}
