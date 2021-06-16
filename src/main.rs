mod libs;

use crate::libs::x::*;

fn main() -> Result<(), String>
{
    let mut xbackend = crate::libs::x::get_xbackend();
    let init_result = xbackend.init();
    
    match init_result.err()
    {
        Some(error) => panic!("error initializing xbackend: {}", error),
        None => {}
    };
    
    let conncet_result = xbackend.connect(None);
    
    match conncet_result.err()
    {
        Some(error) => panic!("failed to connect to x server: {}", error),
        None => {}
    };
    
    let run_result = xbackend.run();
    match run_result.err()
    {
        Some(error_message) => Err(error_message),
        None => Ok(())
    }
}
