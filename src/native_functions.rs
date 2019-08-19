use crate::interpreter::{MankaiObject, RuntimeError};

/// Sum all the arguments. Return an error if a non numeric argument is found.
pub fn sum(arguments: Vec<MankaiObject>) -> Result<MankaiObject, RuntimeError> {
    let mut sum = 0.0;
    for (i, value) in arguments.iter().enumerate() {
        match value {
            MankaiObject::Number(n) => sum += n,
            _ => {
                return Err(RuntimeError::new(&format!(
                    "{}-th argument is not a number!",
                    i + 1
                )))
            }
        }
    }

    Ok(MankaiObject::Number(sum))
}
