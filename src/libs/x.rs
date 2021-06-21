pub mod xcb;

pub enum XEventType 
{
	ConfigureNotify,
	ClientMessage,
	ReparentNotify,
	CreateNotify,
	MapRequest,
	DestroyNotify,
	KeyRelease,
	KeyPress,
	ButtonPress,
	ButtonRelease,
	MotionNotify,
}

pub trait XEvent
{
	fn get_event_type(&mut self) -> XEventType;
}

#[derive(Debug, Copy, Clone)]
pub struct ConfigureNotifyXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct ClientMessageXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct ReparentNotifyXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct CreateNotifyXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct MapRequestXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct DestroyNotifyXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct KeyReleaseXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct KeyPressXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct ButtonPressXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct ButtonReleaseXEvent {}
#[derive(Debug, Copy, Clone)]
pub struct MotionNotifyXEvent {}


impl XEvent for ConfigureNotifyXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for ClientMessageXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for ReparentNotifyXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for CreateNotifyXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for MapRequestXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for DestroyNotifyXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for KeyReleaseXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for KeyPressXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for ButtonPressXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for ButtonReleaseXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

impl XEvent for MotionNotifyXEvent
{
	fn get_event_type(&mut self) -> XEventType
	{
		XEventType::ConfigureNotify
	}
}

pub trait XBackend
{
    fn init(&mut self) -> Result<bool, String>;
    fn connect(&mut self, display: Option<&str>) -> Result<bool, String>;
    fn get_window_id(&mut self, screen_num: usize) -> u32;
    fn run(&mut self) -> Result<(), String>;
    
    fn get_receiver(&mut self) -> &std::sync::mpsc::Receiver<Box<dyn XEvent>>;
}

pub fn get_xbackend() -> impl XBackend
{
xcb::XCBBackend::new()
}



