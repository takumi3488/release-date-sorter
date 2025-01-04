use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::domain::entity::volume::NewVolume;

use super::page::Page;

const SERIES_ID: &str = "danmachi";

#[derive(Clone)]
pub struct Danmachi {}

impl Page<Danmachi> for Danmachi {
    fn get_url() -> String {
        "https://ja.wikipedia.org/wiki/%E3%83%80%E3%83%B3%E3%82%B8%E3%83%A7%E3%83%B3%E3%81%AB%E5%87%BA%E4%BC%9A%E3%81%84%E3%82%92%E6%B1%82%E3%82%81%E3%82%8B%E3%81%AE%E3%81%AF%E9%96%93%E9%81%95%E3%81%A3%E3%81%A6%E3%81%84%E3%82%8B%E3%81%A0%E3%82%8D%E3%81%86%E3%81%8B".to_string()
    }

    fn get_volumes(html: Html) -> Vec<NewVolume> {
        let contents_selector =
            Selector::parse("div#mw-content-text>div.mw-content-ltr.mw-parser-output").unwrap();
        let contents = html.select(&contents_selector).next().unwrap();

        // Selector
        let row_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        // Get elements
        let mut edition_name = None;
        let mut volumes = vec![];
        let date_pattern = Regex::new(r"(\d{4})年(\d{1,2})月(\d{1,2})日").unwrap();
        let mut flg = false;
        for node in contents.children() {
            if node.first_child().is_some_and(|e| {
                !flg && e
                    .value()
                    .as_element()
                    .is_some_and(|e| e.attr("id").is_some_and(|id| id == "既刊一覧"))
            }) {
                flg = true;
            } else if node.first_child().is_some_and(|e| {
                e.value()
                    .as_element()
                    .is_some_and(|e| e.attr("id").is_some_and(|id| id == "漫画"))
            }) {
                break;
            }
            if flg {
                if let Some(first_child) = node.first_child() {
                    let element_ref = ElementRef::wrap(node);
                    if first_child
                        .value()
                        .as_element()
                        .is_some_and(|e| e.name() == "dt")
                    {
                        // Get edition name
                        edition_name =
                            ElementRef::wrap(first_child).map(|el| el.text().collect::<String>());
                        edition_name = edition_name.map(|en| en.trim().to_string());
                    } else if edition_name.clone().is_some()
                        && element_ref.is_some_and(|e| {
                            e.attr("class").is_some_and(|c| c.contains("wikitable"))
                        })
                    {
                        // Get table
                        let table = ElementRef::wrap(node).unwrap();
                        let rows = table.select(&row_selector);
                        let new_volumes = rows
                            .filter_map(|row| {
                                let tds = row.select(&td_selector).collect::<Vec<_>>();
                                if tds.len() != 5 {
                                    return None;
                                }
                                let chapter_name = tds[0].text().collect::<String>();
                                let chapter_name = chapter_name.trim().trim_end_matches(".");
                                let title =
                                    format!("{} {}", edition_name.clone().unwrap(), chapter_name);
                                let publication_date = date_pattern
                                    .captures(&tds[2].text().collect::<String>())
                                    .map(|captures| {
                                        let year = captures
                                            .get(1)
                                            .unwrap()
                                            .as_str()
                                            .parse::<i32>()
                                            .unwrap();
                                        let month = captures
                                            .get(2)
                                            .unwrap()
                                            .as_str()
                                            .parse::<u32>()
                                            .unwrap();
                                        let day = captures
                                            .get(3)
                                            .unwrap()
                                            .as_str()
                                            .parse::<u32>()
                                            .unwrap();
                                        chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap()
                                    })
                                    .or_else(|| {
                                        date_pattern
                                            .captures(&tds[1].text().collect::<String>())
                                            .map(|captures| {
                                                let year = captures
                                                    .get(1)
                                                    .unwrap()
                                                    .as_str()
                                                    .parse::<i32>()
                                                    .unwrap();
                                                let month = captures
                                                    .get(2)
                                                    .unwrap()
                                                    .as_str()
                                                    .parse::<u32>()
                                                    .unwrap();
                                                let day = captures
                                                    .get(3)
                                                    .unwrap()
                                                    .as_str()
                                                    .parse::<u32>()
                                                    .unwrap();
                                                chrono::NaiveDate::from_ymd_opt(year, month, day)
                                                    .unwrap()
                                            })
                                    });
                                publication_date.map(|pd| NewVolume::new(SERIES_ID, &title, pd))
                            })
                            .collect::<Vec<_>>();
                        volumes.extend(new_volumes);
                    }
                }
            }
        }
        volumes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_volumes() {
        let document = include_str!("./fixtures/danmachi.html");
        let html = Danmachi::get_html_from_document(document);
        let volumes = Danmachi::get_volumes(html);
        assert!(volumes.len() > 30);
    }
}
