extern crate reqwest;

use std::io::prelude::*;
use std::io::Read;
use std::fs::File;

pub mod rcsb;

pub trait Downloader {
    fn new() -> Self;

    fn download(&self, id: &str, path: &str);

    fn request_download(uri: &str, path: &str) {
        let client = reqwest::Client::builder()
            .gzip(false)
            .build()
            .expect("buid failed");

        let mut resp = client.get(uri).send().unwrap();
        assert!(&resp.status().is_success());

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let mut file = File::create(&path).unwrap();
        file.write_all(&buf);
    }
}

