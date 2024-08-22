use std::error::Error;
use walkdir::{DirEntry, WalkDir};
use std::path::Path;
use home::home_dir;

fn matches_name(entry: &DirEntry, name: &str) -> bool {

    if let Some(name_str) = entry.file_name().to_str() {
        name_str == name
    } else {
        false
    }
}

pub fn get_icon_path(metadata_path: &str) -> Result<String, Box<dyn Error>> {

    let metadata_name = metadata_path.rsplit_once("/").expect("failed to split").1;
    // println!("{}", metadata_name);
    let homedir_paths: Vec<DirEntry> = WalkDir::new(&home_dir().unwrap())
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();

    let mut metadata_paths = Vec::new();
    for entry in &homedir_paths {
        if entry.file_name().to_string_lossy() == metadata_name {
            metadata_paths.push(entry.path());
        }
    }

    let real_path = metadata_paths[0];

    println!("{}", real_path.display());



    // let real
    let app_id = metadata_name.rsplit_once(".metainfo").expect("failed to split").0;
    let icon_name = app_id.to_string() + ".svg";

    let parent_paths: Vec<DirEntry> = WalkDir::new(real_path.parent().expect("REASON"))
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();

    for entry in &parent_paths {
        if entry.file_name().to_string_lossy() == icon_name {
            return Ok(entry.path().to_string_lossy().to_string());
        }
    }

    let parent_paths: Vec<DirEntry> = WalkDir::new(real_path.parent().expect("REASON").parent().expect("REASON"))
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();

    for entry in &parent_paths {
        if entry.file_name().to_string_lossy() == icon_name {
            return Ok(entry.path().to_string_lossy().to_string());
        }
    }

    Err("file not found".into())

}
