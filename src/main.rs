use std::{collections::HashMap, error::Error};

use clap::Parser;
use reqwest::Client;
use scraper::{Html, Selector};

/// Parse command arguments for nscm-ege-result-parser CLI
#[derive(Parser, Debug, Default, Clone, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Имя для поиска результатов
    #[clap(short, long)]
    pub name: String,
    /// Фамилия для поиска результатов
    #[clap(short, long)]
    pub last_name: String,
    /// Код регистрации (из уведомления) для поиска результатов
    #[clap(short, long)]
    pub id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let url = "http://nscm.ru/egeresult/tablresult.php";

    let mut form_data = HashMap::new();
    form_data.insert("Lastname", &cli.last_name);
    form_data.insert("Name", &cli.name);
    form_data.insert("idnomer", &cli.id);

    let client = Client::new();
    let html = client
        .post(url)
        .form(&form_data)
        .send()
        .await?
        .text()
        .await?;

    let document = Html::parse_document(&html);
    let row_selector = Selector::parse("table.tab_result tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    for row in document.select(&row_selector) {
        let cells: Vec<_> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>())
            .collect();

        if cells.len() == 3 {
            let subject = &cells[0];
            let date = &cells[1];
            let grade = &cells[2];
            println!("Предмет: {}, Дата: {}, Балл: {}", subject, date, grade);
        }
    }

    Ok(())
}
