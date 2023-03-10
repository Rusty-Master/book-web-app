use super::base::Base;
use crate::to_do::{
    enums::TaskStatus,
    traits::{delete::Delete, edit::Edit, get::Get},
};

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

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
