use std::collections::HashMap;
use std::num::ParseIntError;

const IDLE: u16                 = 0b0;
const GET_ARG_1_FIRST_CHAR: u16 = 0b1;
const GET_ARG_1: u16            = 0b10;
const ARG_1_FINISH: u16         = 0b100;
const GET_ARG_2: u16            = 0b1000;
const FINISH: u16               = 0b10000;

enum ParseError {
    ArgumentStartWithNumberError(ASWNError),
    ArgumentStartWithNonUnderlineError(ASWNUError),
    TooManyArgmumentError(TMAError),
    StateMachineError(SMError),
    ParseNumberError(ParseIntError)
}

#[derive(Debug)]
struct ASWNError;

impl std::fmt::Display for ASWNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "First argument can not start with number")
    }
}

impl std::error::Error for ASWNError {}

#[derive(Debug)]
struct ASWNUError;

impl std::fmt::Display for ASWNUError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument can not start with a symbol other than '_'")
    }
}

impl std::error::Error for ASWNUError {}

#[derive(Debug)]
struct TMAError;

impl std::fmt::Display for TMAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Only need one argmument")
    }
}

impl std::error::Error for TMAError {}

#[derive(Debug)]
struct SMError;

impl std::fmt::Display for SMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State machine error")
    }
}

impl std::error::Error for SMError {}

#[derive(Debug)]
struct PNError;

impl std::fmt::Display for PNError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Illegal character while parsing number")
    }
}

impl std::error::Error for PNError {}

/// The Dot Instrction e.g. ProProcess Instrction in this assemblyer must abide by this syntax format:
/// .INSTNAME[SPACE]VARIABLENAME[SPACE]VALUE[SPACE or TAB][\n]
/// or
/// .INSTNAME[SPACE]SETTINGS[SPACE or TAB][\n]
/// or
/// .INSTNAME[SPACE]SETTINGSNAME[SPACE]VALUE[SPACE or TAB][\n]
/// So when check the syntex,
/// I decided to follow this process:
/// 1. Traverse each line, find all the line start with ".", push them into a vector, and delate them in
/// orignal vector
/// 2. For each line start with ".", check them if they are start with an available instruction, if not
/// then throw an Error, collect all the Errors
/// 3. If find avaliable instruction, create a thread to process this line, use the state machine parse
/// and check
/// 4. Throw Error if it does not conform to the syntax

pub struct DotInstrctionsProcessor {
    dot_instrctions: Vec<(usize, String)>,
    set_info: HashMap<String, setting_items>
}

enum setting_items {
    sibool(bool),
    sinum(usize),
    sistr(String)
}

impl DotInstrctionsProcessor {
    pub fn new(orignal_data_in_line: Vec<(usize, String)>) -> (DotInstrctionsProcessor, Vec<(usize, String)>) {
        let mut dot_instrctions = vec![];
        let mut instrcutions = vec![];

        for (line_num, line) in orignal_data_in_line {
            if line.starts_with(".") {
                dot_instrctions.push((line_num, line));
            } else if line.eq("") {
                continue;
            } else {
                instrcutions.push((line_num, line))
            }
        }
        (DotInstrctionsProcessor {
            dot_instrctions,
            set_info: HashMap::new()
        }, instrcutions)
    }

    pub fn generate(&mut self) {
        for (line_num, line) in self.dot_instrctions.clone() {
            if line.starts_with(".SET") {
                match self.two_args(line.trim_start_matches(".SET")) {
                    Ok((si, sv)) => {
                        if self.set_info.contains_key(&si) {
                            self.set_info.insert(si, sv);
                        }
                    },
                    Err(pe) => {
                        match pe {
                            ParseError::ArgumentStartWithNonUnderlineError(e) => {

                            },
                            ParseError::ArgumentStartWithNumberError(e) => {

                            },
                            ParseError::ParseNumberError(e) => {

                            },
                            ParseError::StateMachineError(e) => {

                            },
                            ParseError::TooManyArgmumentError(e) => {

                            }
                        }
                    }
                }

            } else if line.starts_with(".DATA") {

            } else if line.starts_with(".ARRAY") {

            } else if line.starts_with(".DEFINE") {

            } else {
                // Error
            }
        }
    }

