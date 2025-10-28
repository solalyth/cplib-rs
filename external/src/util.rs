use proconio::source::once::OnceSource;
use std::io::BufReader;
use std::fs::File;

pub fn from_file(path: impl AsRef<std::path::Path>) -> OnceSource<BufReader<File>> {
    let f = File::open(path).unwrap();
    OnceSource::new(BufReader::new(f))
}
