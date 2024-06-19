pub type UnsignedCounter = usize;

pub fn default_unsigned_counter() -> UnsignedCounter {
    0
}

pub fn next_unsigned(counter: UnsignedCounter) -> UnsignedCounter {
    counter + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_unsigned_counter_should_return_0() {
        let result: UnsignedCounter = default_unsigned_counter();
        assert_eq!(result, 0);
    }

    #[test]
    fn next_unsigned_should_return_1() {
        let result: UnsignedCounter = next_unsigned(default_unsigned_counter());
        assert_eq!(result, 1);
    }
}
