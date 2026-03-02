use rivals_scraper::Scraper;

#[tokio::main]
async fn main() {
    let scraper = Scraper::new("https://marvelrivals.fandom.com/wiki", 2000);
}
