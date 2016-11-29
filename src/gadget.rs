use function::UsbGadgetFunction;
use config::UsbGadgetConfig;

// USB Language Identifiers (LANGIDs)
// http://www.usb.org/developers/docs/USB_LANGIDs.pdf
pub const LANGID_EN_US: u16 = 0x0409;

pub struct UsbGadget {
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub lang: u16,
    pub product: String,
    pub manufacturer: String,
    pub serial_number: String,
    // attribute names
    pub bcd_usb: Option<u16>,
    pub bcd_device: Option<u16>,
    pub device_class: Option<String>,
    pub device_subclass: Option<String>,
    pub device_protocol: Option<String>,
    pub max_packet_size: Option<String>,
    // functions
    pub functions: Vec<Box<UsbGadgetFunction>>,
    // configurations
    pub configs: Vec<UsbGadgetConfig>,
}

impl UsbGadget {
    pub fn new(name: &str,
               vendor_id: u16,
               product_id: u16,
               lang: u16,
               product: &str,
               manufacturer: &str,
               serial_number: &str)
               -> UsbGadget {
        UsbGadget {
            name: String::from(name),
            vendor_id: vendor_id,
            product_id: product_id,
            lang: lang,
            product: String::from(product),
            manufacturer: String::from(manufacturer),
            serial_number: String::from(serial_number),
            bcd_usb: None,
            bcd_device: None,
            device_class: None,
            device_subclass: None,
            device_protocol: None,
            max_packet_size: None,
            functions: Vec::new(),
            configs: Vec::new(),
        }
    }
}
