
use crypto::digest::Digest;
use crypto::sha1::Sha1;
use regex::Regex;

use std::collections::BTreeMap;
use std::io::Write;

use super::error::TgitError;

use super::file::FileService;
use super::index::Index;

pub struct Commit {
    pub hash: Option<String>,
    pub data: Option<Vec<u8>>,
    pub parent: Option<String>,
    pub files: BTreeMap<String, String>,
}

impl Commit {
    pub fn new(parent: Option<&Commit>) -> Commit {
        let mut commit = Commit {
            hash: None,
            data: None,
            parent: match parent {
                Some(&Commit {
                    hash: Some(ref hash),
                    ..
                }) => Some(hash.to_string()),
                _ => None,
            },
            files: BTreeMap::new(),
        };

        for (ref hash, ref path) in parent.iter().flat_map(|p| p.files.iter()) {
            commit.files.insert(hash.to_string(), path.to_string());
        }

        commit
    }

    pub fn add_from_index(&mut self, index: &Index) {
        for (ref hash, ref path) in index.hashtree.iter() {
            self.files.insert(hash.to_string(), path.to_string());
        }
    }

    pub fn from_string(hash: &str, input: &str) -> Result<Commit, TgitError> {
        let mut commit = Commit::new(None);
        commit.hash = Some(hash.to_string());
        lazy_static! {
            static ref PARENT: Regex = Regex::new(r"parent ([0-9a-f]{40})").unwrap();
            static ref BLOB: Regex = Regex::new(r"blob ([0-9a-f]{40}) (.*)").unwrap();
        }

        for line in input.lines() {
            if let Some(ref caps) = PARENT.captures(line) {
                commit.parent = Some(caps.get(1).unwrap().as_str().to_string());
            }

            if let Some(ref caps) = BLOB.captures(line) {
                let hash = caps.get(1).unwrap().as_str();
                let path = caps.get(2).unwrap().as_str();
                commit.files.insert(hash.to_string(), path.to_string());
            }
        }

        Ok(commit)
    }

    pub fn print(&self) {
        if let Some(ref p) = self.parent {
            println!("parent {}", p);
        }
        for (ref hash, ref path) in self.files.iter() {
            println!("blob {} {}", hash, path);
        }
    }


    pub fn update(&mut self) {
        let mut data = Vec::new();

        if let Some(ref p) = self.parent {
            writeln!(&mut data, "parent {}", p).unwrap();
        }

        for (ref hash, ref path) in self.files.iter() {
            writeln!(&mut data, "blob {} {}", hash, path).unwrap();
        }

        let mut sha = Sha1::new();
        sha.input(&data);
        self.hash = Some(sha.result_str());
        self.data = Some(data);
    }
}

pub fn commit() -> Result<(), TgitError> {
    let fs = FileService::new()?;
    let head_ref = fs.get_head_ref()?;
    let parent_hash = FileService::get_hash_from_ref(&head_ref);
    let mut index = Index::new(&fs.root_dir)?;

    let parent = match parent_hash {
        Some(ref h) => Some(fs.read_commit(h)?),
        None => None,
    };

    let mut commit = Commit::new(parent.as_ref());
    parent.map(|p| p.print());
    commit.add_from_index(&index);
    commit.print();

    fs.write_commit(&mut commit)?;
    index.clear()?;
    Ok(())

}

