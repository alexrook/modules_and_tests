//мне хочется структуру с именованными полями
#[derive(Debug)]
pub struct Pair {
    pub left: i32,
    pub right: i32,
}

impl Pair {
    pub fn default_pair() -> Pair {
        Pair { left: 0, right: 0 }
    }

    pub fn pair_vector_sum(&self, b: &Self) -> Self {
        Pair {
            left: self.left + b.left,
            right: self.right + b.right,
        }
    }

    pub fn pair_scalar_sum(&self, b: &Self) -> i32 {
        self.left + self.right + b.left + b.right
    }
}

impl From<(i32, i32)> for Pair {
    fn from((l, r): (i32, i32)) -> Self {
        Pair { left: l, right: r }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.right == other.right && self.left == other.left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pair_should_return_zero_pair() {
        let result = Pair::default_pair();
        assert_eq!(result, (0, 0).into());
    }

    #[test]
    fn pair_vector_sum_should_return_1() {
        let result = Pair::default_pair().pair_vector_sum(&(1, 1).into());
        assert_eq!(result, (1, 1).into());
    }

    #[test]
    fn pair_scalar_sum_should_return_2() {
        let result = Pair::default_pair().pair_scalar_sum(&(1, 1).into());
        assert_eq!(result, 2);
    }
}
