#[path="./fraction.rs"]
mod fraction;
use std::collections::HashMap;

use fraction::Fraction;
use num_bigint::BigInt;
use num_traits::Num;

fn parse_value(base: u32, value: &str) -> BigInt {
    BigInt::from_str_radix(value, base).expect("Invalid value for the given base")
}

/// Generate the secret using arbitrary-precision arithmetic.
pub fn generate_secret(x: &[BigInt], y: &[BigInt], m: usize) -> BigInt {
    let mut ans = Fraction::new(BigInt::from(0), BigInt::from(1));

    for i in 0..m {
        let mut l = Fraction::new(y[i].clone(), BigInt::from(1));
        for j in 0..m {
            if i != j {
                let temp = Fraction::new(
                    -x[j].clone(),
                    x[i].clone() - x[j].clone(),
                );
                l = l * temp;
            }
        }
        ans = ans + l;
    }

    ans.num
}

/// Recover the secret from shares provided in different bases.
pub fn recover_secret_from_input(input: &HashMap<String, HashMap<String, String>>) -> Result<BigInt, &'static str> {
    let keys = &input["keys"];
    let n = keys["n"].parse::<usize>().expect("Invalid n value");
    let k = keys["k"].parse::<usize>().expect("Invalid k value");

    if n < k {
        return Err("Not enough shares to recover the secret.");
    }

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..=n {
        let share = input[&i.to_string()].clone();
        let base = share["base"].parse::<u32>().expect("Invalid base");
        let value = &share["value"];

        let parsed_value = parse_value(base, value);

        x.push(BigInt::from(i as i128));
        y.push(parsed_value);
    }

    Ok(generate_secret(&x, &y, k))
}