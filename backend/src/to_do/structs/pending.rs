use crate::to_do::{
    enums::TaskStatus,
    traits::{create::Create, edit::Edit, get::Get},
};

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

impl Get for Pending {}
impl Create for Pending {}
impl Edit for Pending {}
