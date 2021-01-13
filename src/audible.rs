#![allow(dead_code)]

use chrono::NaiveDate;
use regex::Regex;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::BTreeMap;
use crate::gen;
use util_rust::parse;

const FILE_IMPORT_BOOKS_PERSONAL: &str = r"E:\ConnectedText Restructure 2020-10-17\Audible Books Personal.txt";
const FILE_IMPORT_PURCHASE_DATES_PERSONAL: &str = r"E:\ConnectedText Restructure 2020-10-17\Audible Books Purchase History Personal.txt";
const FILE_IMPORT_BOOKS_QUADRAVEN: &str = r"E:\ConnectedText Restructure 2020-10-17\Audible Books Quadraven.txt";
const FILE_IMPORT_PURCHASE_DATES_QUADRAVEN: &str = r"E:\ConnectedText Restructure 2020-10-17\Audible Books Purchase History Quadraven.txt";
const PATH_HOME_PROJECT_SOURCE: &str = r"E:\ConnectedText Restructure\Home Project";
const PATH_HOME_PROJECT_DEST: &str = r"E:\ConnectedText Restructure 2020-10-17\Home Dest";
const PATH_HOME_PROJECT_DEST_FIXED: &str = r"E:\ConnectedText Restructure 2020-10-17\Home Dest Fixed";
const PATH_TOOLS_PROJECT_DEST: &str = r"E:\ConnectedText Restructure 2020-10-17\Tools Dest";
const PATH_TOOLS_PROJECT_DEST_FIXED: &str = r"E:\ConnectedText Restructure 2020-10-17\Tools Dest Fixed";
const PATH_GEN_BOOKS: &str = r"E:\ConnectedText Restructure 2020-10-17\Gen Books";

pub fn main() {
    println!("\nAudible start\n");

    // import::fix_file_names(path::Path::new(FILE_FULL_EXPORT), path::Path::new(PATH_TOOLS_PROJECT_DEST), path::Path::new(PATH_TOOLS_PROJECT_DEST_FIXED));
    // import::fix_file_names(path::Path::new(FILE_FULL_EXPORT), path::Path::new(PATH_HOME_PROJECT_DEST), path::Path::new(PATH_HOME_PROJECT_DEST_FIXED));
    // gen_from_audible_books(FILE_IMPORT_BOOKS_PERSONAL, FILE_IMPORT_PURCHASE_DATES_PERSONAL, "personal", PATH_GEN_BOOKS);
    // gen_from_audible_books(FILE_IMPORT_BOOKS_QUADRAVEN, FILE_IMPORT_PURCHASE_DATES_QUADRAVEN, "Quadraven", PATH_GEN_BOOKS);

    println!("\nAudible done\n");
}

fn gen_from_audible_books(file_import_books: &str, file_import_purchase_dates: &str, account_name: &str, path_gen: &str) {
    let books = import_audible_books(file_import_books, file_import_purchase_dates, account_name);
    gen::gen_book_text_files(path_gen, books);
}

