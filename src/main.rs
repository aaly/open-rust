mod libs;

use crate::libs::X::XCB::XCBBackend::XCBBackend;
use crate::libs::X::XBackend::XBackend;
//use crate::libs::*;

//#[path="libs/X/XCB/XCBBackend.rs"] mod XCBBackend;
fn main() 
{
    let mut xcb_backend: libs::X::XCB::XCBBackend::XCBBackend = libs::X::XCB::XCBBackend::XCBBackend::new();
    let initResult = xcb_backend.Init();
    println!("Hello, world!");
}
