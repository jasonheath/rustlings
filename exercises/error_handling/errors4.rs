// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    
    // There's nothing wrong with this but I question how idiomatic it is
    // However, it very communicative and tight IMO
    //fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
    //    if value  < 0 { return Err(CreationError::Negative) };
    //    if value == 0 { return Err(CreationError::Zero) };
    //    Ok(PositiveNonzeroInteger(value as u64))
    //}

    // the use of the match makes this feel idiomatic to me
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
      match value {
        x if x < 0 => Err(CreationError::Negative),
        x if x == 0 => Err(CreationError::Zero),
        x => Ok(PositiveNonzeroInteger(x as u64))
      }
    }

    // this is leveraging ranges and while neat feels forced in this expression 
    // and the cleaner way to do is experimental so probably shouldn't use
    //fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
    //    // Hmm...? Why is this only returning an Ok value?
    //    match value {
    //      (i64::MIN..=-1) => Err(CreationError::Negative),
    //                    0 => Err(CreationError::Zero),
    //       (1..=i64::MAX) => Ok(PositiveNonzeroInteger(value as u64))
    //    }
    //}
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
