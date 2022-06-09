use any_ascii::any_ascii;
use sys_locale::get_locale;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AppLanguage {
    DE,
    EN
}

/// Eliminate non-ASCII characters.
/// Replace common german special characters with their matching counterparts.
/// The parameter is expected to be lower case.
pub fn replace_unicode(word: &str, app_language: AppLanguage) -> String {
    match app_language {
        AppLanguage::DE => {
            // replace umlauts for german language
            let without_umlauts = replace_umlauts(word);

            any_ascii(without_umlauts.as_str())
        }
        _ => any_ascii(word)
    }
}

/// Replace german umlaut characters with their logical counterparts. The parameter is expected to be lowercase.
///
/// * 'ä' -> "ae"
/// * 'ö' -> "oe"
/// * 'ü' -> "ue"
fn replace_umlauts(word: &str) -> String {
    word.replace("ä", "ae").replace("ö", "oe").replace("ü", "ue")
}

pub fn get_app_language() -> AppLanguage {
    match get_locale() {
        Some(locale) => parse_app_language(locale.as_str()),
        None => AppLanguage::EN
    }
}

fn parse_app_language(locale_str: &str) -> AppLanguage {
    match locale_str {
        "de" => AppLanguage::DE,
        "de-DE" => AppLanguage::DE,
        "de-AT" => AppLanguage::DE,
        _ => AppLanguage::EN
    }
}

#[cfg(test)]
#[test]
fn test_replace_unicode() {
    assert_eq!(replace_unicode("schön", AppLanguage::DE), "schoen");
    assert_eq!(replace_unicode("geschoß", AppLanguage::DE), "geschoss");
    assert_eq!(replace_unicode("zäh", AppLanguage::DE), "zaeh");
    assert_eq!(replace_unicode("lüge", AppLanguage::DE), "luege");

    assert_eq!(replace_unicode("schön", AppLanguage::EN), "schon");
    assert_eq!(replace_unicode("geschoß", AppLanguage::EN), "geschoss");
    assert_eq!(replace_unicode("zäh", AppLanguage::EN), "zah");
    assert_eq!(replace_unicode("lüge", AppLanguage::EN), "luge");
}

#[cfg(test)]
#[test]
fn test_replace_umlauts() {
    assert_eq!(replace_umlauts("schön"), "schoen");
    assert_eq!(replace_umlauts("zäh"), "zaeh");
    assert_eq!(replace_umlauts("lüge"), "luege");

    assert_ne!(replace_umlauts("schön"), "schon");
    assert_ne!(replace_umlauts("zäh"), "zah");
    assert_ne!(replace_umlauts("lüge"), "luge");
}

#[cfg(test)]
#[test]
fn test_parse_app_language() {
    assert_eq!(parse_app_language("de"), AppLanguage::DE);
    assert_eq!(parse_app_language("de-AT"), AppLanguage::DE);
    assert_eq!(parse_app_language("de-DE"), AppLanguage::DE);
    assert_eq!(parse_app_language("en"), AppLanguage::EN);
    assert_eq!(parse_app_language("it"), AppLanguage::EN);
    assert_eq!(parse_app_language(""), AppLanguage::EN);

    assert_ne!(parse_app_language("de-DE"), AppLanguage::EN);
    assert_ne!(parse_app_language("it"), AppLanguage::DE);
    assert_ne!(parse_app_language(""), AppLanguage::DE);
}