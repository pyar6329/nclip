use super::*;

#[test]
fn it_returns_zstd_string_when_zstd_encode_is_succeed() {
    const ORIGINAL_STR: &str =
        "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    let encoded_str = Zstd::encode(ORIGINAL_STR).to_string();
    const EXPECTED_STR: &str = "KLUv/QBYUQMASGVsbG8gd29ybGQg44GT44KT44Gr44Gh44GvIOa8ouWtlyDjgqvjgr/jgqvjg4og44ORIOOBsCDjgbQg44OTIE9sw6EgQm9tIGRpYSDDkWnDsW8g4rSwIOK1oyDitZMg4rWNICEg8J+YhA==";
    assert_eq!(encoded_str, EXPECTED_STR);
}

#[test]
fn it_returns_string_when_zstd_decode_is_succeed() {
    const ENCODED_STR: &str = "KLUv/QBYUQMASGVsbG8gd29ybGQg44GT44KT44Gr44Gh44GvIOa8ouWtlyDjgqvjgr/jgqvjg4og44ORIOOBsCDjgbQg44OTIE9sw6EgQm9tIGRpYSDDkWnDsW8g4rSwIOK1oyDitZMg4rWNICEg8J+YhA==";
    let encoded_zstd = Zstd::from(ENCODED_STR);

    let decoded_str = encoded_zstd.decode();
    const EXPECTED_STR: &str =
        "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    assert!(decoded_str.is_ok());
    assert!(decoded_str.is_ok_and(|s| s == EXPECTED_STR));
}

#[test]
fn it_returns_string_when_zstd_encode_and_decode_is_succeed() {
    const ORIGINAL_500_WORDS : &str = "Rustは、Mozillaによって開発されたシステムプログラミング向けのプログラミング言語で、C++やCといった低水準の言語と同等のパフォーマンスを提供しつつ、メモリの安全性とプログラムの並行性を高い水準でサポートすることができるモダンなプログラミング言語です。Rustは2010年に最初に公開され、その後急速に成長し、多くの開発者に支持されています。以下では、Rustの主要な特徴や用途、コミュニティ、そしてなぜRustが注目されているのかについて説明します。Rustの特徴: 1.メモリ安全性: Rustはメモリリークやデータ競合といった一般的なプログラムエラーをコンパイル時に検出するための仕組みを提供します。この特徴は、セキュリティの向上やプログラムの信頼性向上に大きく寄与しています。2.ゼロコスト抽象化: Rustは高度な抽象化を提供する一方で、それがランタイムコストに影響しないように設計されています。これにより、プログラマーは高水準の抽象化を使用できる一方、パフォーマンスを損なわずにコードを記述できます。3.パターンマッチング: Rustはパターンマッチングを豊富にサポートしており";
    let encoded_zstd = Zstd::encode(ORIGINAL_500_WORDS);
    let encoded_base64 = Base64::encode(&ORIGINAL_500_WORDS);

    assert_ne!(encoded_zstd.to_string(), encoded_base64.to_string());
    // compressed base64 by zstd is shorter than normal base64.
    assert!(encoded_zstd.to_string().len() <= encoded_base64.len());

    let decoded_str = encoded_zstd.decode();
    assert!(decoded_str.is_ok());
    assert!(decoded_str.is_ok_and(|s| s == ORIGINAL_500_WORDS));
}

#[test]
fn it_returns_same_string_when_base64_decode_and_encode_are_called() {
    const ORIGINAL_STR: &str =
        "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    let encoded_base64 = Zstd::encode(ORIGINAL_STR);
    let decoded_str = encoded_base64.decode();
    assert!(decoded_str.is_ok());
    assert!(decoded_str.is_ok_and(|s| s == ORIGINAL_STR));
}

#[test]
fn it_returns_decoded_string_when_zstd_string_decode_generated_in_dart_is_succeed() {
    const ENCODED_STR: &str =
        "KLUv/QBYUAMASGVsbG8gd29ybGQg44GT44KT44Gr44Gh44GvIOa8ouWtlyDjgqvjgr/jgqvjg4og44ORIOOBsCDjgbQg44OTIE9sw6EgQm9tIGRpYSDDkWnDsW8g4rSwIOK1oyDitZMg4rWNICEg8J+YhAEAAA==";
    let decoded_str = Zstd::from(ENCODED_STR).decode();
    const EXPECTED_STR: &str =
        "Hello world こんにちは 漢字 カタカナ パ ば ぴ ビ Olá Bom dia Ñiño ⴰ ⵣ ⵓ ⵍ ! 😄";
    assert!(decoded_str.is_ok());
    assert!(decoded_str.is_ok_and(|s| s == EXPECTED_STR));
}