pub fn import_audible_books(file_import_books: &str, file_import_purchase_dates: &str, account_name: &str) -> Vec<BookForAudible> {
    let purchase_dates = import_audible_purchase_dates(file_import_purchase_dates);
    let mut v = vec![];
    let mut book = make_empty_audible_book(account_name);
    let mut title_line = false;
    let file = File::open(file_import_books).unwrap();
    for line_result in io::BufReader::new(file).lines() {
        if book.title.len() > 0 && book.short_title.len() > 0 && book.acquired_date.is_none() {
            // Figure out the aquired date.
            let date = purchase_dates.get(&book.title);
            if let Some(date) = date {
                book.acquired_date = Some(*date);
            } else {
                let date = purchase_dates.get(&book.short_title);
                if let Some(date) = date {
                    book.acquired_date = Some(*date);
                } else {
                    dbg!(&book);
                    panic!("No match for purchase date.");
                }
            }
        }
        let line = line_result.unwrap().trim().to_string();
        //rintln!("{}", line);
        if line.contains("By  cover art") {
            if book.short_title.len() > 0 {
                v.push(book.clone());
            }
            book = make_empty_audible_book(account_name);
            book.short_title = parse::before(&line, "By  cover art").trim().to_string();
            title_line = true;
            continue;
        }
        if title_line {
            book.title = line.to_string();
            title_line = false;
            continue;
        }
        if line.starts_with("By:") {
            let authors = parse::after(&line, "By: ").trim().to_string();
            let authors = authors.split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>();
            //rintln!("{:?}", &authors);
            book.authors = authors;
            continue;
        }
        if line.starts_with("Narrated by:") {
            let narrators = parse::after(&line, "Narrated by: ").trim().to_string();
            let narrators = narrators.split(",").map(|x| x.trim().to_string()).collect::<Vec<String>>();
            book.narrators = narrators;
            continue;
        }
        if line.starts_with("Series:") {
            let series = parse::after(&line, "Series: ").trim().to_string();
            book.series = Some(series);
            continue;
        }
        if line.eq("Finished") {
            book.read = Some(true);
            continue;
        }
    }
    v.push(book.clone());
    dbg!(&v);
    v
}

pub fn import_audible_purchase_dates(file_import: &str) -> BTreeMap<String, NaiveDate> {
    let mut purchase_dates = BTreeMap::new();
    dbg!(&file_import);
    let file = File::open(file_import).unwrap();
    let lines: Vec<String> = io::BufReader::new(file).lines().map(|x| x.unwrap().trim().to_string()).collect::<Vec<_>>();
    let mut date = NaiveDate::from_ymd(1900, 1, 1);
    let date_regex = Regex::new(r"^\d{2}-\d{2}-\d{2}$").unwrap();
    // let date_regex = Regex::new(r"18").unwrap();
    //bg!(lines);
    for line_index in 0..lines.len() {
        if lines[line_index].starts_with("By: ") {
            // See if the next line has a date.
            let date_line = lines[line_index + 1].clone();
            if date_regex.is_match(&date_line) {
                //rintln!("{}", &lines[line_index + 1]);
                let m = u32::from_str_radix(&date_line[..2], 10).unwrap();
                assert!(m > 0);
                assert!(m <= 12);
                let d = u32::from_str_radix(&date_line[3..5], 10).unwrap();
                assert!(d > 0);
                assert!(d <= 31);
                let y = 2000 + i32::from_str_radix(&date_line[6..8], 10).unwrap();
                assert!(y >= 2014);
                assert!(y <= 2020);
                date = NaiveDate::from_ymd(y, m, d);
                //bg!(&date);
            }
            // The title is one line before the "By: " line.
            let title = parse::after(&lines[line_index - 1], "By: ").trim().to_string();
            purchase_dates.insert(title, date.clone());
        }
    }
    dbg!(&purchase_dates);
    dbg!(&purchase_dates.len());
    purchase_dates
}

fn make_empty_audible_book(account_name: &str) -> BookForAudible {
    BookForAudible {
        audible_account: account_name.to_string(),
        short_title: "".to_string(),
        title: "".to_string(),
        format: "Audiobook".to_string(),
        authors: vec![],
        narrators: vec![],
        series: None,
        location: format!("Audible ({})", account_name).to_string(),
        year: None,
        acquired_date: None,
        read: Some(false),
    }
}

#[derive(Clone, Debug)]
pub struct BookForAudible {
    pub audible_account: String,
    pub short_title: String,
    pub title: String,
    pub format: String,
    pub authors: Vec<String>,
    pub narrators: Vec<String>,
    pub series: Option<String>,
    pub location: String,
    pub year: Option<u32>,
    pub acquired_date: Option<NaiveDate>,
    pub read: Option<bool>,
}

#[derive(Clone)]
pub struct BookAcquired {
    pub title: String,
    pub aquired_date: NaiveDate,
}

