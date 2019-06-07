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
    },
    slice,
    ffi,
};

fn run<S: AsRef<ffi::OsStr> + ?Sized>(s: &S) -> io::Result<Box<dyn io::BufRead>> {
    let stdout = Command::new(s)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::Other,
                "Could not capture standard output."
            )
        })?;

    Ok(Box::new(BufReader::new(stdout)))
}

fn main() -> Result<(), io::Error> {
    let reader = run("./scripts/a.sh");
    reader?
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())

}
