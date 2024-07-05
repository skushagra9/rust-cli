pub struct CalculateArgs {
    pub a: i32,
    pub b: i32,
    pub operation: Operation,
}
#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn calculate(args: &CalculateArgs) -> Result<i32, String> {
    let result = match args.operation {
        Operation::Add => args.a + args.b,
        Operation::Subtract => args.a - args.b,
        Operation::Multiply => args.a * args.b,
        Operation::Divide => {
            if args.b == 0 {
                return Err("Division by zero error".to_string());
            }
            args.a / args.b
        }
    };

    Ok(result)
}

#[test]
fn test_calculate_add() {
    let args = CalculateArgs {
        a: 5,
        b: 3,
        operation: Operation::Add,
    };

    let result = calculate(&args);
    assert_eq!(result, Ok(8));
}

#[test]
fn test_calculate_subtract() {
    let args = CalculateArgs {
        a: 5,
        b: 3,
        operation: Operation::Subtract,
    };

    let result = calculate(&args);
    assert_eq!(result, Ok(2));
}

#[test]
fn test_calculate_multiply() {
    let args = CalculateArgs {
        a: 5,
        b: 3,
        operation: Operation::Multiply,
    };

    let result = calculate(&args);
    assert_eq!(result, Ok(15));
}

#[test]
fn test_calculate_divide() {
    let args = CalculateArgs {
        a: 10,
        b: 2,
        operation: Operation::Divide,
    };

    let result = calculate(&args);
    assert_eq!(result, Ok(5));
}

#[test]
fn test_calculate_divide_by_zero() {
    let args = CalculateArgs {
        a: 10,
        b: 0,
        operation: Operation::Divide,
    };

    let result = calculate(&args);
    assert_eq!(result, Err("Division by zero error".to_string()));
}
