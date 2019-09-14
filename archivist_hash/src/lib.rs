use digest::Digest;
use std::fs::File;
use std::io;
use std::io::Write;

fn _hash_file<D: Digest + Write>(path: &str) -> io::Result<Vec<u8>> {
    let mut hasher = D::new();
    let mut file = File::open(path)?;
    io::copy(&mut file, &mut hasher)?;
    Ok(hasher.result().to_owned().to_vec())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
