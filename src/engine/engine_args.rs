use std::path::Path;

use json::JsonValue;

#[derive(Clone)]
pub struct EngineArgs {
    pub path: String,
    pub args: String,
    pub name: String,
}

impl EngineArgs {
    pub fn new(path: String) -> Self {
        let s = path.clone();
        let name = Path::new(&s)
            .file_stem() // 获取不带扩展名的文件名
            .and_then(|s| s.to_str())
            .unwrap_or("engine");
        EngineArgs {
            path,
            args: String::new(),
            name: name.to_string(),
        }
    }
    pub fn to_json(&self) -> JsonValue {
        json::object! {
            path: self.path.clone(),
            args: self.args.clone(),
            name: self.name.clone()
        }
    }
    pub fn from_json(json: &JsonValue) -> Self {
        let path = json["path"].as_str().unwrap().to_string();
        let args = json["args"].as_str().unwrap().to_string();
        let name = json["name"].as_str().unwrap().to_string();
        EngineArgs { path, args, name }
    }
}
