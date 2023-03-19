use core::fmt;

use serde::Serialize;

#[derive(Clone)]
pub enum TaskStatus {
    Done,
    Pending,
}
impl TaskStatus {
    pub fn from_string(input: String) -> Self {
        match input.as_str() {
            "Done" => TaskStatus::Done,
            "Pending" => TaskStatus::Pending,
            _ => panic!("input {} not supported", input),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            TaskStatus::Done => write!(f, "Done"),
            TaskStatus::Pending => write!(f, "Pending"),
        }
    }
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Ok(serializer.serialize_str(&self.to_string().as_str())?)
    }
}
