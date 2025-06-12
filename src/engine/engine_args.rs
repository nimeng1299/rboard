use json::JsonValue;

#[derive(Clone)]
pub struct EngineArgs {
    pub path: String,
    pub args: Vec<String>,
}

impl EngineArgs {
    pub fn new(path: String) -> Self {
        EngineArgs { path, args: vec![] }
    }
    pub fn to_json(&self) -> JsonValue {
        json::object! {
            path: self.path.clone(),
            args: self.args.clone()
        }
    }
    pub fn from_json(json: &JsonValue) -> Self {
        let path = json["path"].as_str().unwrap().to_string();
        let _args = json["args"].clone();
        let mut args = vec![];
        match _args {
            JsonValue::Array(array) => {
                for item in array {
                    if let JsonValue::String(s) = item {
                        args.push(s.clone());
                    }
                }
            }
            _ => {}
        }
        EngineArgs { path, args }
    }
}
