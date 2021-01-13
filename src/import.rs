use std::{fs, io, path};

use std::collections::{HashMap, HashSet};

use crate::*;
use crate::model;

pub fn fix_file_names(path_full_export_file: &path::Path, path_source: &path::Path, path_dest: &path::Path) -> io::Result<()> {
    assert!(path_source.is_absolute());
    assert!(path_source.is_dir());
    assert!(path_dest.is_absolute());
    assert!(path_dest.is_dir());
    assert_ne!(path_source, path_dest);

    let files_and_topics = reconcile_files_and_topics(path_full_export_file, path_source);
    for path_file_source in util::parse::get_files_ci(path_source, "*.txt").unwrap() {
        assert!(path_file_source.is_file());
        dbg!(&path_file_source);
        let topic_name = files_and_topics.get(&path_file_source).unwrap();
        let file_name_dest = topic_name_to_file_name(&topic_name);
        let path_file_dest = path_dest.join(&file_name_dest);
        assert!(!path_file_dest.exists());
        println!("{}", &path_file_dest.to_str().unwrap());
        fs::copy(&path_file_source, &path_file_dest).unwrap();
    }
    Ok(())
}

/*
const FORCE_CASE_STRINGS: [&str; 46] = ["CBTI", "QS", "HOA", "GWRS", "Henry IV", "Henry V",
    "McClure", "HIIT", "LeanGains", "WA State DOR", "UPS", "VCA", "TriNet", "WinDirStat", "XML",
    "PHP", "ADB", "AI", "II", "III", "RDS", "AWS", "API", "ACH", "SQL", "Blocks.org", "CLion",
    "ConnectedText", "CSharp", "CSS", "MatPlotLib", "PC", "DB", "DBAN", "DBI", "DSL", "DOT",
    "DotNet", "EBNF", "ECMA", "EdgeBundlR", "EncFS", "ES File", "Manager PRO", "EspressDashboard",
    "EspressReport"];

pub fn fix_file_names(path_source: &path::Path, path_dest: &path::Path) -> io::Result<()> {
    assert!(path_source.is_absolute());
    assert!(path_source.is_dir());
    assert!(path_dest.is_absolute());
    assert!(path_dest.is_dir());
    assert_ne!(path_source, path_dest);

    for dir_entry in fs::read_dir(path_source)? {
        let dir_entry = dir_entry?;
        // dbg!(&file);
        // dbg!(&dir_entry.metadata());
        if dir_entry.metadata()?.is_file() {
            let file_name_source = dir_entry.file_name()
                .into_string()
                .unwrap();
            if file_name_source.to_lowercase().ends_with(".txt") {
                let file_name_dest = file_name_source.replace("_ ", " ").replace("  ", " ").replace("/", " ");
                let file_name_dest = fix_file_name(&file_name_dest);
                println!("{}", &file_name_dest);
                // rintln!("{}", fix_file_name(&file_name));
                let path_file_source = dir_entry.path();
                assert!(path_file_source.is_file());
                let path_file_dest = path_dest.join(&file_name_dest);
                assert!(!path_file_dest.exists());
                // println!("{}", &full_file_name_source);
                // println!("{}", &full_file_name_dest);
                //s::copy(&full_file_name_source, &full_file_name_dest).unwrap();
            }
        }
    }
    Ok(())
}

fn fix_file_name(file_name: &str) -> String {
    // We want anthing that appears in parentheses to be lowercase unless it is in the force-case list.
    if file_name.contains("(") {
        let (outer_1, inner, outer_2) = util::parse::split_3_two_delimiters(&file_name, "(", ")");
        format!("{}({}){}",
                format::title_case(&outer_1, Some(&FORCE_CASE_STRINGS)),
                format::force_substring_cases(&inner, Some(&FORCE_CASE_STRINGS)),
                format::title_case_file_name(&outer_2, Some(&FORCE_CASE_STRINGS)))
    } else {
        format::title_case_file_name(&file_name, Some(&FORCE_CASE_STRINGS))
    }
}
*/

