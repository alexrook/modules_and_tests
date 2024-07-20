pub struct UnsignedCounter(usize);

impl UnsignedCounter {
    pub fn new() -> Self {
        UnsignedCounter(0)
    }

    pub fn next_unsigned(self) -> Self {
        UnsignedCounter(self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_unsigned_counter_should_return_0() {
        let result: UnsignedCounter = UnsignedCounter::new();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn next_unsigned_should_return_1() {
        let result: UnsignedCounter = UnsignedCounter::new().next_unsigned();
        assert_eq!(result.0, 1);
    }
}
