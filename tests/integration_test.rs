use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

use ferris_says::{say, Who};

// Default width when running the binary
const DEFAULT_WIDTH: usize = 40;

fn hello_fellow_rustaceans_width_24(who: Who) -> Result<(), ()> {
    let expected = match who {
        Who::Ferris => br#"
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
        .to_vec(),
        Who::Clippy => br#"
 __________________________
< Hello fellow Rustaceans! >
 --------------------------
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
        .to_vec(),
    };

    let input = "Hello fellow Rustaceans!";
    let width = 24;

    let actual = say(input, width, who).unwrap_or_else(|_| panic!("can't say"));

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

fn hello_fellow_rustaceans_width_12(who: Who) -> Result<(), ()> {
    let expected = match who {
        Who::Ferris => br#"
 ______________
/ Hello fellow \
\ Rustaceans!  /
 --------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
        .to_vec(),
        Who::Clippy => br#"
 ______________
/ Hello fellow \
\ Rustaceans!  /
 --------------
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
        .to_vec(),
    };

    let input = "Hello fellow Rustaceans!";
    let width = 12;

    let actual = say(input, width, who).unwrap_or_else(|_| panic!("can't say"));

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

fn hello_fellow_rustaceans_width_6(who: Who) -> Result<(), ()> {
    //Hello fellow Rustaceans!
    #[cfg(not(feature = "clippy"))]
    let expected = match who {
        Who::Ferris => br#"
 ________
/ Hello  \
| fellow |
| Rustac |
\ eans!  /
 --------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
        .to_vec(),
        Who::Clippy => br#"
 ________
/ Hello  \
| fellow |
| Rustac |
\ eans!  /
 --------
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
        .to_vec(),
    };

    let input = "Hello fellow Rustaceans!";
    let width = 6;

    let actual = say(input, width, who).unwrap_or_else(|_| panic!("can't say"));

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

fn hello_fellow_rustaceans_width_3(who: Who) -> Result<(), ()> {
    let expected = match who {
        Who::Ferris => br#"
 _____
/ Hel \
| lo  |
| fel |
| low |
| Rus |
| tac |
| ean |
\ s!  /
 -----
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
        .to_vec(),
        Who::Clippy => br#"
 _____
/ Hel \
| lo  |
| fel |
| low |
| Rus |
| tac |
| ean |
\ s!  /
 -----
        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
        .to_vec(),
    };

    let input = "Hello fellow Rustaceans!";
    let width = 3;

    let actual = say(input, width, who).unwrap_or_else(|_| panic!("can't say"));

    assert_eq!(std::str::from_utf8(&expected[1..]).unwrap(), actual);
    Ok(())
}

fn multibyte_string(who: Who) -> Result<(), ()> {
    #[rustfmt::skip]
    let expected = match who {
    Who::Ferris => concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n",
        r#"        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#
        ),
        Who::Clippy => concat!(
        " ____________\n",
        "< 突然の死👻 >\n",
        " ------------\n",
        r#"        \
         \
            __
           /  \
           |  |
           @  @
           |  |
           || |/
           || ||
           |\_/|
           \___/
"#
        ),
    };

    let input = "突然の死👻";
    let width = DEFAULT_WIDTH;

    let actual = say(input, width, who).unwrap_or_else(|_| panic!("can't say"));

    assert_eq!(std::str::from_utf8(&expected.as_bytes()).unwrap(), actual);
    Ok(())
}

macro_rules! tests {
    ($($name:ident),*) => {
        $(
            paste::paste! {
                #[wasm_bindgen_test::wasm_bindgen_test]
                fn [<test_ $name _with_ferris>]() {
                    $name(Who::Ferris).unwrap();
                }

                #[wasm_bindgen_test::wasm_bindgen_test]
                fn [<test_ $name _with_clippy>]() {
                    $name(Who::Clippy).unwrap();
                }
            }
        )*
    };
}

tests!(
    hello_fellow_rustaceans_width_24,
    hello_fellow_rustaceans_width_12,
    hello_fellow_rustaceans_width_6,
    hello_fellow_rustaceans_width_3,
    multibyte_string
);
