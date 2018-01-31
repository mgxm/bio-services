use super::*;
use std::io::prelude::*;
use std::io::Read;
use std::fs::File;

pub struct Mmtf<'a> {
    version: &'a str,
    uri: &'a str,
    representation: &'a str,
}

impl<'a> Mmtf<'a> {
    pub fn new() -> Self {
        Mmtf { ..Default::default() }
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


    pub fn build(&self) -> Mmtf<'a> {
        Mmtf {
            version: self.version,
            uri: self.uri,
            representation: self.representation,
        }
    }

    pub fn url(&self) -> String {
        format!("{}/v{}/{}/", self.uri, self.version, self.representation)
    }

    pub fn download(&self, id: &str, path: &str) {
        let uri = format!("{}{}.mmtf.gz", self.url(), id);

        let client = reqwest::Client::builder()
            .gzip(false)
            .build().expect("buid failed");

        let mut resp = client.get(&uri).send().unwrap();
        assert!(&resp.status().is_success());

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let path = format!("{}/{}.mmtf", path, id);
        let mut file = File::create(&path).unwrap();
        file.write_all(&buf);
    }
}

impl<'a> Downloader for Mmtf<'a> {
    fn download(id: &str, path: &str) {
        let mmtf = Mmtf::new();
        mmtf.download(id, path);
    }
}

impl<'a> Default for Mmtf<'a> {
    fn default() -> Self {
        let version = "1.0";
        let uri     = "https://mmtf.rcsb.org";
        let representation = "full";

        Mmtf { version, uri, representation }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mmtf_default_url() {
        let mmtf = Mmtf::new();
        assert_eq!(mmtf.url(), "https://mmtf.rcsb.org/v1.0/full/");
    }

    #[test]
    fn mmtf_custom_version() {
        let custom = Mmtf::new()
            .with_version("2.0")
            .build();
        assert_eq!(custom.url(), "https://mmtf.rcsb.org/v2.0/full/");
    }

    #[test]
    fn mmtf_custom_represetation() {
        let custom = Mmtf::new()
            .with_representation("reduced")
            .build();
        assert_eq!(custom.url(), "https://mmtf.rcsb.org/v1.0/reduced/");
    }

    #[test]
    fn mmtf_custom_uri() {
        let custom = Mmtf::new()
            .with_uri("localhost")
            .build();
        assert_eq!(custom.url(), "localhost/v1.0/full/");
    }
}
