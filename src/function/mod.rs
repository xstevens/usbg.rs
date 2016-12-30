pub mod hid;
pub mod ecm;

use std::collections::HashMap;
use std::io;
use std::path::Path;

pub trait UsbGadgetFunction {
    fn instance_name(&self) -> &str;
    fn function_type(&self) -> &str;
    fn attributes(&self) -> HashMap<&str, Vec<u8>>;
    fn write_to(&self, base_path: &Path) -> io::Result<()>;
}
    