use std::str::FromStr;
use strum::EnumString;

use crate::options::output::Output;

trait VecTryGet<T> {
    fn try_get(
        &self,
        index: usize,
    ) -> Option<&T>;
}

impl<T> VecTryGet<T> for Vec<T> {
    fn try_get(
        &self,
        index: usize,
    ) -> Option<&T> {
        if self.get(index).is_none() {
            return None
        } else {
            return self.get(index)
        }
    }
}

const INVALID_INPUT:u64 = 0;


#[derive(Debug, EnumString, PartialEq)]
pub enum CommandLineArgs {
    #[strum(ascii_case_insensitive)]
    Check(u64, Output),

    #[strum(ascii_case_insensitive)]
    List(u64, Output),

    #[strum(ascii_case_insensitive)]
    #[strum(serialize = "Upto", serialize="Count")]
    Count(u64, Output),

    #[strum(disabled)]
    Invalid,
}
impl CommandLineArgs {
    pub fn from_args(
        args:&Vec<String>,
        default:Self,
    ) -> Self {
        let mut query_type_option = args.try_get(1);
        let query_int_option = args.try_get(2);
        let query_output_option = args.try_get(3);

        let query_output:Output = match query_output_option {
            Some(option) => {
                match option.to_lowercase().as_str() {
                    "raw"       => Output::Raw,
                    "pickle"    => Output::Pickle,
                    "readable"  => Output::Readable,
                    _ => Output::default(),
                }
            },
            None => Output::Readable,
        };

        let query_int:u64 = match query_int_option {
            Some(query_content_str) => match query_content_str.parse::<u64>() {
                Ok(query_int) => query_int,
                Err(_) => {
                    query_type_option = None;
                    INVALID_INPUT
                }
            },
            None => {
                query_type_option = None;
                INVALID_INPUT
            }
        };

        if query_int != INVALID_INPUT {
            if let Some(query_type_str) = query_type_option {
                return match Self::from_str(query_type_str) {
                    Ok(command) => {
                        match command {
                            Self::Check(_, _) => Self::Check(query_int, query_output),
                            Self::List(_, _) => Self::List(query_int, query_output),
                            Self::Count(_, _) => Self::Count(query_int, query_output),
                            _ => Self::Invalid,
                        }
                    },
                    Err(_) => Self::Invalid,
                }
            } else {
                return Self::Invalid;
            }
        } else {
            if let Some(_) = query_type_option {
                return Self::Invalid;
            } else {
                // Nothing was specified
                return default;
            }
        };
    }
}
