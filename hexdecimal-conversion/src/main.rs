use std::convert::Into;
use std::string::String;

fn main() {
    // Convert a hexadecimal string into a fixed-size array
    let bytes: [u8; 32] =
        hex!["a6bfcdf93a020e3932514040e580bb352d951f98662c37c425a646ee683d1c38"].into();

    // Print the resulting array
    println!("{:?}", bytes);

    #[macro_export]
    macro_rules! hex {
    ($($s:literal)*) => {{
        const STRINGS: &[&'static [u8]] = &[$($s.as_bytes(),)*];
        const LEN: usize = $crate::len(STRINGS);
        const RES: [u8; LEN] = $crate::decode(STRINGS);
        RES
    }};
}
}
