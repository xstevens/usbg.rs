use function::UsbGadgetFunction;

pub struct UsbGadgetConfig<'a> {
    pub id: u8,
    pub name: &'a str,
    pub description: &'a str,
    pub functions: Vec<Box<UsbGadgetFunction>>,
    pub max_power: Option<u16>,
}
