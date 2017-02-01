use std::fs;
use std::io;
use std::path::Path;
use std::os::unix;

use function::UsbGadgetFunction;
use util::write_data;

pub struct UsbGadgetConfig<'a> {
    pub id: u8,
    pub name: &'a str,
    pub description: &'a str,
    pub functions: Vec<Box<UsbGadgetFunction>>,
    pub max_power: Option<u16>,
}

impl<'a> UsbGadgetConfig<'a> {
    pub fn write_to(&self,
                    configs_path: &Path,
                    functions_path: &Path,
                    lang: &str)
                    -> io::Result<()> {
        let config_name = format!("{name}.{id}", name = self.name, id = self.id);
        let config_path = configs_path.join(config_name);
        if !config_path.exists() {
            try!(fs::create_dir(&config_path));
        }
        let config_strings_path = config_path.join("strings").join(&lang);
        if !config_strings_path.exists() {
            try!(fs::create_dir_all(&config_strings_path));
        }

        try!(write_data(config_strings_path.join("configuration").as_path(),
                        self.description.as_bytes()));

        if let Some(max_power) = self.max_power {
            try!(write_data(config_path.join("MaxPower").as_path(),
                            format!("{}", max_power).as_bytes()));
        }

        // symlink config functions
        for func in &self.functions {
            let fname = format!("{func_type}.{instance}",
                                func_type = func.function_type(),
                                instance = func.instance_name());
            let src_path = functions_path.join(&fname);
            let dst_path = config_path.join(&fname);
            if !dst_path.exists() {
                try!(unix::fs::symlink(&src_path, &dst_path));
            }
        }

        Ok(())
    }
}
