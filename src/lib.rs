#![feature(drain_filter)]
#![feature(str_split_once)]

// extern crate util;

// pub use util::*;
// pub mod util;
// extern crate itertools;

extern crate util;
pub use util::*;

pub mod audible;
pub mod gen;
pub mod simple;
pub mod rc_refcell;
// pub mod parse;

pub const DELIMITER_TOPIC: &str = "{{Topic}}";
pub const CT_DUMMY_VALUE: &str = "***";
pub const CATEGORY_BOOKS: &str = "Books";

pub fn topic_name_to_file_name (topic_name: &str) -> String {
    format!("{}.TXT", format::windows_file_name(&topic_name, "_"))
}

// pub mod challenges;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
