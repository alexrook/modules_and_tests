pub type SignedCounter = isize;

pub fn default_signed_counter() -> SignedCounter {
    0
}

pub fn next_signed(counter: SignedCounter) -> SignedCounter {
    counter + 1
}

pub fn prev_signed(counter: SignedCounter) -> SignedCounter {
    counter - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_signed_counter_should_return_0() {
        let result = default_signed_counter();
        assert_eq!(result, 0);
    }

    #[test]
    fn next_signed_should_return_1() {
        let result = next_signed(0);
        assert_eq!(result, 1);
    }

    #[test]
    fn prev_signed_should_return_0() {
        let result = prev_signed(next_signed(0));
        assert_eq!(result, 0);
    }
}
