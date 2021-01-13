#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

use std::path;

// use util::html;
// use util::format;
use util_rust::group::Grouper;
use std::fmt::Write;
use std::collections::BTreeMap;

use connectedtext::*;
use simple::*;

//#[macro_use]
//extern crate util;

const FILE_FULL_EXPORT: &str = r"E:\ConnectedText Restructure 2020-10-17\Home Export One File\Wiki Export.TXT";
const FILE_IMPORT_TOOLS: &str = r"Tools.txt";
const FILE_IMPORT_HOME: &str = r"Home.txt";
const PATH_HOME_ARCHIVE_PROJECT_SOURCE: &str = r"E:\ConnectedText Restructure\Home Archive Project";
const PATH_HOME_ARCHIVE_PROJECT_DEST: &str = r"E:\ConnectedText Restructure\Home Archive Project Dest";
const PATH_TOOLS_PROJECT_SOURCE: &str = r"E:\ConnectedText Restructure\Tools Project";

const PATH_CHROME_BOOKMARKS: &str = r"E:\Temp\bookmarks_1_29_20.html";

fn main() {
    println!("\nConnectedText start\n");

    log::clear();

    // gen::gen_page_from_chrome_bookmarks(path::Path::new(PATH_CHROME_BOOKMARKS));
    // audible::main();
    // try_load_topics();
    // try_load_links();
    // dbg!(count_topics_in_tools());
    // catalog_attributes();
    // catalog_categories();
    // import_topics().report_added_dates();
    import_topics().report_derived_added_dates();


    //bg!(&util::log::get_sorted());
    //import::test_delimited_entries();

    println!("\nConnectedText done\n");
}

fn run_import() {
    let path_file_full_export = path::Path::new(FILE_FULL_EXPORT);
    // let path_source = path::Path::new(PATH_HOME_PROJECT_SOURCE);
    // let path_dest = path::Path::new(PATH_HOME_PROJECT_DEST);
    let path_source = path::Path::new(PATH_HOME_ARCHIVE_PROJECT_SOURCE);
    let path_dest = path::Path::new(PATH_HOME_ARCHIVE_PROJECT_DEST);
    // let path_source = path::Path::new(PATH_TOOLS_PROJECT_SOURCE);
    // let path_dest = path::Path::new(PATH_TOOLS_PROJECT_DEST);

    // import::fix_file_names(path_file_full_export, path_source, path_dest).ok();
    // dbg!(&import::get_image_file_names(path::Path::new(PATH_HOME_PROJECT_DEST)));
    import::copy_image_files(path_source, path_dest).ok();

    // dbg!(&import::get_all_topic_names(path_file_full_export));
    // dbg!(&import::reconcile_files_and_topics(path_file_full_export, path_source));
    // import::fix_file_names(path::Path::new(PATH_TOOLS_PROJECT_SOURCE), path::Path::new(PATH_TOOLS_PROJECT_DEST)).ok();
    // dbg!(&import::get_image_file_names(path::Path::new(PATH_TOOLS_PROJECT_DEST)));
    // import::copy_image_files(path::Path::new(PATH_TOOLS_PROJECT_SOURCE), path::Path::new(PATH_TOOLS_PROJECT_DEST)).ok();

}

fn count_topics_in_tools() -> usize {
    import::import_topics(FILE_IMPORT_TOOLS, "Tools").topics.len()
}

fn import_topics() -> crate::model::Wiki {
    let mut wiki = import::import_topics(FILE_IMPORT_TOOLS, "Tools");
    wiki.append(import::import_topics(FILE_IMPORT_HOME, "Home"));
    import::add_links(&mut wiki);
    wiki
}

fn try_load_topics() {
    let topics = import_topics();
    dbg!(&topics);
}

fn try_load_links() {
    let mut wiki = import_topics();
    import::add_links(&mut wiki);
}

fn catalog_attributes() {
    let wiki = import_topics();
    let mut attributes: BTreeMap<String, AttributeForCatalog> = BTreeMap::new();
    for topic in wiki.topics.values() {
    // for topic in topics.iter().filter(|x| x.category.eq(&Some(CATEGORY_BOOKS.to_string()))) {
        for (attr_name, attr_values) in topic.attributes.iter() {
            let mut attribute = attributes.entry(attr_name.to_string()).or_insert_with(|| { AttributeForCatalog::new(attr_name) } );
            attribute.count += 1;
            attribute.max_values = std::cmp::max(attribute.max_values, attr_values.len());
            for one_value in attr_values.iter() {
                attribute.register_value(one_value);
            }
        }
    }
    dbg!(&attributes);
}

#[derive(Debug)]
struct AttributeForCatalog {
    pub name: String,
    pub count: usize,
    pub max_values: usize,
    pub values: BTreeMap<String, usize>,
}

impl AttributeForCatalog {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: 0,
            max_values: 0,
            values: BTreeMap::new(),
        }
    }

    pub fn register_value(&mut self, value: &str) {
        let mut entry = self.values.entry(value.to_string()).or_insert_with(|| { 0 } );
        *entry += 1;
    }
}

fn catalog_categories() {
    let wiki = import_topics();
    let mut g = Grouper::new("Categories");
    for topic in wiki.topics.values() {
        if let Some(category) = &topic.category {
            g.record_entry(category);
        }
    }
    g.list_by_key();
}

