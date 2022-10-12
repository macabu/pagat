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
    fn test() {
        let payments = Payments::builder()
            .record(Payment::new(
                Person::new("A"),
                Money::new(1200),
                &vec![Person::new("B")],
            ))
            .record(Payment::new(
                Person::new("B"),
                Money::new(1200),
                &vec![Person::new("A"), Person::new("C"), Person::new("D")],
            ))
            .record(Payment::new(
                Person::new("C"),
                Money::new(1200),
                &vec![Person::new("A"), Person::new("B"), Person::new("D")],
            ))
            .record(Payment::new(
                Person::new("D"),
                Money::new(1200),
                &vec![Person::new("B"), Person::new("C")],
            ))
            .build();

        let q = payments.each_pays();

        let s = solver::Solver::new(q).solve();
        dbg!(&s);
    }

    #[test]
    fn who_pays_whom() {
        let everyone = &vec![
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
            .record(Payment::new(Person::new("A"), Money::new(37009),everyone))
            .record(Payment::new(Person::new("B"), Money::new(35300),everyone))
            .record(Payment::new(Person::new("C"), Money::new(7249),everyone))
            .record(Payment::new(Person::new("D"), Money::new(0),everyone))
            .record(Payment::new(Person::new("E"), Money::new(0),everyone))
            .record(Payment::new(Person::new("F"), Money::new(0),everyone))
            .record(Payment::new(Person::new("G"), Money::new(0),everyone))
            .record(Payment::new(Person::new("H"), Money::new(0),everyone))
            .build();

        let obligations = payments.who_pays_whom();
        dbg!(&obligations);
    }

    #[test]
    fn test_debug() {
        let payments = Payments::builder()
            .record(Payment::new(Person::new("A"), Money::new(2000), &vec![Person::new("B"), Person::new("C"), Person::new("H")]))
            .record(Payment::new(Person::new("C"), Money::new(500), &vec![Person::new("H")]))
            .record(Payment::new(Person::new("B"), Money::new(600), &vec![Person::new("C"), Person::new("H")]))
            .build();
        
        let obligations = payments.who_pays_whom();
        dbg!(&obligations);
    }
}
