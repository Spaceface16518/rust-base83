#[cfg(any(feature = "alloc", feature = "std", test))]
pub fn encode(_input: usize) -> Result<String, EncodeError> {
    todo!()
}

#[cfg_attr(feature = "std", derive(Debug, thiserror::Error))]
pub enum EncodeError {}
