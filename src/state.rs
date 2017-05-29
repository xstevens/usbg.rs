use std::fs;
use std::io;
use std::path::PathBuf;

use UsbGadget;
use util::write_data;

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

        // write gadget to ConfigFs
        let gadget_path = self.configfs_path.join(gadget.name);
        gadget.write_to(&gadget_path)?;

        // write UDC to enable
        write_data(gadget_path.join("UDC").as_path(), self.udc_name.as_bytes())?;

        Ok(())
    }

    pub fn disable(&mut self, gadget: UsbGadget) -> io::Result<()> {
        if !self.configfs_path.exists() {
            return Err(io::Error::new(io::ErrorKind::Other, "ConfigFs path does not exist"));
        }

        // TODO: Implement a safe tear down

        Ok(())
    }
}
