#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Money(i32);

impl Money {
    #[inline(always)]
    pub const fn new(m: i32) -> Self {
        Self(m)
    }

    #[inline(always)]
    pub const fn raw(&self) -> i32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_money() {
        let raw_amount = 10;
        let money = Money::new(raw_amount);

        assert_eq!(raw_amount, money.raw());
    }
}
