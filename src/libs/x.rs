pub mod xcb;

pub trait XBackend
{
    fn init(&mut self) -> Result<bool, String>;
    fn connect(&mut self) -> Result<bool, String>;
}

pub fn get_xbackend() -> impl XBackend {
xcb::XCBBackend::new()
}
