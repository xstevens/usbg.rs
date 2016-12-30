extern crate std;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use UsbGadgetFunction;
use util::write_data;

#[derive(Clone)]
pub struct ECMFunction {
    pub instance_name: String,
    pub dev_addr: String,
    pub host_addr: String,
}

impl UsbGadgetFunction for ECMFunction {
    fn instance_name(&self) -> &str {
        return self.instance_name.as_str();
    }

    fn function_type(&self) -> &str {
        return "ecm";
    }

    fn attributes(&self) -> HashMap<&str, Vec<u8>> {
        let mut attrs: HashMap<&str, Vec<u8>> = HashMap::new();

        attrs.insert("dev_addr", format!("{}", self.dev_addr).as_bytes().to_vec());
        attrs.insert("host_addr",
                     format!("{}", self.host_addr).as_bytes().to_vec());

        return attrs;
    }

    fn write_to(&self, base_path: &Path) -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = self.function_type(),
                            instance = self.instance_name());
        let function_path = base_path.join(fname);
        try!(fs::create_dir(&function_path));
        // function attributes
        try!(write_data(function_path.join("dev_addr").as_path(),
                        format!("{}", self.dev_addr).as_bytes()));
        try!(write_data(function_path.join("host_addr").as_path(),
                        format!("{}", self.host_addr).as_bytes()));

        return Ok(());
    }
}
