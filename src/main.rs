use std::{
    process::{
        Command,
        Stdio,
    },
    io,
    io::{
        BufRead,
        BufReader,
        BufWriter,
        Write,
    },
};

extern crate tokio;

use tokio::{
    prelude::*,
};

struct Program {
    reader: Box<dyn BufRead>,
    writer: Box<dyn Write>,
}

impl Program {
    fn new(command_str: String) -> io::Result<Program> {
        let command = Command::new(command_str)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        Ok(Program {
            reader: Box::new(
                BufReader::new(
                    command
                        .stdout
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                "Could not capture stdout"
                            )
                        })?,
                )
            ),
            writer: Box::new(
                BufWriter::new(
                    command
                        .stdin
                        .ok_or_else(|| {
                            io::Error::new(
                                io::ErrorKind::Other,
                                "Could not capture stdin"
                            )
                        })?,
                )
            ),
        })
    }

    fn act(&mut self, input: String) -> io::Result<String> {
        self.writer.write_all(input.as_bytes())?;
        self.writer.flush()?;

        let mut output = String::new();
        let read = future::lazy(|| {
            self.reader.read_line(&mut output);

            future::ok::<_, ()>(())
        });

        tokio::run(read);

        Ok(output)
    }
}

fn main() -> Result<(), io::Error> {
    let mut p = Program::new("./scripts/a.sh".to_string())?;

    loop {
        println!(
            "{}",
            p.act("something new\n".to_string())?,
        );
    }
}
