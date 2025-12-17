fn main() {
    let operator = "+";
    let num1 = 5;
    let num2 = 3;

    let result = match operator {
        "+" => Some(addition(num1, num2).to_string()),
        "-" => Some(subtraction(num1, num2).to_string()),
        "*" => Some(multiplication(num1, num2).to_string()),
        "/" => match division(num1, num2) {
            Ok(result) => Some(result.to_string()),
            Err(error) => Some(error),
        },
        _ => None,
    };

    if let Some(result) = result {
        println!("Result: {}", result);
    } else {
        println!("Invalid operator");
    }
}

fn addition(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn subtraction(num1: i32, num2: i32) -> i32 {
    num1 - num2
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn division(num1: i32, num2: i32) -> Result<i32, String> {
    if num2 == 0 {
        Err(String::from("Division by zero is not allowed"))
    } else {
        Ok(num1 / num2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = addition(2, 3);
        assert_eq!(result, 5)
    }

    #[test]
    fn test_subtraction() {
        let result = subtraction(5, 3);
        assert_eq!(result, 2)
    }

    #[test]
    fn test_multiplication() {
        let result = multiplication(2, 3);
        assert_eq!(result, 6)
    }

    #[test]
    fn test_division() {
        let result = division(6, 3);
        assert_eq!(result, Ok(2))
    }
}
