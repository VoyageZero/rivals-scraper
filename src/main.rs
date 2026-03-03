use dotenvy::dotenv;
use rivals_scraper::Db;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = Db::new().await.expect("Failed to connect to database.");
    println!("Connected to Database!");
}
