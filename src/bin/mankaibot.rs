use std::{env, process};

use tbot::prelude::*;
use tbot::types::message::text::EntityKind;

use mankailib::{Interpreter, Lexer, MankaiError, MankaiObject, Parser};

fn main() {
    // Create a new bot from a token given as command line argument.
    let bot_token = match env::args().nth(1) {
        Some(token) => token,
        None => {
            eprintln!("Usage: mankaibot <bot token>");
            process::exit(1);
        }
    };
    let mut bot = tbot::Bot::new(tbot::Token::new(bot_token)).event_loop();

    // The interpreter for this sessions.
    let mut interpreter = Interpreter::new();

    // Handle for messages.
    bot.text(move |context| {
        // Extract all expressions to evaluate from the message.
        let mut expressions = Vec::new();
        for entity in context.text.entities.iter() {
            if let EntityKind::Code | EntityKind::Pre = entity.kind {
                let expr: String = context
                    .text
                    .value
                    .chars()
                    .skip(entity.offset)
                    .take(entity.length)
                    .collect();
                expressions.push(expr);
            }
        }

        // Process each expression.
        for (i, expr) in expressions.iter().enumerate() {
            // Run the expression and get a result to send to the user.
            let result = match run(expr.into(), &mut interpreter) {
                Ok(object) => object.to_string(),
                Err(error) => error.message,
            };

            println!("[{}] {}", i, result);
            let message = format!("[[{}]] `{}`", i, result);

            // Send the result to the user.
            let reply = context
                .send_message_in_reply(tbot::types::parameters::Text::markdown(&message))
                .into_future()
                .map_err(|err| {
                    dbg!(err);
                });
            tbot::spawn(reply);
        }
    });

    // Setup polling.
    let polling = bot
        .polling()
        .last_n_updates(std::num::NonZeroUsize::new(1).unwrap())
        .allowed_updates(&[tbot::types::parameters::Updates::Message])
        .error_handler(|_error| {
            eprintln!("polling error!");
        });

    // Start bot.
    println!("Mankaibot running!");
    polling.start();
}

/// Run an expression from it's String form. Return a Mankai object or an error.
fn run(source: String, interpreter: &mut Interpreter) -> Result<MankaiObject, MankaiError> {
    let mut lexer = Lexer::new(source);
    lexer.scan()?;

    let mut parser = Parser::new(lexer.tokens);
    let sexp = parser.parse()?;

    let value = interpreter.evaluate(&sexp)?;
    Ok(value)
}
