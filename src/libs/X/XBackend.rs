pub trait XBackend
{
    fn Init(&mut self) -> (bool, String);
}
