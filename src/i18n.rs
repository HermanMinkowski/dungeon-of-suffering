use fluent_bundle::{FluentBundle, FluentResource, FluentArgs};
use unic_langid::LanguageIdentifier;
use std::sync::Arc;

pub struct I18n {
    bundle: Arc<FluentBundle<FluentResource>>,
}

impl I18n {
    pub fn new(locale: &str, ftl_source: &str) -> Self {
        let langid: LanguageIdentifier = locale.parse().expect("Invalid locale");
        let mut bundle = FluentBundle::new(vec![langid]);
        let res = FluentResource::try_new(ftl_source.to_string())
            .map_err(|e| {
                eprintln!("FTL Parse Error: {:?}", e);
                e
            })
            .expect("Failed to parse FTL");
        bundle.add_resource(res).expect("Failed to add FTL resources");
        Self { bundle: Arc::new(bundle) }
    }

    pub fn t(&self, key: &str, args: Option<&FluentArgs>) -> String {
        if let Some(msg) = self.bundle.get_message(key) {
            if let Some(pattern) = msg.value() {
                let mut errors = vec![];
                let value = self.bundle.format_pattern(pattern, args, &mut errors);
                return value.to_string();
            }
        }
        key.to_string()
    }

    pub fn with_english() -> Self {
        let ftl_source = include_str!("../locales/en.ftl");
        Self::new("en", ftl_source)
    }

    pub fn with_french() -> Self {
        let ftl_source = include_str!("../locales/fr.ftl");
        Self::new("fr", ftl_source)
    }
}

