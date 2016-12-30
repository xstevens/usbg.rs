use std::fs;
use std::io::Write;
use std::path::Path;
use std::io;

pub fn write_data(output_path: &Path, data: &[u8]) -> io::Result<()> {
    let mut f = try!(fs::File::create(output_path));
    try!(f.write_all(data));
    return Ok(());
}
