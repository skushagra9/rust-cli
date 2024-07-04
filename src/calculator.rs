use clap::Parser;

#[derive(Parser)]
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

impl std::str::FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "add" => Ok(Operation::Add),
            "subtract" => Ok(Operation::Subtract),
            "multiply" => Ok(Operation::Multiply),
            "divide" => Ok(Operation::Divide),
            _ => Err(format!("Unsupported operation: {}", s)),
        }
    }
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
