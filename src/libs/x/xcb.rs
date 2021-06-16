use crate::libs::x::XBackend;
use crate::libs::x::XEventType;
use crate::libs::x::XEvent;

use std::sync::mpsc::*;
use std::boxed::Box;

pub struct XCBBackend
{
    default_screen_num: i32,
    conn: Option<xcb::base::Connection>,
    sender: std::sync::mpsc::Sender<Box<dyn XEvent>>,
    receiver: std::sync::mpsc::Receiver<Box<dyn XEvent>>,
}

impl XCBBackend
{
    pub fn new() -> XCBBackend
    {
		let (temp_sender, temp_receiver) = channel::<Box<dyn XEvent>>();
        let result = XCBBackend
        {
            default_screen_num: -1,
            conn: None,
            receiver: temp_receiver,
            sender: temp_sender,
        };
        return result;
    }
	
	fn get_connection(&mut self) -> &xcb::Connection
	{
		self.conn.as_ref().unwrap()
	}
}

impl XBackend for XCBBackend
{

    fn init(&mut self) -> Result<bool, String>
    {
		Ok(true)
    }

    fn connect(&mut self, display: Option<&str>) -> Result<bool, String>
    {
        let(conn, default_screen_num) = xcb::Connection::connect(display).unwrap();
        self.conn = Some(conn);
        self.default_screen_num = default_screen_num;
        Ok(true)
    }
    
    fn get_window_id(&mut self, screen_num: usize) -> u32
    {
        let setup = self.conn.as_ref().unwrap().get_setup();
        let screen = setup.roots().nth(screen_num).unwrap();
        let window = screen.root();
        return window;
    }
    
    fn get_receiver(&mut self) -> &std::sync::mpsc::Receiver<Box<dyn XEvent>>
    {
		&self.receiver
	}
    
	
    fn run(&mut self) -> Result<bool, String>
    {
		let root_window = self.get_window_id(self.default_screen_num as usize);
		
		loop 
		{
			let event = XCBBackend::get_connection(self).wait_for_event();
			match event {
				// None is returned in case of I/O error; we'll just bail in that case
				None => { break; }
				Some(event) => 
				{
					let r = event.response_type() & !0x80;
					match r 
					{
						xcb::BUTTON_PRESS => 
						{
							continue;
						}
						xcb::MOTION_NOTIFY => 
						{
							let mne: &xcb::MotionNotifyEvent  = unsafe {
								xcb::cast_event(&event)
							};

							continue;
						}
						xcb::BUTTON_RELEASE => 
						{
							continue;
						}
						xcb::KEY_PRESS => 
						{
							continue;
						}
						xcb::KEY_RELEASE => 
						{
							continue;
						}
						xcb::DESTROY_NOTIFY => 
						{
							continue;
						}
						xcb::MAP_REQUEST => 
						{
							continue;
						}
						xcb::CREATE_NOTIFY => 
						{
							continue;
						}
						xcb::REPARENT_NOTIFY => 
						{
							continue;
						}
						xcb::CLIENT_MESSAGE => 
						{
							continue;
						}
						xcb::CONFIGURE_NOTIFY => 
						{
							continue;
						}
						_ => {}
					}
				}
			}
		}
		
		print!("done loop");
		
		Ok(true)
	}

}

