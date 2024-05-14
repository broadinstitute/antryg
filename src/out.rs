use std::fs::File;
use std::io;
use std::io::{BufWriter, Stdout, Write};
use std::path::PathBuf;
use crate::error::Error;

pub(crate) enum OutWriter {
    File(BufWriter<File>),
    Stdout(BufWriter<Stdout>)
}

impl OutWriter {
    pub(crate) fn new_file(name: &str) -> Result<Self, Error> {
        let file = Error::wrap_err_str(File::create(name), name)?;
        Ok(OutWriter::File(BufWriter::new(file)))
    }
    pub(crate) fn new_path(name: &PathBuf) -> Result<Self, Error> {
        let file =
            Error::wrap_err(File::create(name), || name.to_string_lossy().to_string())?;
        Ok(OutWriter::File(BufWriter::new(file)))
    }
    fn new_stdout() -> Self {
        OutWriter::Stdout(BufWriter::new(io::stdout()))
    }
    pub(crate) fn new(out: Option<String>) -> Result<Self, Error> {
        match out {
            Some(name) => OutWriter::new_file(&name),
            None => Ok(OutWriter::new_stdout())
        }
    }
}

impl Write for OutWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutWriter::File(writer) => writer.write(buf),
            OutWriter::Stdout(writer) => writer.write(buf)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutWriter::File(writer) => writer.flush(),
            OutWriter::Stdout(writer) => writer.flush()
        }
    }
}
