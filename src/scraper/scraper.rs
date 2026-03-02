use reqwest;
use scraper::{Html, Selector};
use tokio::time::{Duration, sleep};

pub struct Scraper {
    client: reqwest::Client,
    base_url: String,
    delay_ms: u64,
}

impl Scraper {
    pub fn new(base_url: &str, delay_ms: u64) -> Self {
        Scraper {
            client: reqwest::Client::new(),
            base_url: base_url.to_string(),
            delay_ms,
        }
    }

    pub async fn fetch_page(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.base_url, path);
        let body = self.client.get(&url).send().await?.text().await?;

        sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(body)
    }

    pub fn parse(&self, html: &str, selector: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse(selector).unwrap();

        document
            .select(&selector)
            .map(|el| el.text().collect::<String>())
            .collect()
    }

    pub fn parse_table(&self, html: &str, table_selector: &str) -> Vec<Vec<String>> {
        let document = Html::parse_document(html);
        let table_sel = Selector::parse(table_selector).unwrap();
        let row_sel = Selector::parse("tr").unwrap();
        let cell_sel = Selector::parse("td, th").unwrap();

        document
            .select(&table_sel)
            .flat_map(|table| {
                table.select(&row_sel).map(|row| {
                    row.select(&cell_sel)
                        .map(|cell| cell.text().collect::<String>().trim().to_string())
                        .collect::<Vec<String>>()
                })
            })
            .collect()
    }
}
