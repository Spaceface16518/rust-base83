#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode<T: AsRef<[u8]>>(_input: T) -> Result<String, EncodeError> {
    todo!()
}

#[cfg_attr(feature = "std", derive(Debug, thiserror::Error))]
pub enum EncodeError {}
