use function::UsbGadgetFunction;
use config::UsbGadgetConfig;

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
    pub configs: Vec<UsbGadgetConfig>,
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
}
