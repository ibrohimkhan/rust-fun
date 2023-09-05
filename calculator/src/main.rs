enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: Operation) -> Result<f64, String> {
    match operation {
        Operation::Add(a, b) => Ok(a + b),
        Operation::Subtract(a, b) => Ok(a - b),
        Operation::Multiply(a, b) => Ok(a * b),
        Operation::Divide(a, b) => {
            if b == 0. {
                Err("Divide by Zero Error".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
}

fn read_from_stdin(label: &str) -> String {
    println!("{}", label);

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Couldn't read from stdin");

    buffer.trim().to_owned()
}

fn main() -> Result<(), String> {
    let first_number = read_from_stdin("Enter first number")
        .parse::<f64>()
        .map_err(|e| e.to_string())?;

    let second_number = read_from_stdin("Enter second number")
        .parse::<f64>()
        .map_err(|e| e.to_string())?;

    let operation =
        match read_from_stdin("Enter one of the following operation: +, -, *, /").as_str() {
            "+" => Operation::Add(first_number, second_number),
            "-" => Operation::Subtract(first_number, second_number),
            "*" => Operation::Multiply(first_number, second_number),
            "/" => Operation::Divide(first_number, second_number),
            _ => return Err(String::from("Unsupported operation")),
        };

    let result = calculate(operation)?;
    println!("Calculation result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_works() {
        let operation = Operation::Add(5., 5.);
        let result = calculate(operation).unwrap();
        assert_eq!(result, 10.);

        let operation = Operation::Multiply(5., 5.);
        let result = calculate(operation).unwrap();
        assert_eq!(result, 25.);
    }

    #[test]
    fn calculate_fails() {
        let operation = Operation::Divide(5., 0.);
        let result = calculate(operation);
        assert!(result.is_err());
    }
}
