pub trait Lang {
    fn get_name(&self) -> String;
    fn translate(&self, source_en: &str) -> String;
}

pub trait Translator {
    fn translate(&self, source_en: &str, lang: &impl Lang) -> String;
}

#[derive(Default)]
pub struct BaseTranslator {}

impl Translator for BaseTranslator {
    fn translate(&self, source_en: &str, lang: &impl Lang) -> String {
        println!("translating {source_en} with lang {}", lang.get_name());
        lang.translate(source_en)
    }
}

#[derive(Default)]
pub struct InverseLang {}

impl Lang for InverseLang {
    fn translate(&self, source_en: &str) -> String {
        source_en.chars().rev().collect::<String>()
    }

    fn get_name(&self) -> String {
        "reverse lang".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inverse_lang() {
        let source = "🍄🍓🧸🧺🪞";
        let expected = "🪞🧺🧸🍓🍄";
        let lang = InverseLang::default();

        assert_eq!(expected, lang.translate(source));
    }

    #[test]
    fn check_inverse_lang_on_empty_str() {
        let source = "";
        let expected = "";
        let lang = InverseLang::default();

        assert_eq!(expected, lang.translate(source));
    }
}
