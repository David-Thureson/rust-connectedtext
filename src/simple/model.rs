use std::collections::BTreeMap;
use chrono::{NaiveDate, Datelike};
use crate::CT_DUMMY_VALUE;
use util_rust::group::Grouper;
use util_rust::{log, parse};

#[derive(Debug)]
pub struct Wiki {
    pub topics: BTreeMap<String, Topic>,
    pub attribute_types: BTreeMap<String, AttributeType>,
}

#[derive(Debug)]
pub struct AttributeType {
    name: String,
    type_: String,
    is_multiple: bool,
    count: usize,
    per_topic_counts: Grouper<usize>,
    date_values: Grouper<NaiveDate>,
    string_values: Grouper<String>,
    bool_values: Grouper<bool>,
    int_values: Grouper<i64>,
}

#[derive(Clone, Debug)]
pub struct Topic {
    pub project_name: String,
    pub name: String,
    pub category: Option<String>,
    pub content: String,
    pub attributes: BTreeMap<String, Vec<String>>,
    pub added_date: Option<NaiveDate>,
    pub title: Option<String>,
    pub series: Option<String>,
    pub authors: Vec<String>,
    pub narrators: Vec<String>,
    pub formats: Vec<String>,
    pub locations: Vec<String>,
    pub year: Option<i32>,
    pub acquired_date: Option<NaiveDate>,
    pub is_read: Option<bool>,
    pub started_date: Option<NaiveDate>,
    pub completed_date: Option<NaiveDate>,
    pub abandoned_date: Option<NaiveDate>,
    pub repeat_score: Option<u32>,
    pub links: Vec<Link>,
}

#[derive(Clone, Debug)]
pub enum Link {
    Internal {
        topic_name: String,
        section_name: Option<String>,
        label: Option<String>,
        type_: LinkType,
    },
    Url {
        url: String,
        label: Option<String>,
    }
}

#[derive(Clone, Debug)]
pub enum LinkType {
    Normal,
    Parent,
    GrandParent,
    Subtopic,
    SeeAlso,
}

impl Wiki {
    pub fn new() -> Self {
        Self {
            topics: BTreeMap::new(),
            attribute_types: BTreeMap::new(),
        }
    }

    pub fn append(&mut self, mut other: Wiki) {
        self.topics.append(&mut other.topics);
        self.resolve_attributes();
    }

    pub fn add_topic(&mut self, topic: Topic) {
        let key = format!("{:<20}{}", topic.project_name.to_lowercase(), topic.name.to_lowercase());
        //bg!(&key);
        self.topics.insert(key, topic);
    }

    pub fn resolve_attributes(&mut self) {
        // self.attribute_types.clear();
        let mut attribute_types = BTreeMap::new();
        for topic in self.topics.values_mut() {
            topic.parse_attributes();
            topic.set_attributes(&mut attribute_types);
        }
        self.attribute_types = attribute_types;
    }

    /*
    // pub fn link_iter(&self) -> FlatMap<Iter<Topic>, Iter<Link>, fn(&Topic) -> Iter<Link> {
    pub fn link_iter(&self) {
        let a = self.topics.iter().flat_map(|topic| topic.links.iter()).into_iter();
        dbg!(&a.);
    }

     */

    pub fn report_link_groups(&self) {
        let mut link_groups = Grouper::new("Links");
        for topic in self.topics.values() {
            for link in topic.links.iter() {
                let key = match link {
                    Link::Internal { topic_name: _, section_name, label, type_: _ } => {
                        match (section_name, label) {
                            (None, None) => "Internal",
                            (None, Some(_)) => "Internal w/ label",
                            (Some(_), None) => "Internal w/ section",
                            (Some(_), Some(_)) => "Internal w/ section and label",
                        }
                    },
                    Link::Url { url: _, label} => {
                        match label {
                            None => "Url",
                            Some(_) => "Url w/label",
                        }
                    },
                };
                link_groups.record_entry(&key);
            }
        }
        link_groups.print_by_count(0, None);
    }

    pub fn report_added_dates(&self) {
        self.report_dates(|topic: &Topic| topic.added_date)
    }

    pub fn report_dates<F>(&self, f: F)
        where F: Fn(&Topic) -> Option<NaiveDate>
    {
        let mut groups = Grouper::new("Dates");
        for topic in self.topics.values() {
            let key = match f(topic) {
                Some(date) => {
                    let year = date.year();
                    year.to_string()
                },
                None => "None".to_string(),
            };
            groups.record_entry(&key);
        }
        groups.list_by_key();
    }

