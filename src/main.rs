use reqwest::blocking::Client;
use std::{env, fs};
use tl::parse;

fn page_parser(resp: &str) -> String {
    let dom = parse(resp, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();

    let chapter_content = dom
        .query_selector("div.epcontent")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .inner_text(parser);
    let cleaned_chapter_content =
        chapter_content.replace("(adsbygoogle = window.adsbygoogle || []).push({});", "");

    let compacted_chapter_content = cleaned_chapter_content
        .split("\n")
        .filter(|x| !x.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

    let chapter_title = compacted_chapter_content
        .split("\n")
        .take(1)
        .collect::<String>();

    let next_chapter = dom
        .query_selector("a[rel='next']")
        .unwrap()
        .next()
        .unwrap()
        .get(parser)
        .unwrap()
        .as_tag()
        .unwrap()
        .attributes()
        .get("href")
        .unwrap()
        .unwrap()
        .as_utf8_str();

    println!("Chapter Title: {}", chapter_title);
    fs::write(
        format!("{}.txt", chapter_title.trim()),
        compacted_chapter_content.to_string(),
    )
    .unwrap();

    println!("Saved {chapter_title}");
    next_chapter.to_string()
}
fn crawler(url: &str, limit: u32, counter: u32) {
    if counter == limit {
        return;
    }
    let cli = Client::new();
    let resp = cli.get(url).send().unwrap().text().unwrap();
    let url = page_parser(&resp);
    crawler(&url, limit, counter + 1)
}

fn main() {
    // Get arguments from command line
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        panic!("Usage: ./rscraper <url> <limit>");
    }

    // Parse arguments
    let url = &args[1];
    let limit = args[2].parse::<u32>().unwrap();

    // Call crawler function with parsed arguments
    crawler(url, limit, 0);
}
