#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: usize| {
    let encoded = base83::encode(data).unwrap();
    let decoded = base83::decode(encoded).unwrap();
    assert_eq!(data, decoded)
});
