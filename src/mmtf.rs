pub use super::{Downloader, ExtFormatter, SaveExt, UrlFormatter};

pub struct MmtfDownloader<'a> {
    version: &'a str,
    uri: &'a str,
    representation: &'a str,
}

impl<'a> Default for MmtfDownloader<'a> {
    fn default() -> Self {
        let version = "1.0";
        let uri = "https://mmtf.rcsb.org";
        let representation = "full";

        MmtfDownloader {
            version,
            uri,
            representation,
        }
    }
}

impl<'a> MmtfDownloader<'a> {
    fn new() -> MmtfDownloader<'a> {
        MmtfDownloader {
            ..Default::default()
        }
    }

    pub fn with_version(&mut self, version: &'a str) -> &mut Self {
        self.version = version;
        self
    }

    pub fn with_representation(&mut self, representation: &'a str) -> &mut Self {
        self.representation = representation;
        self
    }

    pub fn with_uri(&mut self, uri: &'a str) -> &mut Self {
        self.uri = uri;
        self
    }

    pub fn build(&self) -> MmtfDownloader<'a> {
        MmtfDownloader {
            version: self.version,
            uri: self.uri,
            representation: self.representation,
        }
    }
}

impl<'a> ExtFormatter for MmtfDownloader<'a> {
    fn format_ext<S: Into<String>>(&self, id: S) -> String {
        format!("{}.mmtf.gz", id.into())
    }
}

impl<'a> UrlFormatter for MmtfDownloader<'a> {
    fn format_url(&self) -> String {
        format!("{}/v{}/{}/", self.uri, self.version, self.representation)
    }
}

impl<'a> Downloader for MmtfDownloader<'a> {
    fn prepare_url<S: Into<String>>(&self, id: S) -> String {
        format!("{}{}", self.format_url(), self.format_ext(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::fs::File;

    #[test]
    fn mmtf_default_url() {
        let mmtf = MmtfDownloader::new();
        assert_eq!(mmtf.format_url(), "https://mmtf.rcsb.org/v1.0/full/");
    }

    #[test]
    fn mmtf_custom_version() {
        let custom = MmtfDownloader::new().with_version("2.0").build();
        assert_eq!(custom.format_url(), "https://mmtf.rcsb.org/v2.0/full/");
    }

    #[test]
    fn mmtf_custom_represetation() {
        let custom = MmtfDownloader::new().with_representation("reduced").build();
        assert_eq!(custom.format_url(), "https://mmtf.rcsb.org/v1.0/reduced/");
    }

    #[test]
    fn mmtf_custom_uri() {
        let custom = MmtfDownloader::new().with_uri("localhost").build();
        assert_eq!(custom.format_url(), "localhost/v1.0/full/");
    }

    #[test]
    fn mmtf_fech_and_save() {
        let path = Path::new(file!()).parent().unwrap().parent().unwrap();

        let full_path = Path::new(&path).join("173D.mmtf.gz");
        let mmtf = MmtfDownloader::new();
        mmtf.fetch_and_save_on("173D", path).unwrap();
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
        let mmtf = MmtfDownloader::new();
        mmtf.fetch_and_save_on("173D", path).unwrap();
    }
}
