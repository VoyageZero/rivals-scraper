use super::scraper::Scraper;
use scraper::{Html, Selector};

pub struct HeroLink {
    pub id: u32,
    pub hero_name: String,
    pub path: String,
}

impl Scraper {
    pub fn scrape_hero_links(&self, html: &str) -> Vec<HeroLink> {
        let doc = Html::parse_document(html);
        let tbl_sel = Selector::parse("table.fandom-table").unwrap();
    }
}
