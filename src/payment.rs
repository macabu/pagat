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
