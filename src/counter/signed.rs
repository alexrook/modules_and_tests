#[derive(Debug)]
pub struct SignedCounter(isize);

impl SignedCounter {
    pub fn default_signed_counter() -> Self {
        SignedCounter(0)
    }

    //PLS Help: вопрос по дизайну
    //Должен ли self
    //поглощаться как сейчас
    //или не менятся (&self)
    //или менятся in-place (&mut self)
    //
    pub fn next_signed(self) -> Self {
        SignedCounter(self.0 + 1)
    }

    pub fn prev_signed(self) -> Self {
        SignedCounter(self.0 - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_signed_counter_should_return_0() {
        let result = SignedCounter::default_signed_counter();
        assert_eq!(result.0, 0);
    }

    #[test]
    fn next_signed_should_return_1() {
        let default = SignedCounter::default_signed_counter();
        let result = default.next_signed();
        assert_eq!(result.0, 1);
        // assert_eq!(default.0, 0); //default поглощен
    }

    #[test]
    fn prev_signed_should_return_0() {
        let result = SignedCounter::default_signed_counter().prev_signed();
        assert_eq!(result.0, -1);
    }
}
