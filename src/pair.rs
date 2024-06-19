pub type Pair = (i32, i32);

pub fn default_pair() -> Pair {
    (0, 0)
}

pub fn pair_vector_sum(a: Pair, b: Pair) -> Pair {
    (a.0 + b.0, a.1 + b.1)
}

pub fn pair_scalar_sum(a: Pair, b: Pair) -> i32 {
    a.0 + a.1 + b.0 + b.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pair_should_return_zero_pair() {
        let result = default_pair();
        assert_eq!(result, (0, 0));
    }

    #[test]
    fn pair_vector_sum_should_return_1() {
        let result = pair_vector_sum(default_pair(), (1, 1));
        assert_eq!(result, (1, 1));
    }

    #[test]
    fn pair_scalar_sum_should_return_2() {
        let result = pair_scalar_sum(default_pair(), (1, 1));
        assert_eq!(result, 2);
    }
}
