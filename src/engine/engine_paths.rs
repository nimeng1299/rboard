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
    fn read_to_file() -> Option<Self> {
        //获取运行文件夹
        let current_path = std::env::current_dir().ok()?;
        let path = current_path.join("engines.json");
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
        let current = json["current_path"].clone();
        Some(EnginePaths {
            paths,
            current_path: current.as_i32(),
        })
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
