use std::path::PathBuf;
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix;
use gadget::UsbGadget;

pub struct UsbGadgetState {
    pub configfs_path: PathBuf,
    pub udc_name: String,
}

impl UsbGadgetState {
    pub fn new(configfs_path: Option<String>, udc_name: Option<String>) -> UsbGadgetState {
        let mut state = UsbGadgetState {
            configfs_path: PathBuf::from("/sys/kernel/config/usb_gadget"),
            udc_name: String::new(),
        };

        // set different configs_path if one was given
        if let Some(path) = configfs_path {
            state.configfs_path = PathBuf::from(&path);
        }

        // set or retrieve the UDC name
        match udc_name {
            Some(name) => state.udc_name.push_str(name.as_str()),
            None => {
                let udc_dir = PathBuf::from("/sys/class/udc");
                if let Ok(entries) = fs::read_dir(&udc_dir) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            println!("{:?}", entry.file_name());
                            if let Ok(fname) = entry.file_name().into_string() {
                                state.udc_name.push_str(fname.as_str());
                                break;
                            }
                        }
                    }
                }
            }
        }

        return state;
    }

    pub fn enable(&mut self, gadget: UsbGadget) -> Result<bool, String> {
        if !self.configfs_path.exists() {
            return Err("ConfigFs base path does not exist".to_owned());
        }

        let gadget_path = self.configfs_path.join(gadget.name);
        try!(fs::create_dir(&gadget_path).map_err(|e| e.to_string()));

        // vendor and product id
        try!(self.write_data(gadget_path.join("idVendor"),
                             format!("{:#x}", gadget.vendor_id).as_bytes())
                 .map_err(|e| e.to_string()));
        try!(self.write_data(gadget_path.join("idProduct"),
                             format!("{:#x}", gadget.product_id).as_bytes())
                 .map_err(|e| e.to_string()));

        // string attributes
        let lang = format!("{:#x}", &gadget.lang);
        let strings_path = gadget_path.join("strings").join(&lang);
        try!(fs::create_dir_all(&strings_path).map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("serialnumber"),
                             gadget.serial_number.as_bytes())
                 .map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("manufacturer"),
                             gadget.manufacturer.as_bytes())
                 .map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("product"), gadget.product.as_bytes())
                 .map_err(|e| e.to_string()));

        // functions
        let functions_path = gadget_path.join("functions");
        try!(fs::create_dir(&functions_path).map_err(|e| e.to_string()));
        for func in &gadget.functions {
            let fname = format!("{func_type}.{instance}",
                                func_type = func.function_type(),
                                instance = func.instance_name());
            let function_path = functions_path.join(fname);
            try!(fs::create_dir(&function_path).map_err(|e| e.to_string()));
            for (attr, attr_val) in &func.attributes() {
                try!(self.write_data(function_path.join(attr), attr_val)
                         .map_err(|e| e.to_string()));
            }
        }

        // configs
        let configs_path = gadget_path.join("configs");
        try!(fs::create_dir(&configs_path).map_err(|e| e.to_string()));

        for config in &gadget.configs {
            let config_name = format!("{name}.{id}", name = config.name, id = config.id);
            let config_path = configs_path.join(config_name);
            try!(fs::create_dir(&config_path).map_err(|e| e.to_string()));
            let config_strings_path = config_path.join("strings").join(&lang);
            try!(fs::create_dir_all(&config_strings_path).map_err(|e| e.to_string()));

            try!(self.write_data(config_strings_path.join("configuration"),
                                 config.description.as_bytes())
                     .map_err(|e| e.to_string()));

            // symlink config functions
            for func in &gadget.functions {
                let config_functions_path = config_path.join("functions");
                try!(fs::create_dir_all(&config_functions_path).map_err(|e| e.to_string()));

                let fname = format!("{func_type}.{instance}",
                                    func_type = func.function_type(),
                                    instance = func.instance_name());
                let src_path = functions_path.join(&fname);
                let dst_path = config_functions_path.join(&fname);
                try!(unix::fs::symlink(&src_path, &dst_path).map_err(|e| e.to_string()));
            }
        }

        return Ok(true);
    }

    pub fn disable(&mut self, gadget: UsbGadget) -> Result<bool, String> {
        if !self.configfs_path.exists() {
            return Err("ConfigFs base path does not exist".to_owned());
        }

        // TODO: Implement a safe tear down

        return Ok(true);
    }

    fn write_data(&mut self, output_path: PathBuf, data: &[u8]) -> io::Result<()> {
        let mut f = try!(fs::File::create(output_path));
        try!(f.write_all(data));
        return Ok(());
    }
}
