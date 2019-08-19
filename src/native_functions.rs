use crate::interpreter::{MankaiObject, RuntimeError};

/// Sum all the arguments. Return an error if a non numeric argument is found
/// or no arguments are found at all.
pub fn sum(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'+' requires at least one argument!"));
    }

    // Perform the sum.
    let mut sum = 0.0;
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Number(n) => sum += n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument of '+' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(sum))
}

/// Perform subtraction of an arbitrary number of elements.
/// If only one element is given then substract act just inverts it and return,
/// if multiple arguments are given multiple substractions are performed
/// starting from the first argument e.g. substract([a, b, c]) = a - b - c.
pub fn substract(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'-' requires at least one argument!"));
    }

    // If there's only one argument negate it and return.
    if arguments.len() == 1 {
        return match arguments.get(0).unwrap() {
            MankaiObject::Number(n) => Ok(MankaiObject::Number(-n)),
            _ => Err(RuntimeError::new("1st arguments to '-' must be a number!")),
        };
    }

    // If there are more arguments perform the right number of substractions.
    let mut result = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => n.clone(),
        _ => return Err(RuntimeError::new("1st arguments to '-' must be a number!")),
    };

    for (i, value) in arguments.iter().enumerate().skip(1) {
        match value {
            MankaiObject::Number(n) => result -= n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '-' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}

/// Multiply all the arguments. Return an error if a non numeric argument is
/// found or no arguments are found at all.
pub fn multiplication(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'*' requires at least one argument!"));
    }

    // Perform the multiplication of all arguments.
    let mut result = 1.0;
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Number(n) => result *= n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '*' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}

/// Divide all the arguments togheter. Return an error if a non numeric
/// argument is gound or no arguments are found at all.
/// We impose that division([a]) = 1/a.
pub fn division(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    // Check arity.
    if arguments.is_empty() {
        return Err(RuntimeError::new("'/' requires at least one argument!"));
    }

    // Handle the one argument case.
    if arguments.len() == 1 {
        return match arguments.get(0).unwrap() {
            MankaiObject::Number(n) => Ok(MankaiObject::Number(1.0 / n)),
            _ => Err(RuntimeError::new("1st argument to '/' must be a number!")),
        };
    }

    // Handle the multiple arguments case.
    let mut result = match arguments.get(0).unwrap() {
        MankaiObject::Number(n) => n.clone(),
        _ => return Err(RuntimeError::new("1st argument to '/' must be a number!")),
    };

    for (i, value) in arguments.iter().enumerate().skip(1) {
        match value {
            MankaiObject::Number(n) => result /= n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument to '/' must be a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(result))
}
