use function::UsbGadgetFunction;

pub struct UsbGadgetConfig {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub functions: Vec<Box<UsbGadgetFunction>>,
}
