use super::base::Base;
use crate::to_do::enums::TaskStatus;

pub struct Done {
    pub base: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::Done,
        };
        Done { base }
    }
}
