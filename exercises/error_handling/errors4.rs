// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

// 

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        if value < 0 {
            Err(CreationError::Negative)
        }else if value == 0 {
            Err(CreationError::Zero)
        }else {
            Ok(PositiveNonzeroInteger(value as u64))
        //把value转换成 u64类型，由于前面已经进行了异常处理，这样的操作是合法安全的
        }
        //每个分支都要有明确的return声明（以；结尾，如果不写；则可以不写return），否则会报错
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
