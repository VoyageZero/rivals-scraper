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
        let row_sel = Selector::parse("tbody tr").unwrap();
        let num_sel = Selector::parse("td > i > b").unwrap();
        let link_sel = Selector::parse("td > b > a").unwrap();
        let id_sel = Selector::parse("td > span > b").unwrap();
    }
}
