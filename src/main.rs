use std::process::{
    Command,
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

fn main() -> Result<(), io::Error> {
    let reader = run("./scripts/a.sh");
    reader?
        .for_each(|line| println!("{}", line));

    Ok(())

}
