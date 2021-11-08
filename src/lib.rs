use wasm_bindgen::prelude::*;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const INT64_NULL_VALUE: i128 = i128::MIN;
const ALPHANUM_10_NUM_LENGTH_BITS: i128 = 4;
const ALPHANUM10_MAX_LENGTH: i128 = 10;
const SPACE_CODE: u8 = ' ' as u8;
const INCORRECT: &str = "Incorrect:(";

#[wasm_bindgen]
pub fn to_string(original: String) -> String {
    let parsed = i128::from_str_radix(&original.trim(), 16);

    if parsed.is_err() {
        return INCORRECT.to_string();
    }

    let value: i128 = parsed.unwrap();

    if value == INT64_NULL_VALUE {
        return INCORRECT.to_string();
    } 

    let len = value >> (8 * 8 - ALPHANUM_10_NUM_LENGTH_BITS);

    if len == ALPHANUM10_MAX_LENGTH + 1 {
        return INCORRECT.to_string();
    }

    return convert_to(value);
}

#[wasm_bindgen]
pub fn from_string(original: String) -> String {
    let length = original.len();    
    let value = (original.trim()).as_bytes();

    if length > 10 {
        return INCORRECT.to_string();
    }

    let result: i128 = convert_from(value, length as u8);

    if result == -1 {
        return INCORRECT.to_string();
    }

    return format!("{:#X}",result);
}

pub fn convert_to(value: i128) -> String {
    let mut result: String = "".to_string();

    for i in 1..(ALPHANUM10_MAX_LENGTH + 1) {
        let current: i128 = (value >> ((60 - 6 * i) as i128)) & 0x3F;
        result.push(((current as u8) + SPACE_CODE) as char);
    }

    return result.trim().to_string();
}

pub fn convert_from(value: &[u8], length: u8) -> i128 {
    let mut acc: i128 = 0;
    let mut ch;
    let mut check: u8 = 0;

    if length == 0 {
        return -1;
    }

    for i in (0..length).rev() {
        ch = value[i as usize] - SPACE_CODE;
        check |= ch;
        acc |= ((ch & 0x3f) as i128) << (9 - i) * 6;
    }

    if check >> 6 != 0 {
        return -1;
    }

    return ((length as i128) << (8 * 8 - ALPHANUM_10_NUM_LENGTH_BITS)) + acc;
}
