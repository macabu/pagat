use alloc::vec::Vec;

use crate::{money::Money, person::Person};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obligation {
    pub from: Person,
    pub to: Person,
    pub amount: Money,
}

impl Obligation {
    #[inline(always)]
    pub fn builder() -> ObligationBuilder {
        ObligationBuilder::default()
    }
}

#[derive(Default)]
pub struct ObligationBuilder {
    from: Person,
    to: Person,
    amount: Money,
}

impl ObligationBuilder {
    #[inline(always)]
    pub const fn new(from: Person, to: Person, amount: Money) -> Self {
        Self { from, to, amount }
    }

    #[inline(always)]
    pub fn from(mut self, from: Person) -> Self {
        self.from = from;
        self
    }

    #[inline(always)]
    pub fn to(mut self, to: Person) -> Self {
        self.to = to;
        self
    }

    #[inline(always)]
    pub const fn amount(mut self, amount: Money) -> Self {
        self.amount = amount;
        self
    }

    #[inline(always)]
    pub fn build(self) -> Obligation {
        Obligation {
            from: self.from,
            to: self.to,
            amount: self.amount,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Obligations(Vec<Obligation>);

impl Obligations {
    #[inline(always)]
    pub fn new(obligations: &[Obligation]) -> Self {
        Self(obligations.to_vec())
    }

    #[inline(always)]
    pub fn builder() -> ObligationsBuilder {
        ObligationsBuilder::default()
    }

    #[inline(always)]
    pub const fn raw(&self) -> &Vec<Obligation> {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct ObligationsBuilder {
    obligations: Vec<Obligation>,
}

impl ObligationsBuilder {
    #[inline(always)]
    pub fn record(&mut self, o: Obligation) -> &mut Self {
        self.obligations.push(o);
        self
    }

    #[inline(always)]
    pub fn build(&mut self) -> Obligations {
        Obligations::new(&self.obligations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::borrow::ToOwned;

    #[test]
    fn test_obligations() {
        let from_raw = "from";
        let to_raw = "to";
        let amount_raw = 10;

        let from = Person::new(from_raw.to_owned());
        let to = Person::new(to_raw.to_owned());
        let amount = Money::new(amount_raw);

        let obligation = Obligation::builder()
            .from(from)
            .to(to)
            .amount(amount)
            .build();

        assert_eq!(from_raw, obligation.from.raw());
        assert_eq!(to_raw, obligation.to.raw());
        assert_eq!(amount_raw, obligation.amount.raw());

        let obligations = Obligations::builder().record(obligation).build();
        assert_eq!(1, obligations.raw().len());
    }
}
