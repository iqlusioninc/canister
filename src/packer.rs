use crate::error::CanisterError;
use crate::prelude::*;
use libflate::gzip::Encoder;
use std::io::Write;
use std::path::PathBuf;
use tar;
use walkdir::WalkDir;

pub struct Packer<W: Write> {
    writer: W,
    path: PathBuf,
}

impl<W: Write> Packer<W> {
    pub fn new(writer: W) -> Self {
        let config = app_config();
        let path = config.snapshot.path.to_path_buf();
        Self { writer, path }
    }

    pub fn pack(&mut self) -> Result<(), CanisterError> {
        let encoder = Encoder::new(&mut self.writer).unwrap();
        let mut archive = tar::Builder::new(encoder);
        for f in WalkDir::new(&self.path) {
            let f = f.unwrap();
            archive.append_path(f.path()).unwrap();
        }
        Ok(())
    }
}
