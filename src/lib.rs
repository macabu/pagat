mod money;
mod obligation;
mod payment;
mod person;
mod solver;

pub use money::*;
pub use obligation::*;
pub use payment::*;
pub use person::*;
pub use solver::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let payments = Payments::builder()
            .record(Payment::new(
                Person::new("A"),
                Money::new(1200),
                &[Person::new("B")],
            ))
            .record(Payment::new(
                Person::new("B"),
                Money::new(1200),
                &[Person::new("A"), Person::new("C"), Person::new("D")],
            ))
            .record(Payment::new(
                Person::new("C"),
                Money::new(1200),
                &[Person::new("A"), Person::new("B"), Person::new("D")],
            ))
            .record(Payment::new(
                Person::new("D"),
                Money::new(1200),
                &[Person::new("B"), Person::new("C")],
            ))
            .build();

        let expected_obligations = Obligations::builder()
            .record(
                Obligation::builder()
                    .from(Person::new("C"))
                    .to(Person::new("D"))
                    .amount(Money::new(100))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("B"))
                    .to(Person::new("D"))
                    .amount(Money::new(100))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("B"))
                    .to(Person::new("C"))
                    .amount(Money::new(300))
                    .build(),
            )
            .build();

        let obligations = payments.who_pays_whom().unwrap();

        assert_eq!(expected_obligations, obligations);
    }

    #[test]
    fn test_multiple_non_payers() {
        let everyone = &[
            Person::new("A"),
            Person::new("B"),
            Person::new("C"),
            Person::new("D"),
            Person::new("E"),
            Person::new("F"),
            Person::new("G"),
            Person::new("H"),
        ];

        let payments = Payments::builder()
            .record(
                Payment::builder()
                    .from(Person::new("A"))
                    .to(everyone)
                    .amount(Money::new(37009))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("B"))
                    .to(everyone)
                    .amount(Money::new(35300))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("C"))
                    .to(everyone)
                    .amount(Money::new(7249))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("D"))
                    .to(everyone)
                    .amount(Money::new(0))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("E"))
                    .to(everyone)
                    .amount(Money::new(0))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("F"))
                    .to(everyone)
                    .amount(Money::new(0))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("G"))
                    .to(everyone)
                    .amount(Money::new(0))
                    .build(),
            )
            .record(
                Payment::builder()
                    .from(Person::new("H"))
                    .to(everyone)
                    .amount(Money::new(0))
                    .build(),
            )
            .build();

        let expected_obligations = Obligations::builder()
            .record(
                Obligation::builder()
                    .from(Person::new("C"))
                    .to(Person::new("A"))
                    .amount(Money::new(2696))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("E"))
                    .to(Person::new("A"))
                    .amount(Money::new(9944))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("G"))
                    .to(Person::new("A"))
                    .amount(Money::new(9944))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("D"))
                    .to(Person::new("B"))
                    .amount(Money::new(9944))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("F"))
                    .to(Person::new("B"))
                    .amount(Money::new(9944))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("H"))
                    .to(Person::new("B"))
                    .amount(Money::new(9944))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("B"))
                    .to(Person::new("A"))
                    .amount(Money::new(4480))
                    .build(),
            )
            .build();

        let obligations = payments.who_pays_whom().unwrap();

        assert_eq!(expected_obligations, obligations);
    }

    #[test]
    fn test_reduce_same_weight() {
        let payments = Payments::builder()
            .record(Payment::new(
                Person::new("A"),
                Money::new(2000),
                &[Person::new("B"), Person::new("C"), Person::new("H")],
            ))
            .record(Payment::new(
                Person::new("C"),
                Money::new(500),
                &[Person::new("H")],
            ))
            .record(Payment::new(
                Person::new("B"),
                Money::new(600),
                &[Person::new("C"), Person::new("H")],
            ))
            .build();

        let expected_obligations = Obligations::builder()
            .record(
                Obligation::builder()
                    .from(Person::new("B"))
                    .to(Person::new("A"))
                    .amount(Money::new(100))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("C"))
                    .to(Person::new("A"))
                    .amount(Money::new(450))
                    .build(),
            )
            .record(
                Obligation::builder()
                    .from(Person::new("H"))
                    .to(Person::new("A"))
                    .amount(Money::new(950))
                    .build(),
            )
            .build();

        let obligations = payments.who_pays_whom().unwrap();

        assert_eq!(expected_obligations, obligations);
    }
}
