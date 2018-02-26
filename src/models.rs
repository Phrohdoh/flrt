use ::std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Package {
    pub name: String,
    pub latest: Pubspec,
    pub versions: Vec<Pubspec>,
}

impl Package {
    pub fn from_json(json: &str) -> Result<Self, String> {
        Ok(::serde_json::from_str(json).map_err(|e| format!("Error deserializing JSON: {}", e))?)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, String> {
        Ok(::serde_yaml::from_str(yaml).map_err(|e| format!("Error deserializing YAML: {}", e))?)
    }

    pub fn to_yaml(&self) -> Result<String, String> {
        Ok(::serde_yaml::to_string(self).map_err(|e| format!("Error serializing YAML: {}", e))?)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthorData {
    Author(String),
    Authors(Vec<String>),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pubspec {
    pub name: Option<String>,
    pub version: Option<String /* version spec */>,
    pub description: Option<String>,
    pub author: Option<AuthorData>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub dependencies: Option<HashMap<String, String /* version spec */>>,
    pub dev_dependencies: Option<HashMap<String, String /* version spec */>>,
    pub dependency_overrides: Option<HashMap<String, String /* version spec*/>>,
}