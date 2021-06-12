mod libs;

use crate::libs::x::*;

fn main() -> Result<(), String>
{
    let mut xbackend = crate::libs::x::get_xbackend();
    let init_result = xbackend.init();
    
    match init_result.err()
    {
        Some(error_message) => Err(error_message),
        None => Ok(())
    }
}
