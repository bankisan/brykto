use core::fmt;
use std::fmt::Debug;

// Outputs the extended euclidean algorithm gcd result and its associated Bézout coefficients.
// gcd, u, v
// a * u + b * v = gcd
pub struct EuclideanOutput(i32, i32, i32);

// Useful for testing.
impl fmt::Debug for EuclideanOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a * {} + b * {} = {}", self.1, self.2, self.0)
    }
}

fn extended_euclidean_algorithm_inner(r: (i32, i32), u: (i32, i32)) -> EuclideanOutput {
    let (r_0, r_1) = r;

    if r.1 == 0 {
        return EuclideanOutput(r.0, u.0, 0);
    }

    let q = r.0 / r.1;

    // Remainder.
    let r_i = r.0 - q * r.1;
    let u_i = u.0 - q * u.1;

    extended_euclidean_algorithm_inner((r.1, r_i), (u.1, u_i))
}

pub fn extended_euclidean_algorithm(a: i32, b: i32) -> EuclideanOutput {
    let r_0: i32 = a;
    let u_0: i32 = 1;

    let r_1: i32 = b;
    let u_1: i32 = 0;

    if b == 0 {
        return EuclideanOutput(a, u_0, 0);
    }

    // Get gcd and u.
    let EuclideanOutput(g, mut u, _) = extended_euclidean_algorithm_inner((r_0, r_1), (u_0, u_1));

    // Ensure u is a postive number as it's cleaner when finding
    // the inverse of the mod.
    while u < 0 {
        u += b / g;
    }

    // Use equation to find v:
    // a*u + b*v = g
    let v = (g - a * u) / b;
    EuclideanOutput(g, u, v)
}

// Computes a^{-1} mod n where a and n are coprime.
// If a and n are not coprime, returns 0.
pub fn mod_inverse(a: i32, n: i32) -> i32 {
    match extended_euclidean_algorithm(n, a) {
        EuclideanOutput(1, u, _) => {
            return u;
        }

        // Inverse doesn't exists with non-coprime numbers.
        EuclideanOutput(_, _, _) => {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_euclidean_algorithm() {
        let a = 2357;
        let b = 397;
        let output = extended_euclidean_algorithm(a, b);
        assert_eq!(a * output.1 + b * output.2, output.0);
    }

    #[test]
    fn test_mod_inverse() {
        // Compute 397^-1 = 1 mod 2357
        let a = 2357;
        let b = 397;
        let output = extended_euclidean_algorithm(a, b);

        // u is the inverse.
        assert_eq!((a * output.1) % b, 1);

        let inverse = mod_inverse(b, a);
        assert_eq!(inverse, output.1);
    }

    #[test]
    fn test_mod_inverse_non_coprime() {
        let a = 10;
        let b = 2;

        let inverse = mod_inverse(b, a);
        assert_eq!(inverse, 0);
    }
}
