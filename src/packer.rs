use crate::error::Error;
use crate::prelude::*;
use libflate::gzip::Encoder;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Packer<W: Write> {
    writer: W,
    path: PathBuf,
}

impl<W: Write> Packer<W> {
    pub fn new(writer: W) -> Self {
        let config = APPLICATION.config();
        let path = config.path.to_path_buf();
        Self { writer, path }
    }

    pub fn pack(&mut self) -> Result<(), Error> {
        let mut encoder = Encoder::new(&mut self.writer).unwrap();
        {
            let mut archive = tar::Builder::new(&mut encoder);
            for f in WalkDir::new(&self.path) {
                let f = f.unwrap();
                if f.path().is_dir() {
                    continue;
                }
                archive
                    .append_path_with_name(f.path(), f.path().strip_prefix(&self.path).unwrap())
                    .unwrap();
            }
            archive.finish().unwrap();
        }
        encoder.finish().unwrap();
        Ok(())
    }
}
