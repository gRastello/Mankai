use crate::interpreter::*;
use crate::parser::Sexp;

/// The `set!` special form.
pub fn set(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Check that we have exactly two arguments.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("expected exacly two arguments to 'set!'"));
    }

    // Get token that identifies the name of the variable.
    let name = match arguments.get(0).unwrap() {
        Sexp::Atom(token) => token,
        Sexp::List(_) => {
            return Err(RuntimeError::new(
                "expected identifier as first argument to 'set!'",
            ))
        }
    };

    if interpreter.is_special_form(name) {
        return Err(RuntimeError::new(
            "can't assign to 'set!' because the name is reserved for a special form",
        ));
    }

    // Get the value to assign.
    let value = interpreter.evaluate(arguments.get(1).unwrap())?;
    let value_clone = value.clone();

    // Perform the binding.
    interpreter.environment.define(name, value);
    Ok(value_clone)
}
