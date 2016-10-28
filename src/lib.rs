#[macro_use]
extern crate error_chain;
extern crate tempfile;

use std::env;
use std::io;
use std::io::{Write, Read};
use std::process::{Command, ExitStatus};

use tempfile::NamedTempFile;

error_chain! {
    foreign_links {
        std::env::VarError, Var;
        io::Error, IoError;
    }

    errors {
        EditorExitedUnsuccessfully(t: ExitStatus) {
            description("editor exited unsuccessfully")
            display("editor exited unsuccessfully: '{}'", t)
        }
    }
}

pub struct Prompt {
    initial_content: String,
}

impl Prompt {
    pub fn new() -> Prompt {
        Prompt { initial_content: String::new() }
    }

    pub fn initial_content(&mut self, content: &str) -> &mut Prompt {
        self.initial_content = content.to_string();
        self
    }

    pub fn execute(&self) -> Result<String> {
        let editor = try!(env::var("EDITOR"));
        let mut tempfile = try!(NamedTempFile::new());

        if !self.initial_content.is_empty() {
            try!(tempfile.write_all(self.initial_content.as_bytes()));
        }

        let path = tempfile.path();
        let status = try!(Command::new(&editor).arg(path).status());
        if !status.success() {
            return Err(ErrorKind::EditorExitedUnsuccessfully(status).into())
        }

        let mut contents = String::new();
        {
            let mut file = try!(tempfile.reopen());
            try!(file.read_to_string(&mut contents));
        }

        Ok(contents)
    }
}
