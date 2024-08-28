use std::io::{stdin, stdout, StdinLock, Stdout};

use crate::interface::{constants::ResponseMap, instance::Interface};

pub struct Flow<'a> {
    interface: Interface<StdinLock<'a>, Stdout>,
}

impl<'a> Flow<'a> {
    pub fn new() -> Flow<'a> {
        let interface = Interface::new(stdin().lock(), stdout());
        Flow { interface }
    }

    pub fn start(&mut self) {
        let action = self.interface.query_till_done(
            ResponseMap::Control.value(),
            format!("{}\n\nInvalid! Try again:", ResponseMap::Control.value()),
            |x| b"012".contains(&x.bytes().next().unwrap_or(b'9')),
        );

        match action.unwrap().bytes().next() {
            Some(b'1') => todo!(),
            Some(b'2') => todo!(),
            _ => println!("Thank you!"),
        }
    }
}
