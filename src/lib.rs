#[cfg(all(feature = "alloc", not(any(feature = "std", test))))]
extern crate alloc;
#[cfg(any(feature = "std", test))]
extern crate std as alloc;

mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;
