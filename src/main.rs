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

fn run<S: AsRef<ffi::OsStr> + ?Sized>(
    s: &S,
) -> io::Result<Box<dyn Iterator<Item = String>>> {
    let command = Command::new(s)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut stdin = command.stdin.unwrap();

    let mut writer = BufWriter::new(&mut stdin);
    writer.write_all("something\nsomething\n".as_bytes());

    let stdout = command
        .stdout
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Could not capture standard output."
            )
        })?;

    Ok(Box::new(
        BufReader::new(stdout)
            .lines()
            .filter_map(|l| l.ok())
    ))
}

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
        self.writer.write_all(input.as_bytes());

        let mut output = String::new();
        self.reader.read_line(&mut output);

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
