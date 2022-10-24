use crate::money::Money;
use crate::obligation::{Obligation, Obligations};
use crate::person::Person;
use crate::Solver;

#[derive(Debug, Clone)]
pub struct Payment {
    from: Person,
    amount: Money,
    to: Vec<Person>,
}

impl Payment {
    #[inline(always)]
    pub fn new(from: Person, amount: Money, to: &[Person]) -> Self {
        Self {
            from,
            amount,
            to: to.to_vec(),
        }
    }

    #[inline(always)]
    pub fn builder() -> PaymentBuilder {
        PaymentBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct PaymentBuilder {
    from: Person,
    amount: Money,
    to: Vec<Person>,
}

impl PaymentBuilder {
    #[inline(always)]
    pub fn new(from: Person, amount: Money, to: &[Person]) -> Self {
        Self {
            from,
            amount,
            to: to.to_vec(),
        }
    }

    #[inline(always)]
    pub fn from(mut self, from: Person) -> Self {
        self.from = from;
        self
    }

    #[inline(always)]
    pub fn to(mut self, to: &[Person]) -> Self {
        self.to = to.to_vec();
        self
    }

    #[inline(always)]
    pub const fn amount(mut self, amount: Money) -> Self {
        self.amount = amount;
        self
    }

    #[inline(always)]
    pub fn build(self) -> Payment {
        Payment {
            from: self.from,
            to: self.to,
            amount: self.amount,
        }
    }
}

#[derive(Default)]
pub struct PaymentsBuilder {
    payments: Vec<Payment>,
}

impl PaymentsBuilder {
    #[inline(always)]
    pub fn new(payments: &[Payment]) -> Self {
        Self {
            payments: payments.to_vec(),
        }
    }

    #[inline(always)]
    pub fn record(&mut self, payment: Payment) -> &mut Self {
        self.payments.push(payment);
        self
    }

    #[inline(always)]
    pub fn build(&mut self) -> Payments {
        Payments::new(&self.payments)
    }
}

#[derive(Debug)]
pub struct Payments(Vec<Payment>);

impl Payments {
    #[inline(always)]
    pub fn builder() -> PaymentsBuilder {
        PaymentsBuilder::default()
    }

    #[inline(always)]
    pub fn new(payments: &[Payment]) -> Self {
        Self(payments.to_vec())
    }

    #[inline(always)]
    pub(crate) fn each_pays(&self) -> Obligations {
        let mut obligations = Obligations::builder();

        for payment in &self.0 {
            let to = &payment.to;

            let included = match to.iter().find(|person| *person == &payment.from) {
                Some(_) => 0,
                None => 1,
            };

            let included = to.len() + included;

            let total = f64::from(payment.amount.raw() / included as i32).ceil() as i32;

            for debtor in to {
                if debtor == &payment.from || total == 0 {
                    continue;
                }

                obligations.record(
                    Obligation::builder()
                        .from(debtor.clone())
                        .to(payment.from.clone())
                        .amount(Money::new(total))
                        .build(),
                );
            }
        }

        obligations.build()
    }

    #[inline(always)]
    pub fn who_pays_whom(&self) -> Obligations {
        Solver::from(self.each_pays()).solve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_each_pays() {
        let a = Person::new("A");
        let b = Person::new("B");
        let a_spent = Money::new(10);
        let b_spent = Money::new(20);

        let obligations = Payments::builder()
            .record(
                Payment::builder()
                    .from(a.clone())
                    .to(&vec![b.clone()])
                    .amount(a_spent)
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(b.clone())
                    .to(&vec![a.clone()])
                    .amount(b_spent)
                    .build(),
            )
            .build()
            .each_pays();

        let expected_a_pays = b_spent.raw() / 2;
        let expected_b_pays = a_spent.raw() / 2;

        for o in obligations.raw() {
            match &o.from {
                _ if &o.from == &a => {
                    assert_eq!(expected_a_pays, o.amount.raw());
                }
                _ if &o.from == &b => {
                    assert_eq!(expected_b_pays, o.amount.raw());
                }
                _ => unreachable!(),
            }
        }
    }
}
