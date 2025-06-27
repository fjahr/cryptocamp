mod ex01_fast_exp;
mod ex03_fermat;

pub fn elgamal(g: u32, modulus: u32, a: u32, k: u32, m: u32, c1: u32, c2: u32) -> bool {
    // A is the public key Alice publishes
    let A = ex01_fast_exp::fast_power(g, a, modulus);

    // Bob wants to send Alice message m and calculates c1 and c2
    // which is the ciphertext he shares with Alice
    let c1_bob = ex01_fast_exp::fast_power(g, k, modulus);
    assert_eq!(c1_bob, c1);
    let c2_bob = (m * ex01_fast_exp::fast_power(A, k, modulus)) % modulus;
    assert_eq!(c2_bob, c2);

    // Alice calculates m from the cyphertext
    let x = ex01_fast_exp::fast_power(c1, a, modulus);
    let x_inv = ex03_fermat::inverse(x, modulus);
    let m_alice = (x_inv * c2) % modulus;
    assert_eq!(m_alice, m);
    
    true
}

fn main() {
    assert!(elgamal(2, 467, 153, 197, 331, 87, 57));
    // Additional test vectors from here: https://gist.github.com/devinrsmith/19256389288b7e9ff5685a658f9b22d1
    assert!(elgamal(33, 71, 62, 31, 15, 62, 18));
    assert!(elgamal(11, 23, 6, 3, 10, 20, 22));
    assert!(elgamal(3, 809, 68, 89, 100, 345, 517));
    assert!(elgamal(6, 17, 5, 10, 13, 15, 9));

    println!("success");
}
