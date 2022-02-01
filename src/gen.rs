use std::path;
use util::html;
use crate::*;
use chrono::NaiveDate;
use itertools::Itertools;
use std::fs::File;
use std::io::Write;
use crate::audible::BookForAudible;

pub fn gen_page_from_chrome_bookmarks(path_file: &path::Path) {
    assert!(path_file.is_file());
    let bookmark_set = html::parse_chrome_bookmarks(path_file);
    // bookmark_set.display_deep(0);
    let mut s = String::new();
    gen_section_from_chrome_bookmarks(&mut s,1, &bookmark_set);
    println!("{}", s);
}

fn gen_section_from_chrome_bookmarks(s: &mut String, depth: usize, bookmark_set: &html::BookmarkSet) {
    gen_header(s, depth, &bookmark_set.name);
    for link in &bookmark_set.links {
        gen_url_line_label_first(s, &link.url, &link.label);
    }
    for set in &bookmark_set.sets {
        gen_section_from_chrome_bookmarks(s, depth + 1, set);
    }
}

pub fn gen_header(s: &mut String, depth: usize, label: &str) {
    let header_delimiter = "=".repeat(depth);
    s.push_str(&format!("\n\n{}{}{}\n", header_delimiter, label, header_delimiter));
}

pub fn gen_url_line_label_first(s: &mut String, url: &str, label: &str) {
    s.push_str(&format!("\n * {}: [[$URL:{}]]", label, url));
}

pub fn gen_book_text_files(path_gen: &str, books: Vec<BookForAudible>) {
    let added_date = NaiveDate::from_ymd(2020, 10, 18);
    for book in books.iter() {
        let category = gen_category("Books");
        let title = gen_table_field_string_single("Title", Some(book.title.as_ref()));
        // let series = gen_table_field_string_single("Series", book.series.map(|x| &x[..]));
        let series = gen_table_field_string_single("Series", book.series.as_deref());
        let author = gen_table_field_string_multiple("Author", &book.authors);
        let narrator = gen_table_field_string_multiple("Narrator", &book.narrators);
        let format = gen_table_field_string_single("Format", Some(book.format.as_ref()));
        let location = gen_table_field_string_single("Location", Some(book.location.as_ref()));
        let year = gen_table_field_string_single("Year", book.year.map(|x| x.to_string()).as_deref());
        let added = gen_table_field_date_single("Added", Some(added_date.clone()));
        let acquired = gen_table_field_date_single("Acquired", book.acquired_date);
        let read = gen_table_field_bool_single("Read", book.read);
        let started = gen_table_field_date_single("Started", None);
        let completed = gen_table_field_date_single("Completed", None);
        let text = format!("{}\n\n{{|\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                category, title, series, author, narrator, format, location, year, added, acquired,
                read, started, completed);
        let gen_file_name = format!("{}\\{}", path_gen, topic_name_to_file_name(&book.short_title));
        dbg!(&gen_file_name, &text);

        let mut file = File::create(&gen_file_name).unwrap();
        file.write_all(text.as_bytes()).expect("could not write file");
    }
}

fn gen_category(value: &str) -> String {
    // Example:
    // [[$CATEGORY:Books]]
    format!("[[$CATEGORY:{}]]", value).to_string()
}

fn gen_table_field_string_single(label: &str, value: Option<&str>) -> String {
    // Example:
    // ||Title||[[Title:=Flowers for Algernon]]||
    let value = value.unwrap_or(CT_DUMMY_VALUE);
    format!("||{}||[[{}:={}]]||", label, label, value).to_string()
}

fn gen_table_field_string_multiple(label: &str, values: &Vec<String>) -> String {
    // Example:
    // ||Author||[[Author:=Jason Fried]], [[Author:=Heinemeier David Hansson]], [[Author:=Matthew Linderman]]||
    assert!(!values.is_empty());
    let list = values.iter().map(|x| format!("[[{}:={}]]", label, x).to_string()).join(", ");
    format!("||{}||{}||", label, list).to_string()
}

fn gen_table_field_date_single(label: &str, value: Option<NaiveDate>) -> String {
    // Example:
    // ||Added||[[Added:=20171128]]||
    gen_table_field_string_single(label, value.map(|x| x.format("%Y%m%d").to_string()).as_deref())
}

fn gen_table_field_bool_single(label: &str, value: Option<bool>) -> String {
    // Example:
    // ||Read||[[Read:=Yes]]||
    gen_table_field_string_single(label, value.map(|x| if x { "Yes" } else { "No" }))
}


