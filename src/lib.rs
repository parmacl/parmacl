//! Simple Command Line Argument Parser for Rust
//! 
//! A library which uses matchers to parse a command line
//! 
//! This extra

#![allow(clippy::collapsible_else_if)]

mod error;
mod regex_or_text;
mod matcher;
mod arg;
mod parser;

mod parse_state;

pub use error::{
    ErrorId,
    Error,
};

pub use regex_or_text::{
    RegexOrText,
};

pub use matcher:: {
    Matcher,
    Matchers,
    DefaultTagType,
    OptionHasValue,
    ArgType,
    DEFAULT_OPTION_HAS_VALUE,
};

pub use arg::{
    ArgProperties,
    OptionProperties,
    ParamProperties,
    Arg,
    Args,
};

pub use parser::{
    Parser,
    DEFAULT_QUOTE_CHAR,
    DEFAULT_OPTION_ANNOUNCER_CHARS,
    DEFAULT_OPTION_CODES_CASE_SENSITIVE,
    DEFAULT_MULTI_CHAR_OPTION_CODE_REQUIRES_DOUBLE_ANNOUNCER,
    DEFAULT_OPTION_VALUE_ANNOUNCER_CHARS,
    DEFAULT_OPTION_VALUES_CASE_SENSITIVE,
    DEFAULT_PARAMS_CASE_SENSITIVE,
    DEFAULT_EMBED_QUOTE_CHAR_WITH_DOUBLE,
    DEFAULT_ESCAPE_CHAR,
    DEFAULT_PARSE_TERMINATE_CHARS,
};
