use crate::to_do::enums::TaskStatus;

use super::base::Base;

pub struct Pending {
    pub base: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base {
            title: title.to_string(),
            status: TaskStatus::Pending,
        };
        Pending { base }
    }
}
