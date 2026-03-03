#[cfg(test)]
mod tests {
    use rivals_scraper::Scraper;

    const BASE_URL: &str = "https://marvelrivals.fandom.com";

    const MOCK_HERO_TABLE: &str = r#"
        <table class="wikitable">
            <tr><th>Hero</th><th>Role</th></tr>
            <tr><td>Luna Snow</td><td>Strategist</td></tr>
            <tr><td>Magneto</td><td>Vanguard</td></tr>
        </table>
    "#;

    const MOCK_HERO_PAGE: &str = r#"
        <div class="hero-name">Luna Snow</div>
        <a class="hero-link" href="/wiki/Luna_Snow">Luna Snow</a>
    "#;

    #[test]
    fn test_parse_table() {
        let scraper = Scraper::new(BASE_URL, 0);
        let rows = scraper.parse_table(MOCK_HERO_TABLE, "table.wikitable");
        assert_eq!(rows[1][0], "Luna Snow");
    }

    #[test]
    fn test_parse() {
        let scraper = Scraper::new(BASE_URL, 0);
        let names = scraper.parse(MOCK_HERO_PAGE, "div.hero-name");
        assert_eq!(names[0], "Luna Snow");
    }
}
