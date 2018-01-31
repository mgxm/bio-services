extern crate reqwest;

pub mod rcsb;

trait Downloader {
    fn download(id: &str, path: &str);
}
