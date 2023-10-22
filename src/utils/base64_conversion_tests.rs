use super::*;

#[test]
fn it_returns_base64_string_when_encode_is_succeed() {
    let s = "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    let encoded_str = Base64::encode(&s).to_string();
    let expected_str = "SGVsbG8gd29ybGQg44GT44KT44Gr44Gh44GvIOa8ouWtlyDjgqvjgr/jgqvjg4og44ORIOOBsCDjgbQg44OTIE9sw6EgQm9tIGRpYSDDkWnDsW8g4rSwIOK1oyDitZMg4rWNICEg8J+YhA==";
    assert_eq!(&encoded_str, expected_str);
}

#[test]
fn it_returns_string_when_decode_is_succeed() {
    let encoded_str = "SGVsbG8gd29ybGQg44GT44KT44Gr44Gh44GvIOa8ouWtlyDjgqvjgr/jgqvjg4og44ORIOOBsCDjgbQg44OTIE9sw6EgQm9tIGRpYSDDkWnDsW8g4rSwIOK1oyDitZMg4rWNICEg8J+YhA==";
    let base64_str = Base64::from(encoded_str);
    let decoded_str = Base64::decode(&base64_str);
    let decoded_str2 = base64_str.decode();
    let expected_str =
        "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    assert_eq!(&decoded_str, &decoded_str2);
    assert!(decoded_str.is_ok());
    assert!(decoded_str.is_ok_and(|s| s == expected_str));
}
