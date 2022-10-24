use crate::money::Money;
use crate::obligation::{Obligation, Obligations};
use crate::person::Person;
use crate::Solver;

#[derive(Debug, Clone)]
pub struct Payment {
    by: Person,
    amount: Money,
    to: Vec<Person>,
}

impl Payment {
    #[inline(always)]
    pub fn new(by: Person, amount: Money, to: &[Person]) -> Self {
        Self {
            by,
            amount,
            to: to.to_vec(),
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
pub struct Payments(pub(crate) Vec<Payment>);

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
        let mut obligations = Obligations::default();

        for payment in &self.0 {
            let to = &payment.to;
        
            let included = match to.iter().find(| person| *person == &payment.by) {
                Some(_) => 0,
                None => 1,
            };

            let included = to.len() + included;

            let total = f64::from(payment.amount.raw() / included as i32).ceil() as i32;

            for debtor in to {
                if debtor == &payment.by || total == 0 {
                    continue;
                }

                obligations.record(
                    Obligation::builder()
                        .from(debtor.clone())
                        .to(payment.by.clone())
                        .amount(Money::new(total))
                        .build(),
                );
            }
        }

        obligations
    }

    #[inline(always)]
    pub fn who_pays_whom(&self) -> Obligations {
        Solver::from(self.each_pays()).solve()
    }
}
