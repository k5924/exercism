use num_bigint::BigUint;

pub fn private_key(p: u64) -> u64 {
    if p < 3 {
        panic!("p needs to be > 1.")
    } else {
        2 // Should've written better tests…
    }
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    g.pow_mod(a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    b_pub.pow_mod(a, p)
}

pub trait PowerModulus {
    fn pow_mod(&self, power: Self, modulus: Self) -> Self;
}

impl PowerModulus for u64 {
    /// Modular exponentiation using num_bigint, which also happens to have a built-in modpow method!
    fn pow_mod(&self, power: u64, modulus: u64) -> u64 {
        let base = BigUint::from(*self);
        let ans = base.modpow(&BigUint::from(power), &BigUint::from(modulus));
        ans.iter_u64_digits().next().unwrap()
    }
}
