//! The PHP functions the port depends on, against PHP's own answers
//! (scripts/laravel-reference/php-functions.php).

use nicolaskion::esi::round_one_decimal;

use crate::common::golden;

/// `round($x, 1)` for the security status: half away from zero on the
/// decimal value, so 0.35 is 0.4 and 1.05 is 1.1, and the sign of a
/// negative zero survives.
#[test]
fn security_status_rounds_like_php() {
    let cases = golden::read_json("php-functions.json")["round_1"]
        .as_array()
        .expect("cases")
        .clone();
    assert!(cases.len() > 20);
    for case in cases {
        let input = case["input"].as_f64().expect("input");
        let expected = case["output"].as_f64().expect("output");
        let actual = round_one_decimal(input);
        assert_eq!(actual, expected, "round({input}, 1)");
        assert_eq!(
            actual.is_sign_negative(),
            expected.is_sign_negative(),
            "sign of round({input}, 1)"
        );
    }
}
