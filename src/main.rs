use std::fmt;

fn main() {
    // Similar to the Solidity state variables, these variables hold the data to be worked with.
    // However, unlike Solidity, these are not stored on the blockchain but in the program's memory.
    let my_string = String::from("Hello, World!");
    let my_uint = 12345;

    // These function calls and println! statements are analogous to calling the Solidity functions
    // and returning the values from them. The Rust program outputs the results to the console.
    println!("Bytes representation of string: {:?}", ByteVec(get_string_bytes(&my_string)));
    println!("Bytes representation of uint: {:?}", ByteVec(get_uint_bytes(my_uint).to_vec()));
    println!("Hex representation of string: {}", get_string_hex(&my_string));
    println!("Hex representation of uint: {}", get_uint_hex(my_uint));
}

// Analogous to Solidity's getStringBytes function, this function converts a string to bytes.
fn get_string_bytes(s: &str) -> Vec<u8> {
    s.as_bytes().to_vec()
}

// Analogous to Solidity's getUintBytes function, this function converts a uint to bytes.
fn get_uint_bytes(u: u64) -> [u8; 8] {
    u.to_be_bytes()
}

// Analogous to Solidity's getStringHex function, this function converts a string to a hexadecimal string.
fn get_string_hex(s: &str) -> String {
    s.as_bytes().iter().map(|byte| format!("{:02x}", byte)).collect()
}

// Analogous to Solidity's getUintHex function, this function converts a uint to a hexadecimal string.
fn get_uint_hex(u: u64) -> String {
    format!("{:x}", u)
}

// ByteVec is a custom wrapper type for Vec<u8>, used to provide a custom Debug implementation
// to format the bytes in a manner similar to Solidity's hexadecimal representation.
struct ByteVec(Vec<u8>);

impl fmt::Debug for ByteVec {
    // This custom Debug implementation is similar to how Solidity automatically
    // formats bytes types as hexadecimal strings prefixed with "0x".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "0x{}",
            self.0
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<String>>()
                .concat()
        )
    }
}


