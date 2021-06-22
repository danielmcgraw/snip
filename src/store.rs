use std::collections::HashMap;
use std::fs::{File, write};
use std::io::{Read, Write};
use std::path::Path;
use cli_clipboard::{ ClipboardContext, ClipboardProvider };
use url::Url;
use webbrowser;
use zip::{CompressionMethod, ZipWriter, write};

extern crate dirs;

const FILE_NAME: &str = ".snip";

#[derive(Debug)]
pub struct Store {
    topics: HashMap<String, HashMap<String, String>>,
}

impl Store {
    pub fn new() -> Store {
        let mut store = Store{ topics: HashMap::new() };
        store.read_in();
        store
    }

    pub fn add_list(&mut self, list_name: &str) {
        self.topics.entry(list_name.to_string()).or_insert(HashMap::new());
    }

    fn get_list(&self, list_name: &str) -> Option<&HashMap<String, String>> {
        self.topics.get(list_name)
    }

    pub fn delete_list(&mut self, list_name: &str) {
        self.topics.remove(list_name);
    }

    pub fn print_list(&self, list_name: &str) {
        if let Some(list) = self.get_list(list_name) {
            for (key, val) in list {
                println!("{}: {}", key, val);
            }
        }
    }

    pub fn add_list_entry(&mut self, list_name: &str, key: &str, val: &str) {
        if let Some(list) = self.topics.get_mut(list_name) {
            list.entry(key.to_string()).or_insert(val.to_string());
            println!("Entry inserted in to {}", list_name);
        } else {
            self.add_list(list_name);
            self.add_list_entry(list_name, key, val);
        }
    }

    fn get_list_entry(&self, list_name: &str, key: &str) -> Option<&String> {
        if let Some(list) = self.get_list(list_name) {
            list.get(key)
        } else {
            None
        }
    }

    pub fn copy_list_entry(&self, list_name: &str, key: &str) {
        let mut ctx = ClipboardContext::new().unwrap();
        if let Some(val) = self.get_list_entry(list_name, key) {
            ctx.set_contents(val.to_owned()).unwrap();
        }
    }

    pub fn open_list_entry(&self, list_name: &str, key: &str) {
        if let Some(val) = self.get_list_entry(list_name, key) {
            let _ = Url::parse(val).expect(format!("'{}' is not a URL. Can't open.", val).as_str());
            let _ = webbrowser::open(val);
        }
    }

    pub fn print_list_entry(&self, list_name: &str, key: &str) {
        if let Some(val) = self.get_list_entry(list_name, key) {
            println!("{}", val);
        }
    }

    pub fn delete_list_entry(&mut self, list_name: &str, key: &str) {
        if let Some(list) = self.topics.get_mut(list_name) {
            list.remove(key);
        }
    }

    pub fn print_all(&self) {
        for key in self.topics.keys() {
            println!("{}", key);
            println!("---------------");
            self.print_list(&key);
            println!("---------------\n");
        }
    }

    pub fn nuke(&mut self) {
        self.topics = HashMap::new();
    }

    pub fn export(&self, path: &str) {
        let options = write::FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o755);
        let zip_file = File::create(&format!("{}/{}", path, "snip.zip")).unwrap();
        let mut zip = ZipWriter::new(zip_file);
        let mut buffer = Vec::new();

        if let Some(path) = dirs::home_dir() {
            let _ = zip.start_file(".snip", options);
            let mut f = File::open(format!("{}/{}", path.display(), FILE_NAME)).expect(".snip file cant be read");

            f.read_to_end(&mut buffer).expect("Can't read .snip file");
            zip.write_all(&*buffer).expect("Can't write zip file");
            buffer.clear();
        } else {
            println!("Impossible to get your home dir!");
        }
        zip.finish().expect("Failed to write zip file");
    }

    pub fn write_out(&self) {
        if let Some(path) = dirs::home_dir() {
            let _ = write(
                format!("{}/{}", path.display(), FILE_NAME),
                serde_json::to_string(&self.topics).unwrap()
            );
        } else {
            println!("Impossible to get your home dir!");
        }
    }

    pub fn read_in(&mut self) {
        if let Some(path) = dirs::home_dir() {
            if Path::new(&format!("{}/{}", path.display(), FILE_NAME)).exists() {
                let file = File::open(format!("{}/{}", path.display(), FILE_NAME))
                    .expect("file should open read only");
                self.topics = serde_json::from_reader(file)
                    .expect("file should be proper JSON");
            }
        }
    }
}

#[cfg(test)]
mod test_store {
    use super::*;

    #[test]
    fn new_store() {
        let store = Store::new();
        assert!(store.topics.is_empty());
    }

    #[test]
    fn create_a_list() {
        let list_name = "a_list";
        let mut store = Store::new();
        assert!(store.topics.is_empty());
        store.add_list(list_name);
        assert!(!store.topics.is_empty());
        assert_ne!(store.get_list(list_name), None);
    }

    #[test]
    fn create_a_list_entry() {
        let list_name = "a_list";
        let entry_key = "entry_key";
        let entry_val = "entry_val";
        let mut store = Store::new();
        store.add_list(list_name);
        store.add_list_entry(list_name, entry_key, entry_val);
        assert_eq!(store.get_list_entry(list_name, entry_key).unwrap(), entry_val);
    }

    #[test]
    fn copy_a_specific_entry_from_a_list() {
        let list_name = "a_list";
        let entry_key = "entry_key";
        let entry_val = "entry_val";
        let mut store = Store::new();
        store.add_list(list_name);
        store.add_list_entry(list_name, entry_key, entry_val);
        store.copy_list_entry(list_name, entry_key);

        let mut ctx = ClipboardContext::new().unwrap();
        assert_eq!(ctx.get_contents().unwrap(), entry_val);
    }

    #[test]
    fn delete_a_list() {
        let list_name = "a_list";
        let mut store = Store::new();
        assert!(store.topics.is_empty());
        store.add_list(list_name);
        assert_ne!(store.get_list(list_name), None);

        store.delete_list(list_name);
        assert_eq!(store.get_list(list_name), None);
    }

    #[test]
    fn delete_an_entry_from_a_list() {
        let list_name = "a_list";
        let entry_key = "entry_key";
        let entry_val = "entry_val";
        let mut store = Store::new();
        store.add_list(list_name);
        store.add_list_entry(list_name, entry_key, entry_val);
        assert_eq!(store.get_list_entry(list_name, entry_key).unwrap(), entry_val);

        store.delete_list_entry(list_name, entry_key);
        assert_eq!(store.get_list_entry(list_name, entry_key), None);
    }

    #[test]
    fn nuke_all_the_lists() {
        let mut store = Store::new();
        store.add_list("list_1");
        store.add_list("list_2");
        store.add_list_entry("list_1", "entry_key", "entry_val");

        assert_ne!(store.topics, HashMap::new());
        store.nuke();
        assert_eq!(store.topics, HashMap::new());

    }
}
