use crate::parse_error::{ParseError};
use crate::parse_error_id::{ParseErrorId};

#[derive(PartialEq, Eq)]
pub(crate) enum ArgParseState {
    WaitBinary,
    WaitOptionOrParam,
    InParam,
    InParamPossibleEndQuote,
    InParamEscaped,
    InOption,
}

#[derive(PartialEq, Eq)]
pub(crate) enum OptionParseState {
    InCode,
    WaitOptionValue,
    InValue,
    InValuePossibleEndQuote,
    InValueEscaped,
}

pub(crate) struct ParseState {
    pub(crate) multi_char_option_code_requires_double_announcer: bool,
    pub(crate) line: String,
    pub(crate) line_len: usize,
    pub(crate) arg_parse_state: ArgParseState,
    pub(crate) option_parse_state: OptionParseState,
    pub(crate) line_char_idx: usize,
    pub(crate) arg_start_line_char_idx: usize,
    pub(crate) option_code_start_line_char_idx: usize,
    pub(crate) arg_quote_char: char,
    pub(crate) option_announcer_char: char,
    pub(crate) option_code: String,
    pub(crate) option_value_announcer_is_ambiguous: bool,
    pub(crate) current_option_value_may_be_param: bool,
    pub(crate) value_quoted: bool,
    pub(crate) value_bldr: String,
    pub(crate) arg_count: usize,
    pub(crate) option_count: usize,
    pub(crate) param_count: usize,
    pub(crate) current_param_is_binary: bool,
}

impl ParseState {
    pub fn new(
        line: &str,
        first_arg_is_binary: bool,
        multi_char_option_code_requires_double_announcer: bool,
    ) -> Self {
        ParseState {
            multi_char_option_code_requires_double_announcer,
            line: String::from(line),
            line_len: line.chars().count(),
            arg_parse_state: if first_arg_is_binary { ArgParseState::WaitBinary } else { ArgParseState::WaitOptionOrParam },
            option_parse_state: OptionParseState::InCode,
            line_char_idx: 0,
            arg_start_line_char_idx: 0,
            option_code_start_line_char_idx: 0,
            arg_quote_char: '\0',
            option_announcer_char: '\0',
            option_code: String::from(""),
            option_value_announcer_is_ambiguous: false,
            current_option_value_may_be_param: false,
            value_quoted: false,
            value_bldr: String::with_capacity(30),
            arg_count: 0,
            option_count: 0,
            param_count: 0,
            current_param_is_binary: false,
        }
    }

    pub(crate) fn set_option_code(& mut self, optional_ending_index: Option<usize>) -> Result<(), ParseError> {
        let ending_index = optional_ending_index.unwrap_or(self.line_len);
        let raw_option_code = &self.line[self.option_code_start_line_char_idx..ending_index];

        let mut raw_option_iterator = raw_option_code.chars();
        let optioned_first_char = raw_option_iterator.next();
        match optioned_first_char {
            None => {
                self.option_code = String::from("");
                Ok(())
            },
            Some(first_char) => {
                if !self.multi_char_option_code_requires_double_announcer {
                    self.option_code = String::from(raw_option_code);
                    Ok(())
                } else {
                    let first_char_is_announcer = first_char == self.option_announcer_char;
                    let announcer_is_one_char_only = raw_option_iterator.next() != None;
                    if announcer_is_one_char_only {
                        if first_char_is_announcer {
                            self.option_code = String::from("");
                        } else {
                            self.option_code = String::from(raw_option_code);
                        }
                        Ok(())
                    } else {
                        self.option_code = String::from(raw_option_code);
                        if first_char_is_announcer {
                            Ok(())
                        } else {
                            let error = self.create_option_error(ParseErrorId::OptionCodeMissingDoubleAnnouncer);
                            Err(error)
                        }
                    }
                }
            }
        }
    }

    pub fn increment_line_char_idx(&mut self) {
        self.line_char_idx += 1;
    }

    pub fn create_option_error(&self, error_id: ParseErrorId) -> ParseError {
        ParseError::new_option(error_id, self.line_char_idx, self.arg_count, self.option_count, &self.option_code)
    }

    pub fn create_param_error(&self, error_id: ParseErrorId) -> ParseError {
        ParseError::new_param(error_id, self.line_char_idx, self.arg_count, self.option_count, &self.value_bldr)
    }
}