    pub fn report_derived_added_dates(&self) {
        let mut earliest_inbound_links: BTreeMap<String, (String, NaiveDate)> = BTreeMap::new();
        for topic in self.topics.values() {
            if let Some(added_date) = topic.added_date {
                for link in topic.links.iter() {
                    match link {
                        Link::Internal{ topic_name: ref_topic_name, .. } => {
                            match earliest_inbound_links.get(ref_topic_name) {
                                Some((_, found_date)) => {
                                    if added_date < *found_date {
                                        //rintln!("Replacing {} with {}.", found_date, added_date);
                                        earliest_inbound_links.insert(ref_topic_name.to_string(), (topic.name.clone(), added_date));
                                    }
                                },
                                None => {
                                    earliest_inbound_links.insert(ref_topic_name.to_string(), (topic.name.clone(), added_date));
                                }
                            }
                        },
                        _ => {},
                    };
                }
            }
        }
        //bg!(&earliest_inbound_links);
        let mut no_date = vec![];
        for topic in self.topics.values().filter(|topic| topic.added_date.is_none()) {
            match earliest_inbound_links.get(&topic.name) {
                Some((other_topic_name, added_date)) => println!("{}: {}: from {}", added_date, topic.name, other_topic_name),
                None => no_date.push(&topic.name),
            }
        }
        //bg!(&no_date);
    }
}

impl Topic {
    pub fn new(project_name: &str, name: &str, content: &str) -> Self {
        let mut topic = Topic {
            project_name: project_name.to_string(),
            name: name.to_string(),
            category: None,
            content: content.to_string(),
            attributes: BTreeMap::new(),
            added_date: None,
            title: None,
            series: None,
            authors: vec![],
            narrators: vec![],
            formats: vec![],
            locations: vec![],
            year: None,
            acquired_date: None,
            is_read: None,
            started_date: None,
            completed_date: None,
            abandoned_date: None,
            repeat_score: None,
            links: vec![],
        };
        topic.parse_category();
        topic
    }

    fn log(&self, message: &str) {
        log::log(&format!("{}: {}: {}", self.project_name, self.name, message));
    }

    fn parse_category(&mut self) {
        let category_lines = self.content.split("\n").filter(|x| x.trim().starts_with("[[$CATEGORY:")).collect::<Vec<_>>();
        self.category = match category_lines.len() {
            0 => None,
            1 => Some(parse::between(category_lines[0], "[[$CATEGORY:", "]]").trim().to_string()),
            _ => {
                self.log("Multiple $CATEGORY lines.");
                None
            },
        }
    }

    fn parse_attributes(&mut self) {
        self.attributes.clear();
        for line in self.content.split("\n") {
            if line.contains("||[[") && line.contains(":=") {
                //bg!(&line);
                if !line.trim().starts_with("||") || !line.trim().ends_with("||") {
                    self.log("Attribute line is missing pipes at start or end.");
                    continue;
                }
                let line = parse::between(line, "||", "||");
                let (attribute_name, values_part) = parse::split_2(line, "||");
                if self.attributes.contains_key(attribute_name) {
                    self.log(&format!("Attribute {:?} appears more than once.", attribute_name));
                    continue;
                }
                let values_part = parse::between(values_part, "[[", "]]");
                let values_part = values_part.replace("]], [[", "]],[[");
                let mut values = vec![];
                for value_split in values_part.split("]],[[") {
                    //bg!(&value_split);
                    if value_split.contains("[[") || value_split.contains("]]") {
                        self.log(&format!("Value split {:?}; contains extra delimiters.", value_split));
                        continue;
                    }
                    if !value_split.contains(":=") {
                        self.log(&format!("Value split {:?}; does not contain \":=\".", value_split));
                        continue;
                    }
                    let (value_attribute_name, value_attribute_value) = parse::split_2(value_split, ":=");
                    if value_attribute_name != attribute_name {
                        self.log(&format!("Attribute name = {:?} but value says {:?}.", attribute_name, value_attribute_name));
                        continue;
                    }
                    if value_attribute_value != CT_DUMMY_VALUE {
                        if value_attribute_value.trim().is_empty() || value_attribute_value.contains("*") {
                            self.log(&format!("Unexpected blank value in {:?}.", value_split));
                        }
                        values.push(value_attribute_value.to_string());
                    }
                }
                self.attributes.insert(attribute_name.to_string(), values);
            }
        }
    }

    fn set_attributes(&mut self, attribute_types: &mut BTreeMap<String, AttributeType>) {
        self.added_date = self.attribute_date(attribute_types, "Added");
        self.title = self.attribute_string(attribute_types, "Title");
        self.series = self.attribute_string(attribute_types, "Series");
        self.authors = self.attribute_string_mult(attribute_types, "Author");
        self.narrators = self.attribute_string_mult(attribute_types, "Narrator");
        self.formats = self.attribute_string_mult(attribute_types, "Format");
        self.locations = self.attribute_string_mult(attribute_types, "Location");
        self.year = self.attribute_i32(attribute_types, "Year");
        self.acquired_date = self.attribute_date(attribute_types, "Acquired");
        self.is_read = self.attribute_bool(attribute_types, "Read");
        self.started_date = self.attribute_date(attribute_types, "Started");
        self.completed_date = self.attribute_date(attribute_types, "Completed");
        self.abandoned_date = self.attribute_date(attribute_types, "Abandoned");
        self.repeat_score = self.attribute_u32(attribute_types, "Repeat");
    }

