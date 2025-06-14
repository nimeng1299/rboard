use json::JsonValue;

#[derive(Clone)]
pub struct EngineArgs {
    pub path: String,
    pub args: String,
    pub name: String,
}

impl EngineArgs {
    pub fn new(path: String) -> Self {
        EngineArgs {
            path,
            args: String::new(),
            name: String::new(),
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
