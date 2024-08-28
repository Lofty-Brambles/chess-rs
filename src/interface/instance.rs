use std::io::{BufRead, Write};

use snafu::{ResultExt, Snafu};

use super::constants::ResponseMap;

pub struct Interface<R, W> {
    reader: R,
    writer: W,
}

#[derive(Debug, Snafu)]
pub enum InterfaceErrors {
    #[snafu(display("could not write to the output stream"))]
    WriteFailure { source: std::io::Error },
    #[snafu(display("could not read_line from the input stream"))]
    ReadLineFailure { source: std::io::Error },
}

impl<R, W> Interface<R, W>
where
    R: BufRead,
    W: Write,
{
    pub fn new(reader: R, writer: W) -> Interface<R, W> {
        Interface { reader, writer }
    }

    pub fn send_statement(&mut self, key: ResponseMap) -> Result<(), InterfaceErrors> {
        write!(&mut self.writer, "{}", key.value()).context(WriteFailureSnafu {})
    }

    fn query_once(&mut self, question: &str) -> Result<String, InterfaceErrors> {
        write!(&mut self.writer, "{question}").context(WriteFailureSnafu {})?;
        self.writer.flush().ok();
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .context(ReadLineFailureSnafu {})?;
        Ok(input.trim().to_string())
    }

    pub fn query_till_done<F>(
        &mut self,
        question: String,
        fallback_question: String,
        check_condition: F,
    ) -> Result<String, InterfaceErrors>
    where
        F: Fn(&str) -> bool,
    {
        let mut input = self.query_once(&question)?;
        loop {
            if check_condition(&input) {
                return Ok(input);
            }

            input = self.query_once(&fallback_question)?;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Interface;

    use crate::interface::constants::ResponseMap;

    #[test]
    fn interface_send_statement_test() {
        let mut output = Vec::new();
        let _ = Interface::new(&b""[..], &mut output).send_statement(ResponseMap::Control);
        let output = String::from_utf8(output).expect("not utf-8");

        assert_eq!(ResponseMap::Control.value(), output);
    }

    #[test]
    fn interface_query_once_test() {
        let input = b"general kenobi";
        let mut output = Vec::new();
        let answer = {
            let mut interface = Interface::new(&input[..], &mut output);
            interface.query_once("hello there")
        };

        assert_eq!(String::from_utf8(output).expect("not utf-8"), "hello there");
        assert_eq!(answer.unwrap_or(String::new()), "general kenobi");
    }

    #[test]
    fn interface_query_till_done_test() {
        let input = b"No\nNo\nNo\nYes\n";
        let mut output = Vec::new();

        let question = "Yes/No?".to_string();
        let fallback_question = "Again, Yes/No?".to_string();
        let answer = {
            let mut interface = Interface::new(&input[..], &mut output);
            interface.query_till_done(question, fallback_question, |x| x != "No")
        };

        let expected = "Yes/No?Again, Yes/No?Again, Yes/No?Again, Yes/No?";
        assert_eq!(String::from_utf8(output).expect("not utf-8"), expected);
        assert_eq!(answer.unwrap_or(String::new()), "Yes");
    }
}
