#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Person(String);

impl Person {
    #[inline(always)]
    pub fn new(p: &str) -> Self {
        Self(p.into())
    }

    #[inline(always)]
    pub const fn raw(&self) -> &String {
        &self.0
    }
}
