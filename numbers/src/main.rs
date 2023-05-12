fn main() {
    let my_function = MathOp::Quersumme.function(5);
    let result = my_function(1234);
    let result = my_function(1234);

    println!("{:?}", result);
}

enum MathOp {
    Quersumme,
    Primefaktoren,
}
impl MathOp {
    fn function(&self, i: u64) -> impl Fn(u64) -> u64 {
        match self {
            MathOp::Quersumme => move |n| quersumme(n) + i,
            MathOp::Primefaktoren => todo!()
        }
    }

    fn apply(&self, n: u64) -> MathOpResult {
        match self {
            MathOp::Primefaktoren => MathOpResult::NumberList(prime_factors(n)),
            MathOp::Quersumme => MathOpResult::Number(quersumme(n)),
        }
    }
}

enum MathOpResult {
    Number(u64),
    NumberList(Vec<u64>),
}

fn quersumme(n: u64) -> u64 {
    let mut n = n;
    let mut result = 0;

    while n>0 {
        let last_digit = n%10;
        result += last_digit;
        n = (n-last_digit)/10;
    }

    result
}

fn prime_factors(n: u64) -> Vec<u64> {
    let mut n = n;

    let mut result = Vec::new();

    for candidate in 2..n/2+1 {
        while n%candidate == 0 {
            result.push(candidate);
            n /= candidate;
        }
    }
    if n > 1 {
        result.push(n);
    }

    result
}

#[cfg(test)]
mod test {
    use rstest::*;
    use crate::{prime_factors, quersumme};

    #[rstest]
    #[case::_4(4, vec![2, 2])]
    #[case::_7(7, vec![7])]
    #[case::_12(12, vec![2, 2, 3])]
    fn test_prime_factors(#[case] n: u64, #[case] expected: Vec<u64>) {
        assert_eq!(expected, prime_factors(n));
    }

    #[rstest]
    #[case::_4(4, 4)]
    #[case::_12(12, 3)]
    #[case::_12(124, 7)]
    fn test_quersumme(#[case] n: u64, #[case] expected: u64) {
        assert_eq!(expected, quersumme(n));
    }
}