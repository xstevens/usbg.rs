use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix;
use UsbGadget;
use UsbGadgetFunction;
use UsbGadgetConfig;
use util::write_data;
use util::create_dir_if_not_exists;

pub struct UsbGadgetState {
    configfs_path: PathBuf,
    udc_name: String,
}

impl UsbGadgetState {
    pub fn new() -> UsbGadgetState {
        let mut state = UsbGadgetState {
            configfs_path: PathBuf::from("/sys/kernel/config/usb_gadget"),
            udc_name: String::new(),
        };

        let udc_dir = PathBuf::from("/sys/class/udc");
        if let Ok(entries) = fs::read_dir(&udc_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("Using UDC: {:?}", entry.file_name());
                    if let Ok(fname) = entry.file_name().into_string() {
                        state.udc_name.push_str(fname.as_str());
                        break;
                    }
                }
            }
        }

        return state;
    }

    pub fn configfs_path<'a>(&'a mut self, configfs_path: PathBuf) -> &'a mut UsbGadgetState {
        self.configfs_path = configfs_path;
        self
    }

    pub fn udc_name<'a>(&'a mut self, udc_name: &str) -> &'a mut UsbGadgetState {
        self.udc_name = String::from(udc_name);
        self
    }

    pub fn enable(&mut self, gadget: UsbGadget) -> io::Result<()> {
        if !self.configfs_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, "ConfigFs path does not exist"));
        }

        let gadget_path = self.configfs_path.join(gadget.name);
        try!(create_dir_if_not_exists(&gadget_path));

        // vendor and product id
        try!(write_data(gadget_path.join("idVendor").as_path(),
                        format!("0x{:04x}", gadget.vendor_id).as_bytes()));
        try!(write_data(gadget_path.join("idProduct").as_path(),
                        format!("0x{:04x}", gadget.product_id).as_bytes()));

        // bcdDevice and bcdUSB
        if let Some(bcd_device) = gadget.bcd_device {
            try!(write_data(gadget_path.join("bcdDevice").as_path(),
                            format!("0x{:04x}", bcd_device).as_bytes()));
        }
        if let Some(bcd_usb) = gadget.bcd_usb {
            try!(write_data(gadget_path.join("bcdUSB").as_path(),
                            format!("0x{:04x}", bcd_usb).as_bytes()));
        }

        // string attributes
        let lang = format!("0x{:04x}", &gadget.lang);
        let strings_path = gadget_path.join("strings").join(&lang);
        try!(create_dir_if_not_exists(&strings_path));
        try!(write_data(strings_path.join("serialnumber").as_path(),
                        gadget.serial_number.as_bytes()));
        try!(write_data(strings_path.join("manufacturer").as_path(),
                        gadget.manufacturer.as_bytes()));
        try!(write_data(strings_path.join("product").as_path(),
                        gadget.product.as_bytes()));

        // functions
        let functions_path = gadget_path.join("functions");
        try!(create_dir_if_not_exists(&functions_path));
        for func in &gadget.functions {
            try!(self.write_function(functions_path.as_path(), func));
            // try!(func.write_to(functions_path.as_path()).map_err(|e| e.to_string()));
        }

        // configs
        let configs_path = gadget_path.join("configs");
        try!(create_dir_if_not_exists(&configs_path));
        for config in &gadget.configs {
            try!(self.write_config(configs_path.as_path(),
                                   config,
                                   functions_path.as_path(),
                                   lang.as_str()));
        }

        // write UDC to enable
        try!(write_data(gadget_path.join("UDC").as_path(), self.udc_name.as_bytes()));

        Ok(())
    }

    pub fn disable(&mut self, gadget: UsbGadget) -> io::Result<()> {
        if !self.configfs_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, "ConfigFs path does not exist"));
        }

        // TODO: Implement a safe tear down

        Ok(())
    }

    fn write_function(&mut self,
                      functions_path: &Path,
                      func: &Box<UsbGadgetFunction>)
                      -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = func.function_type(),
                            instance = func.instance_name());
        let function_path = functions_path.join(fname);
        if !function_path.exists() {
            try!(fs::create_dir(&function_path));
        }
        // function attributes
        for (attr, attr_val) in &func.attributes() {
            try!(write_data(function_path.join(attr).as_path(), attr_val));
        }

        Ok(())
    }

    fn write_config(&mut self,
                    configs_path: &Path,
                    config: &UsbGadgetConfig,
                    functions_path: &Path,
                    lang: &str)
                    -> io::Result<()> {
        let config_name = format!("{name}.{id}", name = config.name, id = config.id);
        let config_path = configs_path.join(config_name);
        if !config_path.exists() {
            try!(fs::create_dir(&config_path));
        }
        let config_strings_path = config_path.join("strings").join(&lang);
        if !config_strings_path.exists() {
            try!(fs::create_dir_all(&config_strings_path));
        }

        try!(write_data(config_strings_path.join("configuration").as_path(),
                        config.description.as_bytes()));

        if let Some(max_power) = config.max_power {
            try!(write_data(config_path.join("MaxPower").as_path(),
                            format!("{}", max_power).as_bytes()));
        }

        // symlink config functions
        for func in &config.functions {
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
