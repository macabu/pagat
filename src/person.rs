use alloc::string::String;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct Person(String);

impl Person {
    #[inline(always)]
    pub fn new(p: impl Into<String>) -> Self {
        Self(p.into())
    }

    #[inline(always)]
    pub const fn raw(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::borrow::ToOwned;

    #[test]
    fn test_newtype_person() {
        let raw_name = "Alice Bob";
        let from_str_ref = Person::new(raw_name);
        assert_eq!(raw_name, from_str_ref.raw());

        let raw_name = "Bob Alice".to_owned();
        let from_owned_string = Person::new(raw_name.clone());
        assert_eq!(&raw_name, from_owned_string.raw());
    }
}