    fn attribute_single(&self, attr_name: &str) -> Option<String> {
        match self.attributes.get(attr_name) {
            Some(attr_values) => {
                match attr_values.len() {
                    0 => None,
                    1 => {
                        let one_value = attr_values[0].to_string();
                        if one_value.eq(CT_DUMMY_VALUE) {
                            None
                        } else {
                            Some(one_value)
                        }
                    },
                    _ => {
                        self.log(&format!("Multiple values for attribute {:?}.", attr_name));
                        None
                    },
                }
            },
            _ => None,
        }
    }

    fn attribute_date(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Option<NaiveDate> {
        match self.attribute_single(attr_name) {
            Some(one_value) => {
                match self.parse_date(&one_value) {
                    Some(one_value) => {
                        let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with(|| { AttributeType::new(attr_name, "Date", false) } );
                        attribute_type.date_values.record_entry(&one_value);
                        Some(one_value)
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn attribute_string(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Option<String> {
        match self.attribute_single(attr_name) {
            Some(one_value) => {
                let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with(|| { AttributeType::new(attr_name, "String", false) } );
                attribute_type.string_values.record_entry(&one_value);
                Some(one_value)
            },
            _ => None,
        }
    }

    fn attribute_string_mult(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Vec<String> {
        let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with(|| { AttributeType::new(attr_name, "String", true) } );
        let mut v = vec![];
        match self.attributes.get(attr_name) {
            Some(attr_values) => {
                for one_value in attr_values {
                    if !one_value.eq(CT_DUMMY_VALUE) {
                        attribute_type.string_values.record_entry(one_value);
                        v.push(one_value.to_string());
                    }
                }
            },
            _ => {},
        };
        v
    }

    fn attribute_i32(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Option<i32> {
        match self.attribute_single(attr_name) {
            Some(one_value) => {
                let one_value = i32::from_str_radix(&one_value, 10);
                match one_value {
                    Ok(one_value) => {
                        let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with(|| { AttributeType::new(attr_name, "Int", false) } );
                        attribute_type.int_values.record_entry(&(one_value as i64));
                        Some(one_value)
                    },
                    _ => {
                        self.log(&format!("Problem parsing i32 = {:?} for attribute {:?}.", one_value, attr_name));
                        None
                    },
                }
            },
            _ => None,
        }
    }

    fn attribute_u32(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Option<u32> {
        match self.attribute_single(attr_name) {
            Some(one_value) => {
                let one_value = u32::from_str_radix(&one_value, 10);
                match one_value {
                    Ok(one_value) => {
                        let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with(|| { AttributeType::new(attr_name, "Int", false) } );
                        attribute_type.int_values.record_entry(&(one_value as i64));
                        Some(one_value)
                    },
                    _ => {
                        self.log(&format!("Problem parsing u32 = {:?} for attribute {:?}.", one_value, attr_name));
                        None
                    },
                }
            },
            _ => None,
        }
    }

    fn attribute_bool(&self, attribute_types: &mut BTreeMap<String, AttributeType>, attr_name: &str) -> Option<bool> {
        match self.attribute_single(attr_name) {
            Some(one_value) => {
                let one_value = one_value.to_lowercase();
                let one_value =
                    if one_value.eq("yes") {
                        Some(true)
                    } else {
                        if one_value.eq("no") {
                            Some(false)
                        } else {
                            self.log(&format!("Problem parsing bool = {:?} for attribute {:?}.", one_value, attr_name));
                            None
                        }
                    };
                match one_value {
                    Some(one_value) => {
                        let attribute_type = attribute_types.entry(attr_name.to_string()).or_insert_with( | | { AttributeType::new(attr_name, "Bool", false) } );
                        attribute_type.bool_values.record_entry( & one_value);
                        Some(one_value)
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }

    fn parse_date(&self, date_string: &str) -> Option<NaiveDate> {
        let y = i32::from_str_radix(&date_string[..4], 10);
        let m = u32::from_str_radix(&date_string[4..6], 10);
        let d = u32::from_str_radix(&date_string[6..8], 10);
        match (y, m, d) {
            (Ok(y), Ok(m), Ok(d)) => {
                if y < 2000 || y > 2030 || m < 1 || m > 12 || d < 1 || d > 31 {
                    self.log(&format!("Problem parsing date = {:?}.", date_string));
                    None
                } else {
                    let date = NaiveDate::from_ymd_opt(y, m, d);
                    match date {
                        Some(date) => Some(date),
                        _ => {
                            self.log(&format!("Problem parsing date = {:?}.", date_string));
                            None
                        }
                    }
                }
            },
            _ => {
                self.log(&format!("Problem parsing date = {:?}.", date_string));
                None
            }
        }
    }

}

impl AttributeType {
    pub fn new(name: &str, type_: &str, is_multiple: bool) -> Self {
        Self {
            name: name.to_string(),
            type_: type_.to_string(),
            is_multiple,
            count: 0,
            per_topic_counts: Grouper::new("per_topic_counts"),
            date_values: Grouper::new("date_values"),
            string_values: Grouper::new("date_values"),
            bool_values: Grouper::new("bool_values"),
            int_values: Grouper::new("int_values"),
        }
    }
}
