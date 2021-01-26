#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn decode<T: AsRef<[u8]>>(_input: T) -> Result<usize, DecodeError> {
    todo!()
}

#[cfg_attr(feature = "std", derive(Debug, thiserror::Error))]
pub enum DecodeError {}
