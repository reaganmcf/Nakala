use crate::val::Val;
use nu_ansi_term::Color::{Green, Red, Yellow};

#[derive(Debug)]
pub enum EngineError {
    InvalidAddOperation { x: Val, y: Val },
    InvalidSubOperation { x: Val, y: Val },
    InvalidMulOperation { x: Val, y: Val },
    InvalidDivOperation { x: Val, y: Val },
    InvalidNegOperation { x: Val },
    InvalidGreaterThanOperation { x: Val, y: Val },
    InvalidGreaterThanOrEqOperation { x: Val, y: Val },
    InvalidLessThanOperation { x: Val, y: Val },
    InvalidLessThanOrEqOperation { x: Val, y: Val },
    InvalidNotOperation { x: Val },
    InvalidAndOperation { x: Val, y: Val },
    InvalidOrOperation { x: Val, y: Val },
    VariableAlreadyExists { variable_name: String },
    VariableUndefined { variable_name: String },
    FunctionAlreadyExists { function_name: String },
    FunctionUndefined { function_name: String },
    MismatchedParameterCount { actual: usize, expected: usize },
    MismatchedTypes { actual: Val, expected: Val },
    NotYetImplemented,
    Unknown,
}

fn missing_handler_msg(handler: &str, x: &Val, y: &Val) -> String {
    format!(
        "Could not find {} handler for the provided types {} and {}",
        handler,
        Yellow.paint(x.get_type()),
        Yellow.paint(y.get_type())
    )
}

fn missing_handler_msg_single(handler: &str, x: &Val) -> String {
    format!(
        "Could not find {} handler for the provided type {}",
        handler,
        Yellow.paint(x.get_type())
    )
}

impl std::fmt::Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            EngineError::InvalidAddOperation { x, y } => missing_handler_msg("ADD", x, y),
            EngineError::InvalidSubOperation { x, y } => missing_handler_msg("SUB", x, y),
            EngineError::InvalidMulOperation { x, y } => missing_handler_msg("MUL", x, y),
            EngineError::InvalidDivOperation { x, y } => missing_handler_msg("DIV", x, y),
            EngineError::InvalidNegOperation { x } => missing_handler_msg_single("NEG", x),
            EngineError::InvalidGreaterThanOperation { x, y } => {
                missing_handler_msg("GREATER_THAN", x, y)
            }
            EngineError::InvalidGreaterThanOrEqOperation { x, y } => {
                missing_handler_msg("GREATER_THAN_OR_EQ", x, y)
            }
            EngineError::InvalidLessThanOperation { x, y } => {
                missing_handler_msg("LESS_THAN", x, y)
            }
            EngineError::InvalidLessThanOrEqOperation { x, y } => {
                missing_handler_msg("LESS_THAN_OR_EQ", x, y)
            }
            EngineError::InvalidNotOperation { x } => missing_handler_msg_single("NOT", x),
            EngineError::InvalidAndOperation { x, y } => missing_handler_msg("AND", x, y),
            EngineError::InvalidOrOperation { x, y } => missing_handler_msg("OR", x, y),
            EngineError::VariableAlreadyExists { variable_name } => format!(
                "The variable {} already exists in the scope",
                Yellow.paint(variable_name)
            ),
            EngineError::VariableUndefined { variable_name } => {
                format!(
                    "The variable {} is undefined in the scope",
                    Yellow.paint(variable_name)
                )
            }
            EngineError::FunctionAlreadyExists { function_name } => format!(
                "The function {} already exists in the scope",
                Yellow.paint(function_name)
            ),

            EngineError::FunctionUndefined { function_name } => {
                format!(
                    "The function {} is undefined in the scope",
                    Yellow.paint(function_name)
                )
            }
            EngineError::MismatchedParameterCount { actual, expected } => format!(
                "The function expected {} parameters, but received {}",
                Green.paint(format!("{}", expected)),
                Yellow.paint(format!("{}", actual))
            ),
            EngineError::MismatchedTypes { actual, expected } => format!(
                "Expected type {}, but got {} instead",
                Green.paint(expected.get_type().to_string()),
                Yellow.paint(actual.get_type().to_string()),
            ),
            EngineError::NotYetImplemented => "This feature is not yet implemented".into(),
            EngineError::Unknown => "An unknown error occurred".into(),
        };

        f.write_str(format!("{}: {}", Red.paint("Engine Error"), msg).as_str())
    }
}

impl std::error::Error for EngineError {}
