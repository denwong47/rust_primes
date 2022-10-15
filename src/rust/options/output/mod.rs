use std::io::{stdout, Write};
use strum::EnumString;

use serde_pickle;
use serde::ser::Serialize;

#[derive(Debug, EnumString, PartialEq)]
pub enum Output {
    #[strum(ascii_case_insensitive)]
    Raw,

    #[strum(ascii_case_insensitive)]
    Pickle,

    #[strum(ascii_case_insensitive)]
    Readable,
}
impl Default for Output {
    fn default() -> Self {
        return Self::Readable;
    }
}
impl Output {
    pub fn display<T: Serialize + std::fmt::Debug>(&self, value:&T) {
        return match &self {
            Self::Raw => {
                println!("{:?}", value)
            },
            Self::Pickle => {
                let pickled:Vec<u8> = serde_pickle::ser::to_vec(
                    value,
                    serde_pickle::ser::SerOptions::new(),
                ).or_else(
                    |_| {Ok::<Vec<u8>, u8>(vec![0,0,0,0])}
                ).unwrap();

                let mut stdout_writer = stdout();

                // Must use the return
                stdout_writer.write_all(&pickled).unwrap();
            },
            Self::Readable => {
                println!("Returned Value: {:?}", value)
            },
        }
    }
}
