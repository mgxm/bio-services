extern crate reqwest;
extern crate tempfile;

use std::io::prelude::*;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::io::{Error, Seek, SeekFrom};

pub mod pdb;
pub mod mmtf;

pub trait ExtFormatter {
    fn format_ext<S: Into<String>>(&self, id: S) -> String;
}

pub trait UrlFormatter {
    fn format_url(&self) -> String;
}

pub trait SaveExt {
    fn save_on<P: AsRef<Path>>(&mut self, path: P) -> Result<File, Error>;
}

impl SaveExt for File {
    fn save_on<P: AsRef<Path>>(&mut self, path: P) -> Result<File, Error> {
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

    fn fetch<S: Into<String>>(&self, id: S) -> Result<File, Error> {
        let uri = self.prepare_url(id);
        Ok(try!(Self::request_file(&uri)))
    }

    fn fetch_and_save_on<S: Into<String>, P: AsRef<Path>>(
        &self,
        id: S,
        path: P,
    ) -> Result<File, Error> {
        let id = id.into();

        let id_with_extension = self.format_ext(id.clone());
        let path = path.as_ref().join(id_with_extension);

        let mut file = try!(self.fetch(id));
        file.seek(SeekFrom::Start(0)).unwrap();
        file.save_on(path)
    }

    fn request_file(uri: &str) -> Result<File, Error> {
        let client = reqwest::Client::builder()
            .gzip(false)
            .build()
            .expect("buid failed");

        let mut resp = client.get(uri).send().unwrap();
        assert!(&resp.status().is_success());

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let mut tmpfile: File = tempfile::tempfile().unwrap();
        try!(tmpfile.write_all(&buf));
        tmpfile.seek(SeekFrom::Start(0)).unwrap();
        Ok(tmpfile)
    }
}
