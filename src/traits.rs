use crate::error::OverlayError;

pub trait SolidPatch<T>
where
    T: AsRef<[u8]>, Self: Sized
{
    fn new(offset: u64, content: T) -> Result<Self, OverlayError>;
}

pub trait Contains {
    fn contains(&self, offset: u64) -> bool;
}
