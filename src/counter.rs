use anyhow::Result;
use std::fs::{read_dir, read_to_string};
use std::path::Path;
use tiktoken_rs::cl100k_base;

type StringPath = String;

pub enum FolderItems {
    File(StringPath),
    Folder(StringPath),
}

pub fn ls(path: &StringPath) -> Result<Vec<FolderItems>> {
    let entries = read_dir(path)?;
    let folder_items: Vec<FolderItems> = entries
        .map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let item_result = if path.is_dir() {
                FolderItems::Folder(path.to_str()?.to_string())
            } else {
                FolderItems::File(path.to_str()?.to_string())
            };
            Some(item_result)
        })
        .filter_map(|item_result| match item_result {
            Some(item) => Some(item),
            None => None,
        })
        .collect();
    return Ok(folder_items);
}

pub fn is_md(path: &StringPath) -> bool {
    let path = Path::new(path);
    let extension = path.extension();
    match extension {
        Some(ext) => ext == "md",
        None => false,
    }
}

pub fn read_txt(path: &StringPath) -> Result<String> {
    Ok(read_to_string(path)?)
}

pub fn recursive_ls(path: &StringPath) -> Result<Vec<FolderItems>> {
    let current = ls(path)?;
    let mut result = vec![];
    for item in current {
        match item {
            FolderItems::File(file) => {
                if is_md(&file) {
                    result.push(FolderItems::File(file));
                }
            }
            FolderItems::Folder(folder) => {
                result.push(FolderItems::Folder(folder.clone()));
                let mut sub = recursive_ls(&folder)?;
                result.append(&mut sub);
            }
        }
    }
    return Ok(result);
}

pub fn count_tokens(txt: &String) -> Result<usize> {
    let bpe = cl100k_base()?;
    let tokens = bpe.encode_with_special_tokens(txt);
    Ok(tokens.len())
}

pub fn recursive_count_tokens(path: &StringPath) -> Result<usize> {
    let files = recursive_ls(path)?;
    let md_files: Vec<&String> = files
        .iter()
        .filter_map(|item| match item {
            FolderItems::File(path) if is_md(path) => Some(path),
            _ => None,
        })
        .collect();
    let txt_concat = md_files
        .iter()
        .map(|path| read_txt(path))
        .filter_map(|txt| txt.ok())
        .collect::<Vec<String>>()
        .join("\n------\n");
    return count_tokens(&txt_concat);
}
