pub use services::Downloader;

pub struct Mmtf<'a> {
    version: &'a str,
    uri: &'a str,
    representation: &'a str,
}

impl<'a> Default for Mmtf<'a> {
    fn default() -> Self {
        let version = "1.0";
        let uri = "https://mmtf.rcsb.org";
        let representation = "full";

        Mmtf {
            version,
            uri,
            representation,
        }
    }
}

impl<'a> Mmtf<'a> {
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
}

impl<'a> Downloader for Mmtf<'a> {
    fn new() -> Mmtf<'a> {
        Mmtf {
            ..Default::default()
        }
    }

    fn download(&self, id: &str, path: &str) {
        let file = format!("{}.mmtf.gz", id);
        let uri = format!("{}{}", self.url(), file);
        let path = format!("{}/{}", path, file);

        Self::request_download(&uri, &path);
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
        let custom = Mmtf::new().with_version("2.0").build();
        assert_eq!(custom.url(), "https://mmtf.rcsb.org/v2.0/full/");
    }

    #[test]
    fn mmtf_custom_represetation() {
        let custom = Mmtf::new().with_representation("reduced").build();
        assert_eq!(custom.url(), "https://mmtf.rcsb.org/v1.0/reduced/");
    }

    #[test]
    fn mmtf_custom_uri() {
        let custom = Mmtf::new().with_uri("localhost").build();
        assert_eq!(custom.url(), "localhost/v1.0/full/");
    }
}
