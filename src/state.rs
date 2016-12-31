use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::io::Write;
use std::os::unix;
use UsbGadget;
use UsbGadgetFunction;
use UsbGadgetConfig;

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
                    println!("{:?}", entry.file_name());
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

    pub fn udc_name<'a>(&'a mut self, udc_name: String) -> &'a mut UsbGadgetState {
        self.udc_name = udc_name.clone();
        self
    }

    pub fn enable(&mut self, gadget: UsbGadget) -> Result<bool, String> {
        if !self.configfs_path.exists() {
            return Err("ConfigFs base path does not exist".to_owned());
        }

        let gadget_path = self.configfs_path.join(gadget.name);
        try!(fs::create_dir(&gadget_path).map_err(|e| e.to_string()));

        // vendor and product id
        try!(self.write_data(gadget_path.join("idVendor").as_path(),
                        format!("{:#x}", gadget.vendor_id).as_bytes())
            .map_err(|e| e.to_string()));
        try!(self.write_data(gadget_path.join("idProduct").as_path(),
                        format!("{:#x}", gadget.product_id).as_bytes())
            .map_err(|e| e.to_string()));

        // bcdDevice and bcdUSB
        if let Some(bcd_device) = gadget.bcd_device {
            try!(self.write_data(gadget_path.join("bcdDevice").as_path(),
                            format!("{:#x}", bcd_device).as_bytes())
                .map_err(|e| e.to_string()));
        }
        if let Some(bcd_usb) = gadget.bcd_usb {
            try!(self.write_data(gadget_path.join("bcdUSB").as_path(),
                            format!("{:#x}", bcd_usb).as_bytes())
                .map_err(|e| e.to_string()));
        }

        // string attributes
        let lang = format!("{:#x}", &gadget.lang);
        let strings_path = gadget_path.join("strings").join(&lang);
        try!(fs::create_dir_all(&strings_path).map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("serialnumber").as_path(),
                        gadget.serial_number.as_bytes())
            .map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("manufacturer").as_path(),
                        gadget.manufacturer.as_bytes())
            .map_err(|e| e.to_string()));
        try!(self.write_data(strings_path.join("product").as_path(),
                        gadget.product.as_bytes())
            .map_err(|e| e.to_string()));

        // functions
        let functions_path = gadget_path.join("functions");
        try!(fs::create_dir(&functions_path).map_err(|e| e.to_string()));
        for func in &gadget.functions {
            try!(self.write_function(functions_path.as_path(), func).map_err(|e| e.to_string()));
            // try!(func.write_to(functions_path.as_path()).map_err(|e| e.to_string()));
        }

        // configs
        let configs_path = gadget_path.join("configs");
        try!(fs::create_dir(&configs_path).map_err(|e| e.to_string()));
        for config in &gadget.configs {
            try!(self.write_config(configs_path.as_path(),
                              config,
                              functions_path.as_path(),
                              lang.as_str())
                .map_err(|e| e.to_string()));
        }

        // write UDC to enable
        // TODO: This commented section seems not possible at the moment with the compiler
        // getting overly aggressive with borrow checking.
        // https://github.com/rust-lang/rfcs/issues/811
        // try!(self.write_data(gadget_path.join("UDC"), self.udc_name.as_bytes())
        //          .map_err(|e| e.to_string()));
        // duplicating write_data logic here to move on
        let mut f = try!(fs::File::create(gadget_path.join("UDC")).map_err(|e| e.to_string()));
        try!(f.write_all(self.udc_name.as_bytes()).map_err(|e| e.to_string()));

        return Ok(true);
    }

    pub fn disable(&mut self, gadget: UsbGadget) -> Result<bool, String> {
        if !self.configfs_path.exists() {
            return Err("ConfigFs base path does not exist".to_owned());
        }

        // TODO: Implement a safe tear down

        return Ok(true);
    }

    fn write_data(&mut self, output_path: &Path, data: &[u8]) -> io::Result<()> {
        let mut f = try!(fs::File::create(output_path));
        try!(f.write_all(data));
        return Ok(());
    }

    fn write_function(&mut self,
                      functions_path: &Path,
                      func: &Box<UsbGadgetFunction>)
                      -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = func.function_type(),
                            instance = func.instance_name());
        let function_path = functions_path.join(fname);
        try!(fs::create_dir(&function_path));
        // function attributes
        for (attr, attr_val) in &func.attributes() {
            try!(self.write_data(function_path.join(attr).as_path(), attr_val));
        }

        return Ok(());
    }

    fn write_config(&mut self,
                    configs_path: &Path,
                    config: &UsbGadgetConfig,
                    functions_path: &Path,
                    lang: &str)
                    -> io::Result<()> {
        let config_name = format!("{name}.{id}", name = config.name, id = config.id);
        let config_path = configs_path.join(config_name);
        try!(fs::create_dir(&config_path));
        let config_strings_path = config_path.join("strings").join(&lang);
        try!(fs::create_dir_all(&config_strings_path));

        try!(self.write_data(config_strings_path.join("configuration").as_path(),
                             config.description.as_bytes()));

        if let Some(max_power) = config.max_power {
            try!(self.write_data(config_path.join("MaxPower").as_path(),
                                 format!("{}", max_power).as_bytes()));
        }

        // symlink config functions
        for func in &config.functions {
            let fname = format!("{func_type}.{instance}",
                                func_type = func.function_type(),
                                instance = func.instance_name());
            let src_path = functions_path.join(&fname);
            let dst_path = config_path.join(&fname);
            try!(unix::fs::symlink(&src_path, &dst_path));
        }

        return Ok(());
    }
}
