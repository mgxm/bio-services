pub mod service {
    trait Downloader {
        fn download(id: &str, path: &str);
    }

    pub mod rcsb {
        extern crate reqwest;
        use super::*;
        use std::io::Read;
        use std::fs::File;
        use std::io::prelude::*;

        pub struct Mmtf {
            version: String,
            uri: String,
            representation: String,
        }

        impl Mmtf {
            pub fn new() -> Self {
                Mmtf { ..Default::default() }
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

        impl Downloader for Mmtf {
            fn download(id: &str, path: &str) {
                let mmtf = Mmtf::new();
                mmtf.download(id, path);
            }
        }

        impl Default for Mmtf {
            fn default() -> Self {
                let version = String::from("1.0");
                let uri     = String::from("https://mmtf.rcsb.org");
                let representation = String::from("full");

                Mmtf { version, uri, representation }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rcsb_url() {
        let rcsb = service::rcsb::Mmtf::new();
        assert_eq!(rcsb.url(), "https://mmtf.rcsb.org/v1.0/full/");
    }
}
