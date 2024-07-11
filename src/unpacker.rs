use bytes::Bytes;
//use crate::error::Error;
//use libflate::gzip::Decoder;
//use sha2::{Digest, Sha256};
//use std::io::{self, Read};
use std::path::PathBuf;
use tokio_stream::Stream;

pub struct Unpacker<S> {
    //hasher: Hasher<R>,
    path: PathBuf,
    stream: S,
}

impl<S> Unpacker<S>
where
    S: Stream<Item = reqwest::Result<Bytes>>,
{
    pub fn new(stream: S, path: impl Into<PathBuf>) -> Self {
        //let hasher = Hasher::new(reader);
        Self {
            //hasher,
            stream,
            path: path.into(),
        }
    }

    /*    pub fn unpack(&mut self) -> Result<(), Error> {
        let decoder = Decoder::new(&mut self.hasher).unwrap();
        let mut archive = tar::Archive::new(decoder);
        archive.unpack(&self.path).unwrap();
        Ok(())
    }

    pub fn hex_digest(mut self) -> HexDigest {
        // drain remaining data in the tarball
        io::copy(&mut self.hasher, &mut io::sink()).unwrap();
        self.hasher.hex_digest()
    }*/
}

/*struct Hasher<R: Stream<Item = reqwest::Result<Bytes>>> {
    reader: R,
    digest: Sha256,
}

impl<R: Stream<Item = reqwest::Result<Bytes>>> Hasher<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            digest: Sha256::default(),
        }
    }

    pub fn hex_digest(self) -> HexDigest {
        HexDigest(hex::encode(self.digest.finalize()))
    }

    async fn read(&mut self, buffer: &mut [u8]) -> io::Result<Bytes> {
        let bytes = self.reader.next().await?;
        self.digest.update(&buffer[..bytes]);
        Ok(bytes)
    }
}*/

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HexDigest(pub String);

impl HexDigest {
    pub fn new(digest: &str) -> HexDigest {
        HexDigest(digest.to_string())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}
