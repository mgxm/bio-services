extern crate reqwest;
extern crate tempfile;

use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::fmt;
use std::convert::From;
use std::io::{Error, ErrorKind, Seek, SeekFrom};

pub mod pdb;
pub mod mmtf;

pub use pdb::PdbDownloader;
pub use mmtf::MmtfDownloader;

#[derive(Debug)]
pub enum DownloaderError {
    Request(reqwest::StatusCode),
    Io(Error),
}

impl fmt::Display for DownloaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DownloaderError::Request(ref err) => write!(f, "{}", err),
            DownloaderError::Io(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<Error> for DownloaderError {
    fn from(error: Error) -> Self {
        DownloaderError::Io(error)
    }
}

pub trait ExtFormatter {
    fn format_ext<S: Into<String>>(&self, id: S) -> String;
}

pub trait UrlFormatter {
    fn format_url(&self) -> String;
}

pub trait SaveExt {
    fn save_on<P: AsRef<Path>>(&mut self, path: P) -> Result<File, DownloaderError>;
}

impl SaveExt for File {
    fn save_on<P: AsRef<Path>>(&mut self, path: P) -> Result<File, DownloaderError> {
        // Create a new File.
        let mut file = try!(File::create(path));

        // Get bytes of tmpfile
        let mut bytes: Vec<u8> = Vec::new();
        try!(self.read_to_end(&mut bytes));

        // Save bytes on new file
        try!(file.write_all(&bytes[..]));
        Ok(file)
    }
}

pub trait Downloader: UrlFormatter + ExtFormatter {
    fn prepare_url<S: Into<String>>(&self, id: S) -> String;

    fn download<S: Into<String>>(id: S) -> Result<File, DownloaderError>;

    fn fetch<S: Into<String>>(&self, id: S) -> Result<File, DownloaderError> {
        let uri = self.prepare_url(id);
        Ok(try!(Self::request_file(&uri)))
    }

    fn fetch_and_save_on<S: Into<String>, P: AsRef<Path>>(
        &self,
        id: S,
        path: P,
    ) -> Result<File, DownloaderError> {
        if path.as_ref().is_dir() {
            let id = id.into();

            let id_with_extension = self.format_ext(id.clone());
            let path = path.as_ref().join(id_with_extension);

            let mut file = try!(self.fetch(id));
            file.seek(SeekFrom::Start(0)).unwrap();
            return file.save_on(path);
        } else {
            let error = format!("Invalid path: `{:?}`", &path.as_ref());
            Err(DownloaderError::Io(Error::new(ErrorKind::Other, error)))
        }
    }

    fn request_file(uri: &str) -> Result<File, DownloaderError> {
        let client = reqwest::Client::builder()
            .gzip(false)
            .build()
            .expect("buid failed");

        let mut resp = client.get(uri).send().unwrap();

        match &resp.status() {
            &reqwest::StatusCode::Ok => (),
            _ => return Err(DownloaderError::Request(resp.status())),
        }

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let mut tmpfile: File = tempfile::tempfile().unwrap();
        try!(tmpfile.write_all(&buf));
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        Ok(tmpfile)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_handle_request_file_error() {
        let mmtf = mmtf::MmtfDownloader::new();
        let res = mmtf.fetch("nothing");

        if let DownloaderError::Request(err) = res.unwrap_err() {
            assert_eq!(err, reqwest::StatusCode::NotFound);
        } else {
            panic!();
        }
    }
}
