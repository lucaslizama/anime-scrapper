use parser::AnimeData;
use scraper::{Html, Selector};
use std::error::Error;

mod parser;

const BASE_URL: &str = "https://nyaa.si/";

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .gzip(true)
        .build()?;
    let body = client
        .get(BASE_URL)
        .query(&[
            ("q", "Spy+Family"),
            ("f", "2"),
            ("c", "1_2"),
            ("s", "id"),
            ("o", "desc"),
        ])
        .send()?
        .text()?;
    let parsed_body = Html::parse_document(&body);
    let tr_selector = Selector::parse("tbody > tr").unwrap();
    let mut data: Vec<AnimeData> = parsed_body
        .select(&tr_selector)
        .map(|row| AnimeData::new(&row))
        .collect();
    data.sort_by(|a, b| a.name.cmp(&b.name));

    let question = requestty::Question::select("Anime")
        .message("Select anime to download")
        .choices(
            &data
                .iter()
                .map(|a| a.name.to_string())
                .collect::<Vec<String>>()[..],
        )
        .default_separator()
        .build();

    let index = requestty::prompt_one(question)
        .unwrap()
        .as_list_item()
        .unwrap()
        .index;

    open::that(&data[index].links[1]).unwrap();

    Ok(())
}
