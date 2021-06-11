pub mod xcb;

pub trait XBackend
{
    fn init(&mut self) -> (bool, String);
}
