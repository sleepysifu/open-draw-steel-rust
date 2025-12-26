use std::{fs, path::Path};
use indexmap::IndexMap;
use serde::de::DeserializeOwned;
use serde_json::from_str;

pub fn load_set<T>(dir: &Path) -> Result<IndexMap<String, T>, String> where T: DeserializeOwned {
    if !dir.exists() {
        return Err(format!("Directory {} does not exist", dir.display()));
    }
    
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Err(format!("Failed to read directory {}", dir.display())),
    };

    let mut definitions = IndexMap::new();
    
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let file_name_with_ext = entry.file_name().to_string_lossy().to_string();
        let file_name = file_name_with_ext
            .strip_suffix(".json")
            .unwrap_or(&file_name_with_ext)
            .to_string();
        
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            return Err(format!("File {} is not a JSON file", path.display()));
        }
        
        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Err(format!("Failed to read file {}", path.display())),
        };
        
        let definition: T = match from_str(&content) {
            Ok(d) => d,
            Err(_) => return Err(format!("Failed to parse file {}", path.display())),
        };
        
        definitions.insert(file_name, definition);
    }
    
    Ok(definitions)
}