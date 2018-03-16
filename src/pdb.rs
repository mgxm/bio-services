pub use super::{Downloader, ExtFormatter, SaveExt, UrlFormatter, DownloaderError};
use std::fs::File;

pub struct PdbDownloader<'a> {
    uri: &'a str,
    compression: &'a str,
}

impl<'a> Default for PdbDownloader<'a> {
    fn default() -> Self {
        let uri = "https://files.rcsb.org/download/";
        let compression = "compressed";

        PdbDownloader { uri, compression }
    }
}

impl<'a> PdbDownloader<'a> {
    pub fn new() -> PdbDownloader<'a> {
        PdbDownloader {
            ..Default::default()
        }
    }

    pub fn with_compression(&mut self, compression: &'a str) -> &mut Self {
        self.compression = compression;
        self
    }

    pub fn with_uri(&mut self, uri: &'a str) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn build(&self) -> PdbDownloader<'a> {
        PdbDownloader {
            uri: self.uri,
            compression: self.compression,
        }
    }
}

impl<'a> ExtFormatter for PdbDownloader<'a> {
    fn format_ext<S: Into<String>>(&self, id: S) -> String {
        if self.compression == "compressed" {
            format!("{}.pdb1.gz", id.into())
        } else {
            format!("{}.pdb1", id.into())
        }
    }
}

impl<'a> UrlFormatter for PdbDownloader<'a> {
    fn format_url(&self) -> String {
        format!("{}", self.uri)
    }
}

impl<'a> Downloader for PdbDownloader<'a> {
    fn prepare_url<S: Into<String>>(&self, id: S) -> String {
        format!("{}{}", self.format_url(), self.format_ext(id))
    }

    fn download<S: Into<String>>(id: S) -> Result<File, DownloaderError> {
        Ok(PdbDownloader::new().fetch(id)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::File;
    use std::env;

    #[test]
    fn pdb_default_url() {
        let pdb = PdbDownloader::new();
        assert_eq!(pdb.format_url(), "https://files.rcsb.org/download/");
    }

    #[test]
    fn pdb_with_compression() {
        let pdb = PdbDownloader::new()
            .with_compression("uncompressed")
            .build();
        assert_eq!(pdb.format_ext("1hh3"), "1hh3.pdb1");
        let pdb = PdbDownloader::new();
        assert_eq!(pdb.format_ext("1hh3"), "1hh3.pdb1.gz");
    }

    #[test]
    fn pdb_fech_and_save() {
        let path = env::current_dir().unwrap();
        let full_path = Path::new(&path).join("1hh3.pdb1.gz");
        let mmtf = PdbDownloader::new();
        mmtf.fetch_and_save_on("1hh3", path).unwrap();
        assert_eq!(full_path.exists(), true);
        //remove file
        if full_path.exists() {
            use std::fs;
            fs::remove_file(full_path);
        }
    }

    #[test]
    #[should_panic]
    fn mmtf_panic_on_fech_and_save() {
        let path = Path::new(file!());
        let mmtf = PdbDownloader::new();
        mmtf.fetch_and_save_on("1hh3", path).unwrap();
    }
}
