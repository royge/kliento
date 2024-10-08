use std::error::Error;
use calamine::{DataType, Range};

pub trait Extraction<T> {
    fn extract(&mut self, sheet: Range<DataType>) -> Result<(), Box<dyn Error>>;
}

pub trait Counter<T> {
    fn count(&self) -> usize;
}
