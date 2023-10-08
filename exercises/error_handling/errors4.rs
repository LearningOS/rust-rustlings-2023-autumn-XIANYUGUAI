// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.


#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm... Why is this always returning an Ok value?
        // Solution 1
        /*
        match value {
            ref v if (value > 0) => 
                Ok(PositiveNonzeroInteger(*v as u64)),
            ref v if (value == 0) =>
                Err(CreationError::Zero),
            ref v if (value < 0) =>
                Err(CreationError::Negative),
            _ => todo!(),
        }
        */
        if value > 0 {
            return Ok(PositiveNonzeroInteger(value as u64));
        } else if value == 0 {
            return Err(CreationError::Zero);
        } else {
            return Err(CreationError::Negative);
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
