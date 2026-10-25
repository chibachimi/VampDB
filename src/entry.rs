#[derive(Debug, Clone)]
pub struct Entry {
    index: usize,
    contents: Vec<String>,
}

impl Entry {
    pub fn new(index: usize, content: String) -> Entry {
        let contents: Vec<String> = content.split(",").map(|s| s.to_string()).collect();
        Entry {
            index,
            contents
        }
    }   

    pub fn get_line(&self) -> usize {
        // Need to add two because the index starts at 0, and because the first line contains the labels
        self.index + 2
    }

    pub fn get_contents(self) -> Vec<String> {
        self.contents
    }
}