pub fn get_image_path(path: &path::Path) -> path::PathBuf {
    assert!(path.is_absolute());
    assert!(path.is_dir());
    let path_buf = path.join(r"Images\");
    if !path_buf.exists() {
        fs::create_dir(&path_buf).unwrap();
    }
    assert!(path_buf.is_dir());
    path_buf
}

pub fn get_image_file_names(path: &path::Path) -> io::Result<HashSet<String>> {
    assert!(path.is_dir());
    Ok(util::parse::find_in_files_ci(path, "*.txt","[[$IMG:Images\\", "]]")?.iter()
        // If there's a pipe character, take only the part before it.
        .map(|x| util::parse::before(&x, "|").to_string())
        .collect())
}

pub fn copy_image_files(path_source: &path::Path, path_dest: &path::Path) -> io::Result<()> {
    assert_ne!(path_source, path_dest);
    assert!(path_source.is_absolute());
    assert!(path_source.is_dir());
    assert!(path_dest.is_absolute());
    assert!(path_dest.is_dir());

    let path_image_source = get_image_path(path_source);
    let path_image_dest = get_image_path(path_dest);
    assert_ne!(path_image_source, path_image_dest);
    assert!(path_image_source.is_absolute());
    assert!(path_image_source.is_dir());
    assert!(path_image_dest.is_absolute());
    assert!(path_image_dest.is_dir());

    for file_name in get_image_file_names(&path_dest)? {
        let path_file_source = path_image_source.join(&file_name);
        let path_file_dest = path_image_dest.join(&file_name);
        assert_ne!(path_file_source, path_file_dest);
        assert!(path_file_source.is_absolute());
        assert!(path_file_dest.is_absolute());
        assert!(!path_file_dest.exists(), "Destination file exists: \"{}\"", &path_file_dest.to_str().unwrap());
        println!("{}", &path_file_source.to_str().unwrap());
        if path_file_source.exists() {
            println!("{}", &path_file_dest.to_str().unwrap());
            fs::copy(&path_file_source, &path_file_dest).unwrap();
        } else {
            println!("\nNo source file.\n");
        }
    }

    Ok(())
}

pub fn get_all_topic_names(path_full_export_file: &path::Path) -> io::Result<Vec<String>> {
    assert!(path_full_export_file.is_absolute());
    assert!(path_full_export_file.is_file());
    util::parse::find_in_file(path_full_export_file, "****************** ", "\r\n")
}

pub fn reconcile_files_and_topics(path_full_export_file: &path::Path, path_source: &path::Path) -> HashMap<path::PathBuf, String> {
    assert!(path_full_export_file.is_absolute());
    assert!(path_full_export_file.is_file());
    assert!(path_source.is_absolute());
    assert!(path_source.is_dir());

    let mut topics: Vec<(String, String)> = get_all_topic_names(path_full_export_file)
        .unwrap()
        .iter()
        .map(|topic_name| (topic_name.to_string(), topic_name_to_file_name(topic_name).to_lowercase()))
        .collect();
    dbg!(&topics);
    let mut map = HashMap::new();
    for (path_file, file_name_from_file) in util::parse::get_files_ci(&path_source, "*.txt")
        .unwrap()
        .iter()
        .map(|path_file| (path_file, path_file.file_name().unwrap().to_str().unwrap().to_lowercase())) {
        let mut file_name_matches: Vec<String> = topics
            .drain_filter(|(_, file_name_from_topic)| file_name_from_topic == &file_name_from_file)
            .map(|(topic_name, _)| topic_name)
            .collect();
        if file_name_matches.len() > 0 {
            assert!(file_name_matches.len() == 1, "Too many matches for {}", &file_name_from_file);
            map.insert(path_file.to_owned(), file_name_matches.remove(0));
        }
    }
    dbg!(&map);
    map
}

pub fn import_topics(file_import: &str, project_name: &str) -> crate::model::Wiki {
    let map = util::parse::read_file_into_sections(file_import, DELIMITER_TOPIC);
    //bg!(map.keys().map(|x| format!("|{}|", x)).collect::<Vec<_>>());
    let mut wiki = crate::model::Wiki::new();
    for (name, content) in map.iter() {
        let topic = model::Topic::new(project_name, name, content);
        wiki.topics.push(topic);
    }
    wiki
}

