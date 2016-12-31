extern crate usbg;

use std::path::{Path, PathBuf};
use std::fs;

use usbg::UsbGadget;
use usbg::UsbGadgetState;
use usbg::UsbGadgetFunction;
use usbg::UsbGadgetConfig;
use usbg::hid;
use usbg::ecm;

fn main() {
    // general setup
    let mut g1 = UsbGadget::new("g1",
                                0x1d6b, // Linux Foundation
                                0x0104, // Multifunction Composite Gadget
                                usbg::LANGID_EN_US, // LANGID English
                                "USB Armory",
                                "Inverse Path",
                                "d34db33f0123456789");
    g1.bcd_device = Some(0x0100); // version 1.0.0
    g1.bcd_usb = Some(0x0200); // USB 2.0

    // add ECM ethernet
    let ecm_function = Box::new(ecm::ECMFunction {
        instance_name: "usb0".to_owned(),
        dev_addr: "1a:55:89:a2:69:41".to_owned(),
        host_addr: "1a:55:89:a2:69:42".to_owned(),
    });
    g1.functions.push(ecm_function.clone());

    // add HID keyboard
    let hid_function = Box::new(hid::HIDFunction {
        instance_name: "usb0".to_owned(),
        protocol: hid::HID_PROTOCOL_KEYBOARD,
        subclass: hid::HID_SUBCLASS_BOOT,
        report_length: 8,
        report_desc: &hid::HID_KEYBOARD_REPORT_DESC,
    });
    g1.functions.push(hid_function.clone());

    // add configuration
    let mut c1_functions: Vec<Box<UsbGadgetFunction>> = Vec::new();
    c1_functions.push(hid_function.clone());
    c1_functions.push(ecm_function.clone());
    let c1 = UsbGadgetConfig {
        id: 1,
        name: "c".to_owned(),
        description: "USB Armory ECM + HID".to_owned(),
        max_power: Some(120),
        functions: c1_functions,
    };
    g1.configs.push(c1);

    // normally this would be done already via mount but we're just testing here
    let tmp_configfs = PathBuf::from("/tmp/configfs/usb_gadget");
    fs::create_dir_all(tmp_configfs.as_path());

    let mut usb_state = UsbGadgetState::new();
    usb_state.udc_name("someudc.hg0".to_owned());
    usb_state.configfs_path(tmp_configfs);
    match usb_state.enable(g1) {
        Ok(_) => println!("Enabled"),
        Err(e) => println!("Failed: {}", e),
    }
}
