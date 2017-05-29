use std::fs;
use std::io::Write;
use std::path::Path;
use std::io;

pub fn write_data(output_path: &Path, data: &[u8]) -> io::Result<()> {
    let mut f = fs::File::create(output_path)?;
    f.write_all(data)?;

    Ok(())
}

pub fn create_dir_if_not_exists(path: &Path) -> io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    Ok(())
}
