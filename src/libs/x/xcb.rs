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

    fn init(&mut self) -> Result<bool, String>
    {
	Ok(true)
    }

    fn connect(&mut self) -> Result<bool, String>
    {
        Ok(true)
    }
}

