use fluent::{bundle::FluentBundle, FluentResource};
use intl_memoizer::concurrent::IntlLangMemoizer;
use unic_langid::langid;

pub type TranslationType = FluentBundle<FluentResource, IntlLangMemoizer>;

pub async fn load_langs() -> TranslationType {
    let ftl_string = include_str!("../assets/translations/ru-RU.ftl").to_string();
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let langid_ru = langid!("ru-RU");
    let mut lang_bundle: TranslationType = FluentBundle::new_concurrent(vec![langid_ru]);

    {
        lang_bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");
    }

    lang_bundle
}
