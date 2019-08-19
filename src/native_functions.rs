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
                    "{}-th argument of '+' is not a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(sum))
}
