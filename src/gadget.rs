use std::io;
use std::path::Path;

use function::UsbGadgetFunction;
use config::UsbGadgetConfig;
use util::{create_dir_if_not_exists, write_data};

// USB Language Identifiers (LANGIDs)
// http://www.usb.org/developers/docs/USB_LANGIDs.pdf
pub const LANGID_EN_US: u16 = 0x0409;

pub struct UsbGadget<'a> {
    pub name: &'a str,
    pub vendor_id: u16,
    pub product_id: u16,
    pub lang: u16,
    pub product: &'a str,
    pub manufacturer: &'a str,
    pub serial_number: &'a str,
    // attribute names
    pub bcd_usb: Option<u16>,
    pub bcd_device: Option<u16>,
    pub device_class: Option<u8>,
    pub device_subclass: Option<u8>,
    pub device_protocol: Option<u8>,
    // functions
    pub functions: Vec<Box<UsbGadgetFunction>>,
    // configurations
    pub configs: Vec<UsbGadgetConfig<'a>>,
}

impl<'a> UsbGadget<'a> {
    pub fn new(name: &'a str,
               vendor_id: u16,
               product_id: u16,
               lang: u16,
               product: &'a str,
               manufacturer: &'a str,
               serial_number: &'a str)
               -> UsbGadget<'a> {
        UsbGadget {
            name: name,
            vendor_id: vendor_id,
            product_id: product_id,
            lang: lang,
            product: product,
            manufacturer: manufacturer,
            serial_number: serial_number,
            bcd_usb: None,
            bcd_device: None,
            device_class: None,
            device_subclass: None,
            device_protocol: None,
            functions: Vec::new(),
            configs: Vec::new(),
        }
    }

    pub fn write_to(&self, gadget_path: &Path) -> io::Result<()> {
        try!(create_dir_if_not_exists(&gadget_path));

        // vendor and product id
        try!(write_data(gadget_path.join("idVendor").as_path(),
                        format!("0x{:04x}", self.vendor_id).as_bytes()));
        try!(write_data(gadget_path.join("idProduct").as_path(),
                        format!("0x{:04x}", self.product_id).as_bytes()));

        // bcdDevice and bcdUSB
        if let Some(bcd_device) = self.bcd_device {
            try!(write_data(gadget_path.join("bcdDevice").as_path(),
                            format!("0x{:04x}", bcd_device).as_bytes()));
        }
        if let Some(bcd_usb) = self.bcd_usb {
            try!(write_data(gadget_path.join("bcdUSB").as_path(),
                            format!("0x{:04x}", bcd_usb).as_bytes()));
        }

        // string attributes
        let lang = format!("0x{:04x}", &self.lang);
        let strings_path = gadget_path.join("strings").join(&lang);
        try!(create_dir_if_not_exists(&strings_path));
        try!(write_data(strings_path.join("serialnumber").as_path(),
                        self.serial_number.as_bytes()));
        try!(write_data(strings_path.join("manufacturer").as_path(),
                        self.manufacturer.as_bytes()));
        try!(write_data(strings_path.join("product").as_path(),
                        self.product.as_bytes()));

        // functions
        let functions_path = gadget_path.join("functions");
        try!(create_dir_if_not_exists(&functions_path));
        for func in &self.functions {
            try!(func.write_to(functions_path.as_path()));
        }

        // configs
        let configs_path = gadget_path.join("configs");
        try!(create_dir_if_not_exists(&configs_path));
        for config in &self.configs {
            try!(config.write_to(configs_path.as_path(),
                                 functions_path.as_path(),
                                 lang.as_str()));
        }

        Ok(())
    }
}
