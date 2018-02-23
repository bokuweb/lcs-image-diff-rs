use std::io;
use std::path::Path;
use std::fs::create_dir_all;

pub fn mkdirp<P: AsRef<Path>>(p: P) -> io::Result<()> {
    if let Err(e) = create_dir_all(p) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Ok(());
        }
        return Err(e);
    }
    Ok(())
}