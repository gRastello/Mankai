use crate::interpreter::*;
use crate::parser::Sexp;
use crate::token::*;

/// The `define!` special form.
pub fn define(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Check that we have exactly two arguments.
    if arguments.len() != 2 {
        return Err(RuntimeError::new("expected exacly two arguments to 'set!'"));
    }

    // Get token that identifies the name of the variable. Return an error if
    // trying to set! a special form of a native function or if the token is not
    // an identifier.
    let name = match arguments.get(0).unwrap() {
        Sexp::Atom(token) => token,
        Sexp::List(_) => {
            return Err(RuntimeError::new(
                "expected identifier as first argument to 'set!'",
            ))
        }
    };

    if let TokenKind::Identifier = name.kind {

    } else {
        return Err(RuntimeError::new(&format!(
            "'{}' is not an identifier!",
            name.lexeme
        )));
    }

    if interpreter.is_special_form(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a special form!",
            name.lexeme
        )));
    }

    if interpreter.is_native_fucntion(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a native function!",
            name.lexeme
        )));
    }

    if interpreter.is_constant(name) {
        return Err(RuntimeError::new(&format!(
            "can't assign to '{}' because the name is reserved for a constant!",
            name.lexeme
        )));
    }

    // Get the value to assign.
    let value = interpreter.evaluate(arguments.get(1).unwrap())?;
    let value_clone = value.clone();

    // Perform the binding.
    interpreter.environment.define(name, value);
    Ok(value_clone)
}

/// The 'defun!' special form.
pub fn defun(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Arity check.
    if arguments.len() != 3 {
        return Err(RuntimeError::new(
            "'defun!' requires exactly three arguments!",
        ));
    }

    // Get name for the function.
    let name = match arguments.get(0).unwrap() {
        Sexp::Atom(token) => {
            if let TokenKind::Identifier = token.kind {
                token.lexeme.clone()
            } else {
                return Err(RuntimeError::new(
                    "1st argument to 'defun!' must be an identifier!",
                ));
            }
        }
        _ => {
            return Err(RuntimeError::new(
                "1st argument to 'defun!' must be an identifier!",
            ))
        }
    };

    // Get vector of identifiers for the arguments of the function.
    let mut arguments_identifiers = Vec::new();
    let arguments_raw = match arguments.get(1).unwrap() {
        Sexp::List(list) => list,
        _ => {
            return Err(RuntimeError::new(
                "2nd argument to 'defun!' must be a list of identifiers!",
            ))
        }
    };
    for (i, identifier) in arguments_raw.iter().enumerate() {
        match identifier {
            Sexp::Atom(token) => {
                if let TokenKind::Identifier = token.kind {
                    arguments_identifiers.push(token.clone());
                } else {
                    return Err(RuntimeError::new(&format!(
                        "{}th argument is not an identifier!",
                        i + 1
                    )));
                }
            }
            _ => {
                return Err(RuntimeError::new(&format!(
                    "Expected list of arguments: {}th argument is not an identifier!",
                    i + 1
                )));
            }
        }
    }

    // Get the body of the function.
    let body = (*arguments.get(2).unwrap()).clone();

    // Construct the function
    let function = MankaiObject::Function {
        name: Some(name.clone()),
        arguments_identifiers,
        body,
    };
    let function_clone = function.clone();

    // Bind the newly created function to its name.
    interpreter
        .environment
        .define(&Token::new(name, TokenKind::Identifier), function);
    Ok(function_clone)
}

/// The `if!` special form.
pub fn if_special_form(
    interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Check that we have exactly three arguments.
    if arguments.len() != 3 {
        return Err(RuntimeError::new("'if!' requires exactly three arguments!"));
    }

    // Evaluate the condition.
    let condition = interpreter.evaluate(arguments.get(0).unwrap())?;

    // Evaluate the "then" or the "else" branch accordingly.
    match condition {
        MankaiObject::Bool(true) => interpreter.evaluate(arguments.get(1).unwrap()),
        MankaiObject::Bool(false) => interpreter.evaluate(arguments.get(2).unwrap()),
        _ => Err(RuntimeError::new(
            "1st argument to 'if!' must evaluate to a boolean!",
        )),
    }
}

/// The `lambda!` special form. Returns a Mankai function.
pub fn lambda(
    _interpreter: &mut Interpreter,
    arguments: Vec<&Sexp>,
) -> Result<MankaiObject, RuntimeError> {
    // Arity check.
    if arguments.len() != 2 {
        return Err(RuntimeError::new(
            "'lambda!' requires exactly two arguments!",
        ));
    }

    // Get vector of identifiers for the arguments of the function.
    let mut arguments_identifiers = Vec::new();
    let arguments_raw = match arguments.get(0).unwrap() {
        Sexp::List(list) => list,
        _ => return Err(RuntimeError::new("1st argumeth")),
    };
    for (i, identifier) in arguments_raw.iter().enumerate() {
        match identifier {
            Sexp::Atom(token) => {
                if let TokenKind::Identifier = token.kind {
                    arguments_identifiers.push(token.clone());
                } else {
                    return Err(RuntimeError::new(&format!(
                        "{}th argument is not an identifier!",
                        i + 1
                    )));
                }
            }
            _ => {
                return Err(RuntimeError::new(&format!(
                    "Expected list of arguments: {}th argument is not an identifier!",
                    i + 1
                )));
            }
        }
    }

    // Get the body of the function.
    let body = (*arguments.get(1).unwrap()).clone();

    // Return the function.
    Ok(MankaiObject::Function {
        name: None,
        arguments_identifiers,
        body,
    })
}
