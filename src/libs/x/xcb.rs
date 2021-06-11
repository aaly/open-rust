use crate::libs::x::XBackend;

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

    fn init(&mut self) -> (bool, String)
    {
        (true, String::from("Success!"))
    }
}

