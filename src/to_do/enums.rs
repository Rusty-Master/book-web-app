use core::fmt;

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
