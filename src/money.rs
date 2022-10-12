#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Money(pub(crate) i32);

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

