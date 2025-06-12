use std::{fs::File, io::Write, path::PathBuf};

use json::JsonValue;

pub struct EnginePaths {
    pub paths: Vec<String>,
    pub current_path: Option<i32>,
}

impl EnginePaths {
    pub fn new() -> Self {
        EnginePaths {
            paths: Vec::new(),
            current_path: None,
        }
    }
    fn get_current_path() -> Option<PathBuf> {
        let current_path = std::env::current_dir().ok()?;
        Some(current_path.join("engines.json"))
    }
    fn read_to_file() -> Option<Self> {
        let path = Self::get_current_path()?;
        let s = std::fs::read_to_string(path).ok()?;
        let json = json::parse(s.as_str()).ok()?;
        let path = json["paths"].clone();
        let mut paths = vec![];
        match path {
            JsonValue::Array(path) => {
                for i in path {
                    match i {
                        JsonValue::String(s) => paths.push(s),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Some(EnginePaths {
            paths,
            current_path: None,
        })
    }
    fn save(&self) -> Result<(), String> {
        let mut array = JsonValue::new_array();
        for p in self.paths.clone() {
            let _ = array.push(p);
        }
        let data = json::object! {
            paths: array
        };
        let mut file = File::create(Self::get_current_path().ok_or("can open file".to_string())?)
            .map_err(|e| e.to_string())?;
        file.write_all(data.to_string().as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
    pub fn add(&mut self, path: PathBuf) -> Result<(), String> {
        self.paths
            .push(path.to_str().ok_or("cannot add path")?.to_string());
        self.save()
    }
    pub fn get_all_paths(&self) -> Vec<&str> {
        self.paths.iter().map(|s| s.as_str()).collect()
    }
}

impl Default for EnginePaths {
    fn default() -> Self {
        match Self::read_to_file() {
            Some(engine_paths) => engine_paths,
            None => Self::new(),
        }
    }
}
