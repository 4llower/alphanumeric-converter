#[cfg(test)]
mod tests {
    use alphanumeric_convert_extension::*;
    
    #[test]
    fn test_convert_to_string() {
        assert_eq!(to_string("A18720928B30D38F".to_string()), "&'()*+,-./");
        assert_eq!(to_string("9F7EFC00420C4140".to_string()), "]^_ !\"#$%");
        assert_eq!(to_string("8D76DF8E7AEFC000".to_string()), "UVWXYZ[\\");
        assert_eq!(to_string("7BAFC31CB3D00000".to_string()), "NOPQRST");
        assert_eq!(to_string("6A29AABB2D000000".to_string()), "HIJKLM");
        assert_eq!(to_string("58A3925980000000".to_string()), "BCDEF");
        assert_eq!(to_string("45D8661000000000".to_string()), "789A");
        assert_eq!(to_string("3515580000000000".to_string()), "456");
        assert_eq!(to_string("2493000000000000".to_string()), "23");
        assert_eq!(to_string("1440000000000000".to_string()), "1");
        assert_eq!(to_string("dgjkdsjfldsjds".to_string()), "Incorrect Alphanumeric Number");
        assert_eq!(to_string("".to_string()), "Incorrect Alphanumeric Number");
        assert_eq!(to_string("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA".to_string()), "Incorrect Alphanumeric Number");
    }

    #[test]
    fn test_convert_from_string() {
        assert_eq!(convert_from("BCDEF".as_bytes(), 5), 0x58A3925980000000);
        assert_eq!(convert_from("&'()*+,-./".as_bytes(), 10), 0xA18720928B30D38F);
        assert_eq!(convert_from("]^_ !\"#$%".as_bytes(), 9), 0x9F7EFC00420C4140);
        assert_eq!(convert_from("UVWXYZ[\\".as_bytes(), 8), 0x8D76DF8E7AEFC000);
        assert_eq!(convert_from("asddasa".as_bytes(), 7), -1);
        assert_eq!(convert_from("".as_bytes(), 0), -1);
    }
}