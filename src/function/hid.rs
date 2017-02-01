extern crate std;

use std::fs;
use std::io;
use std::path::Path;

use UsbGadgetFunction;
use util::write_data;

// HID 1.11 Specification http://www.usb.org/developers/hidpage/HID1_11.pdf
// HID 1.11 Section 4.1: The HID Class
pub const HID_CLASS: u8 = 3;
// HID 1.11 Section 4.2: Subclass
pub const HID_SUBCLASS_BOOT: u8 = 1;
// HID 1.11 Section 4.3: Protocols
pub const HID_PROTOCOL_KEYBOARD: u8 = 1;
pub const HID_PROTOCOL_MOUSE: u8 = 2;
// HID 1.11 Appendix E.6 Report Descriptor (Keyboard); also see Appendix B.1
pub static HID_KEYBOARD_REPORT_DESC: [u8; 63] =
    [0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7, 0x15, 0x00, 0x25,
     0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08, 0x81, 0x03, 0x95, 0x05,
     0x75, 0x01, 0x05, 0x08, 0x19, 0x01, 0x29, 0x05, 0x91, 0x02, 0x95, 0x01, 0x75, 0x03, 0x91,
     0x03, 0x95, 0x06, 0x75, 0x08, 0x15, 0x00, 0x25, 0x65, 0x05, 0x07, 0x19, 0x00, 0x29, 0x65,
     0x81, 0x00, 0xC0];

#[derive(Clone)]
pub struct HIDFunction<'a> {
    pub instance_name: &'a str,
    pub protocol: u8,
    pub subclass: u8,
    pub report_length: u32,
    pub report_desc: &'a [u8],
}

impl<'a> HIDFunction<'a> {
    pub fn new(instance_name: &'a str,
               protocol: u8,
               subclass: u8,
               report_length: u32,
               report_desc: &'a [u8])
               -> HIDFunction<'a> {
        HIDFunction {
            instance_name: instance_name,
            protocol: protocol,
            subclass: subclass,
            report_length: report_length,
            report_desc: report_desc,
        }
    }
}

impl<'a> UsbGadgetFunction for HIDFunction<'a> {
    fn instance_name(&self) -> &str {
        return self.instance_name;
    }

    fn function_type(&self) -> &str {
        return "hid";
    }

    fn write_to(&self, functions_path: &Path) -> io::Result<()> {
        let fname = format!("{func_type}.{instance}",
                            func_type = self.function_type(),
                            instance = self.instance_name());
        let function_path = functions_path.join(fname);
        try!(fs::create_dir(&function_path));
        // function attributes
        try!(write_data(function_path.join("protocol").as_path(),
                        format!("{}", self.protocol).as_bytes()));
        try!(write_data(function_path.join("subclass").as_path(),
                        format!("{}", self.subclass).as_bytes()));
        try!(write_data(function_path.join("report_length").as_path(),
                        format!("{}", self.report_length).as_bytes()));
        try!(write_data(function_path.join("report_desc").as_path(),
                        self.report_desc));

        return Ok(());
    }
}
