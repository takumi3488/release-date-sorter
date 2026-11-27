use chrono::{Datelike, NaiveDate};
use regex::Regex;
use scraper::{ElementRef, Html, Selector};

use crate::domain::entity::volume::NewVolume;

use super::page::Page;

const SERIES_ID: &str = "aobuta";
const TITLE_PATTERN: &str = r"^『(.+?)』";
/// Captures the date, then the optional release month/day in "初版発行（M月D日発売）".
const DATE_PATTERN: &str =
    r"(\d{4})年(\d{1,2})月(\d{1,2})日(?:初版発行（(?:(\d{1,2})月(\d{1,2})日)?)?";

#[derive(Clone)]
pub struct Aobuta {}

/// Text of `li` without nested lists (special editions), citations and styles.
fn own_text(li: ElementRef<'_>) -> String {
    li.children()
        .filter_map(|child| match ElementRef::wrap(child) {
            Some(el) if matches!(el.value().name(), "ul" | "ol" | "sup" | "style") => None,
            Some(el) => Some(el.text().collect::<String>()),
            None => child.value().as_text().map(|t| t.to_string()),
        })
        .collect()
}

/// Parses e.g. "『タイトル』、2022年12月10日初版発行（12月9日発売）、ISBN …".
/// The release date ("発売") is preferred over the first-printing date ("初版発行").
fn parse_item(text: &str, title_pattern: &Regex, date_pattern: &Regex) -> Option<NewVolume> {
    let title = title_pattern.captures(text.trim_start())?.get(1)?.as_str();
    let captures = date_pattern.captures(text)?;
    let date = NaiveDate::from_ymd_opt(
        captures[1].parse().ok()?,
        captures[2].parse().ok()?,
        captures[3].parse().ok()?,
    )?;
    let publication_date = match (captures.get(4), captures.get(5)) {
        (Some(month), Some(day)) => {
            let month = month.as_str().parse().ok()?;
            // Released in December, first printing dated January of the next year.
            let year = if month > date.month() {
                date.year() - 1
            } else {
                date.year()
            };
            NaiveDate::from_ymd_opt(year, month, day.as_str().parse().ok()?)?
        }
        _ => date,
    };
    Some(NewVolume::new(SERIES_ID, title, publication_date))
}

impl Page<Aobuta> for Aobuta {
    fn get_url() -> String {
        "https://ja.wikipedia.org/wiki/%E9%9D%92%E6%98%A5%E3%83%96%E3%82%BF%E9%87%8E%E9%83%8E%E3%82%B7%E3%83%AA%E3%83%BC%E3%82%BA".to_string()
    }

    fn get_id() -> String {
        SERIES_ID.to_string()
    }

    fn get_volumes(html: Html) -> Vec<NewVolume> {
        // Novels only: comics and anime live in other sections.
        let Ok(item_selector) = Selector::parse(r#"section[aria-labelledby="既刊一覧"] li"#)
        else {
            return vec![];
        };
        let (Ok(title_pattern), Ok(date_pattern)) =
            (Regex::new(TITLE_PATTERN), Regex::new(DATE_PATTERN))
        else {
            return vec![];
        };
        html.select(&item_selector)
            .filter_map(|li| parse_item(&own_text(li), &title_pattern, &date_pattern))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn volumes_of(document: &str) -> Vec<(String, Option<NaiveDate>)> {
        Aobuta::get_volumes(Aobuta::get_html_from_document(document))
            .into_iter()
            .map(|v| (v.title, Some(v.publication_date)))
            .collect()
    }

    fn volume(title: &str, y: i32, m: u32, d: u32) -> (String, Option<NaiveDate>) {
        (title.to_string(), NaiveDate::from_ymd_opt(y, m, d))
    }

    #[test]
    fn test_get_volumes() {
        let volumes = volumes_of(include_str!("./fixtures/aobuta.html"));

        // 15 numbered volumes + 2 "+" volumes; special editions are excluded.
        assert_eq!(volumes.len(), 17);
        assert_eq!(
            volumes.first(),
            Some(&volume(
                "青春ブタ野郎はバニーガール先輩の夢を見ない",
                2014,
                4,
                10
            ))
        );
        // Released a day before the first-printing date.
        assert!(volumes.contains(&volume(
            "青春ブタ野郎はマイスチューデントの夢を見ない",
            2022,
            12,
            9
        )));
        assert!(volumes.contains(&volume(
            "青春ブタ野郎はビーチクイーンの夢を見ない+",
            2025,
            7,
            10
        )));
        assert_eq!(
            volumes.last(),
            Some(&volume(
                "青春ブタ野郎はドクターピッグの夢を見ない+",
                2026,
                10,
                9
            ))
        );
    }

    #[test]
    fn test_release_before_new_year() {
        let volumes = volumes_of(
            r#"<section aria-labelledby="既刊一覧"><ul>
            <li>『タイトル』、2020年1月10日初版発行（12月27日発売）、ISBN
            <ul><li>「ドラマCD付き特装版」同日発売、ISBN</li></ul></li>
            </ul></section>"#,
        );
        assert_eq!(volumes, vec![volume("タイトル", 2019, 12, 27)]);
    }
}
