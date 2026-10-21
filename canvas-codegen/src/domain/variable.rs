use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VariableType {
    #[default]
    String,
    Number,
    Boolean,
}

impl std::fmt::Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableType::String => write!(f, "String"),
            VariableType::Number => write!(f, "Number"),
            VariableType::Boolean => write!(f, "Boolean"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: String,
    pub data_type: VariableType,
    pub default_value: String,
}

impl Variable {
    pub fn new(name: String, data_type: VariableType, default_value: String) -> Self {
        Self {
            name,
            data_type,
            default_value,
        }
    }
}
