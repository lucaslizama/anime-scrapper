use scraper::{ElementRef, Selector};

#[derive(Debug)]
pub struct AnimeData {
    pub category: String,
    pub name: String,
    pub links: Vec<String>,
    pub size: String,
    pub date: String,
    pub seeds: u32,
    pub leeches: u32,
    pub downloads: u32,
}

impl AnimeData {
    pub fn new(row: &ElementRef) -> Self {
        let td_selector = Selector::parse("td").unwrap();
        let cells: Vec<ElementRef> = row.select(&td_selector).map(|e| e).collect();
        let category = Self::parse_category(&cells[0]);
        let name = Self::parse_name(&cells[1]);
        let links: Vec<String> = Self::parse_links(&cells[2]);
        let size = cells[3].text().next().unwrap().to_string();
        let date = cells[4].text().next().unwrap().to_string();
        let seeds = cells[5].text().next().unwrap().parse::<u32>().unwrap();
        let leeches = cells[6].text().next().unwrap().parse::<u32>().unwrap();
        let downloads = cells[7].text().next().unwrap().parse::<u32>().unwrap();

        Self {
            category,
            name,
            links,
            size,
            date,
            seeds,
            leeches,
            downloads,
        }
    }

    fn parse_category(cell: &ElementRef) -> String {
        let img_selector = Selector::parse("a > img").unwrap();

        cell.select(&img_selector)
            .next()
            .unwrap()
            .value()
            .attr("src")
            .unwrap()
            .to_string()
    }

    fn parse_name(cell: &ElementRef) -> String {
        let a_selector = Selector::parse("a").unwrap();
        cell.select(&a_selector)
            .last()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string()
    }

    fn parse_links(cell: &ElementRef) -> Vec<String> {
        let a_selector = Selector::parse("a").unwrap();
        cell.select(&a_selector)
            .map(|e| e.value().attr("href").unwrap().to_string())
            .collect()
    }
}
