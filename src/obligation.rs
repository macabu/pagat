use crate::{money::Money, person::Person};

#[derive(Debug)]
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

#[derive(Debug, Default)]
pub struct Obligations(pub(crate) Vec<Obligation>);

impl Obligations {
    #[inline(always)]
    pub const fn obligations(&self) -> &Vec<Obligation> {
        &self.0
    }

    #[inline(always)]
    pub fn record(&mut self, o: Obligation) {
        self.0.push(o)
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
