// TODO This should read through all of the files and manage all of the items in it
// Every line is a new item

use std::{error, fs::File, io::{self, BufRead, BufReader}, iter::empty, ops::Index};

use thiserror::Error;

use crate::entry::Entry;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("Could not retrieve requested entry at index: {0}")]
    RetrievalError(usize),
}

#[derive(Debug)]
pub struct Loader {
    labels: Vec<String>,
    entries: Vec<Entry>,
}

impl Loader {
    pub fn new(path: &str) -> Loader {
        // TODO How to get the entries?
        let all: Vec<String> = load_buf(path);

        let labels: Vec<String> = all[0].split(",").map(|i| i.to_string()).collect();
        let entries:Vec<Entry> = make_entries(&all[1..all.len()]);

        // TODO Need to split the string at the commas, and the line will be the index (skipping 0)
        Loader { labels, entries }
    }

    pub fn get_labels(&self) -> Vec<String> {
        self.labels.clone()
    }
    pub fn get_entries(&self) -> Vec<Entry> {
        self.entries.clone()
    }

    // pub fn at(&self, indx: usize) -> &Entry {
    pub fn at(&self, indx: usize) -> Result<&Entry, LoaderError> {
        // TODO Another way to bypass this unwrap?
        self.entries.get(indx)
    }
}

fn load_buf(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Unable to read file.");
    let buf = BufReader::new(file);

    buf.lines().map(|l| l.expect("Could not read line")).collect()
}

fn make_entries(items: &[String]) -> Vec<Entry> {
    let mut finals: Vec<Entry> = Vec::new();
    for item in items {
        let line = items.iter().position(|r| r == item).unwrap();
        finals.push(Entry::new(line.to_owned(), item.to_string()));
    }
    finals
}
