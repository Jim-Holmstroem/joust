use std::process::{
    Command,
    Child,
    Stdio,
};
use std::{
    io,
    io::{
        BufRead,
        BufReader,
        Read,
        BufWriter,
        Write,
    },
    slice,
    ffi,
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
        println!("act({})", input);
        self.writer.write_all(input.as_bytes());
        println!("wrote");

        let mut output = String::new();
        self.reader.read_line(&mut output);
        println!("read");

        Ok(output)
    }
}

fn main() -> Result<(), io::Error> {
    let mut program = Program::new("./scripts/a.sh".to_string())?;

    loop {
        println!(".");
        program.act("something new\n".to_string());
    }

    Ok(())
}
