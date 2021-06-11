mod libs;

use crate::libs::x::XBackend;

//#[path="libs/X/XCB/XCBBackend.rs"] mod XCBBackend;
fn main() 
{
    let mut xcb_backend: crate::libs::x::xcb::XCBBackend = crate::libs::x::xcb::XCBBackend::new();
    let init_result = xcb_backend.init();
    println!("Hello, world!");
}
