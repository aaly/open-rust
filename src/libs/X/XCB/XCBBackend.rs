// #[path="../XBackend.rs"] mod XBackend;

use crate::libs::X::XBackend::XBackend;

pub struct XCBBackend 
{
}


impl XCBBackend 
{
    pub fn new() -> XCBBackend 
    {
        XCBBackend{}
    }
}

impl XBackend for XCBBackend
{

    fn Init(&mut self) -> (bool, String)
    {
        (true, String::from("Success!"))
    }
}

