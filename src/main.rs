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
    thread,
    time::{
        Duration,
    },
};

use futures::future;

extern crate crossbeam;

use crossbeam::{
    select,
    crossbeam_channel::{
        unbounded,
    },
};

struct Program {
    reader: Box<dyn BufRead + Send>,
    writer: Box<dyn Write + Send>,
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

        let (s, r) = unbounded();

        let mut output = String::new();
        thread::spawn(move || {
            s.send(self.reader.read_line(&mut output)).unwrap();
            //s.send(output).unwrap();
        });

        select!{
            recv(r) -> msg => panic!(),
            default(Duration::from_millis(500)) => println!("timed out"),
        }

        Ok("".to_string())
    }
}

fn main() -> Result<(), io::Error> {
    let mut p = Program::new("./scripts/a.sh".to_string())?;

    // TODO stream::unfold?
    loop {
        println!(
            "{}",
            p.act("something new\n".to_string())?,
        );
    }
}
