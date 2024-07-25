use anyhow::Result;
use ignore::Walk;
use std::fs::read_to_string;
use std::path::Path;
use tiktoken_rs::o200k_base;

type StringPath = String;

struct File(StringPath);

fn is_md(path: &StringPath) -> bool {
    let path = Path::new(path);
    let extension = path.extension();
    match extension {
        Some(ext) => ext == "md",
        None => false,
    }
}

fn read_txt(path: &StringPath) -> Result<String> {
    Ok(read_to_string(path)?)
}

fn recursive_ls_ignored(path: &StringPath) -> Result<Vec<File>> {
    Ok(Walk::new(path)
        .filter_map(|e| {
            let e = e.ok()?;
            if e.file_type()?.is_file() {
                return Some(File(e.path().to_str()?.to_string()));
            }
            None
        })
        .collect::<Vec<_>>())
}

fn count_tokens(txt: &String) -> Result<usize> {
    let bpe = o200k_base()?;
    let tokens = bpe.encode_with_special_tokens(txt);
    Ok(tokens.len())
}

pub fn recursive_count_tokens(path: &StringPath) -> Result<usize> {
    let files = recursive_ls_ignored(path)?;
    let md_files: Vec<&String> = files
        .iter()
        .filter_map(|file| if is_md(&file.0) { Some(&file.0) } else { None })
        .collect();
    let txt_concat = md_files
        .iter()
        .map(|path| read_txt(path))
        .filter_map(|txt| txt.ok())
        .collect::<Vec<String>>()
        .join("\n------\n");
    return count_tokens(&txt_concat);
}
