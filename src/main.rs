use std::{collections::HashMap, io::Error} ;

use vampdb::{entry::Entry, loader::Loader};

fn main() -> Result<(), Error> {
    let path = "hello.txt";

    let loader = Loader::new(path);
    println!("{:#?}", &loader);
    println!();

    println!("{:#?}", loader.get_labels());

    let mut db: HashMap<usize, Entry> = HashMap::new();
    // db.insert(loader.at(0).get_line(), loader.at(0).to_owned());
    // TODO Handle options and errors here?
    let entry: &Entry = loader.at(0)?;
    // db.insert(loader.at(0).get_line(), loader.at(0).to_owned());
    db.iter().for_each(|e| println!("File Line: {}, Line Contents: {:#?}", &e.0, &e.1.clone().get_contents()));
}
