#[path="./fraction.rs"]
mod fraction;
use fraction::Fraction;

pub fn generate_secret(x: &[i128], y: &[i128], m: usize) -> i128 {
    let mut ans = Fraction::new(0, 1);

    for i in 0..m {
        let mut l = Fraction::new(y[i], 1);
        for j in 0..m {
            if i != j {
                let temp = Fraction::new(-x[j], x[i] - x[j]);
                l = l * temp;
            }
        }
        ans = ans + l;
    }

    ans.num
}

pub fn recover_secret(shares: &[(i128, i128)], k: usize) {
    if shares.len() < k {
        println!("Not enough shares to recover the secret.");
        return;
    }

    let x: Vec<i128> = shares.iter().take(k).map(|(x, _)| *x).collect();
    let y: Vec<i128> = shares.iter().take(k).map(|(_, y)| *y).collect();

    let secret = generate_secret(&x, &y, k);
    println!("Recovered Secret: {}", secret);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_secret() {
        let x = vec![1, 2, 3, 4];
        let y = vec![602, 1139, 1676, 2213];
        let result = generate_secret(&x, &y, 3);
        assert_eq!(result, 65);
    }
}