    /// Instructions that need to remove the beginning of a line
    fn one_arg(&self, line: &str) -> Result<String, ParseError> {
        let mut curr_state = IDLE;

        let mut arg = String::new();

        for c in line.chars() {
            match curr_state {
                IDLE => {
                    if c == ' ' && c == '\t' {
                        curr_state = GET_ARG_1_FIRST_CHAR;
                    }
                },
                GET_ARG_FIRST_CHAR => {
                    if c.is_ascii_digit() {
                        return Err(ParseError::ArgumentStartWithNumberError(ASWNError));
                    } else if c.is_ascii_punctuation() && c != '_' {
                        return Err(ParseError::ArgumentStartWithNonUnderlineError(ASWNUError));
                    } else {
                        arg.push(c);
                        curr_state = GET_ARG_1;
                    }
                },
                GET_ARG => {
                    if c != '\n' && c != ' ' && c != '\t' {
                        arg.push(c)
                    } else {
                        curr_state = ARG_1_FINISH;
                    }
                },
                ARG_1_FINISH => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        return Err(ParseError::TooManyArgmumentError(TMAError));
                    }
                },
                _ => {
                    curr_state = IDLE;
                    return Err(ParseError::StateMachineError(SMError));
                }
            }
        }
        return Ok(arg);
    }

    /// Instructions that need to remove the beginning of a line
    fn two_args(&self, line: &str) -> Result<(String, setting_items), ParseError> {
        let arg_str: u8 = 0b0;
        let arg_not_str: u8 = 0b1;

        let mut curr_state = IDLE;

        let mut arg_2_type = arg_not_str;

        let mut arg_1 = String::new();
        let mut arg_2 = String::new();

        for c in line.chars() {
            match curr_state {
                IDLE => {
                    if c == ' ' && c == '\t' {
                        curr_state = GET_ARG_1_FIRST_CHAR;
                    }
                },
                GET_ARG_1_FIRST_CHAR => {
                    if c.is_ascii_digit() {
                        return Err(ParseError::ArgumentStartWithNumberError(ASWNError));
                    } else if c.is_ascii_punctuation() && c != '_' {
                        return Err(ParseError::ArgumentStartWithNonUnderlineError(ASWNUError));
                    } else {
                        arg_1.push(c);
                        curr_state = GET_ARG_1;
                    }
                },
                GET_ARG_1 => {
                    if c != ' ' && c != '\t' {
                        arg_1.push(c)
                    } else {
                        curr_state = ARG_1_FINISH;
                    }
                },
                ARG_1_FINISH => {
                    if c != ' ' && c != '\t' {
                        if c == '\"' {
                            arg_2_type = arg_str;
                            curr_state = GET_ARG_2;
                        } else {
                            arg_2.push(c);
                            curr_state = GET_ARG_2;
                        }
                    }
                },
                GET_ARG_2 => {
                    if arg_2_type == arg_not_str {
                        if c != ' ' && c != '\t' && c != '\n' {
                            arg_2.push(c);
                        } else {
                            curr_state = FINISH;
                        }
                    } else if arg_2_type == arg_str {
                        if c != '\"' {
                            arg_2.push(c);
                        } else {
                            curr_state = FINISH;
                        }
                    } else {
                        return Err(ParseError::StateMachineError(SMError));
                    }
                },
                FINISH => {
                    if c != ' ' && c != '\t' && c != '\n' {
                        return Err(ParseError::TooManyArgmumentError(TMAError));
                    }
                }
                _ => {
                    curr_state = IDLE;
                    return Err(ParseError::StateMachineError(SMError));
                }
            }
        }

        if arg_2_type == arg_not_str {
            if arg_2.ends_with("H") {
                match usize::from_str_radix(arg_2.trim_end_matches("H"), 16) {
                    Ok(a) => return Ok((arg_1, setting_items::sinum(a))),
                    Err(e) => return Err(ParseError::ParseNumberError(e))
                };
            } else if arg_2.ends_with("O") {
                match usize::from_str_radix(arg_2.trim_end_matches("O"), 8) {
                    Ok(a) => return Ok((arg_1, setting_items::sinum(a))),
                    Err(e) => return Err(ParseError::ParseNumberError(e))
                };
            } else if arg_2.ends_with("B") {
                match usize::from_str_radix(arg_2.trim_end_matches("B"), 2) {
                    Ok(a) => return Ok((arg_1, setting_items::sinum(a))),
                    Err(e) => return Err(ParseError::ParseNumberError(e))
                };
            } else {
                match usize::from_str_radix(arg_2.as_str(), 10) {
                    Ok(a) => return Ok((arg_1, setting_items::sinum(a))),
                    Err(e) => return Err(ParseError::ParseNumberError(e))
                };
            }
        } else if arg_2_type == arg_str {
            if arg_2.to_ascii_uppercase() == "TRUE" {
                return Ok((arg_1, setting_items::sibool(true)));
            } else if arg_2.to_ascii_uppercase() == "FALSE" {
                return Ok((arg_1, setting_items::sibool(false)));
            } else {
                return Ok((arg_1, setting_items::sistr(arg_2)));
            }
        } else {
            return Err(ParseError::StateMachineError(SMError));
        }
    }
}

struct AST {
    
}

fn set_parser(line: String) {

}