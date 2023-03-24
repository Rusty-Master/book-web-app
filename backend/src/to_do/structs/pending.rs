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

#[cfg(test)]
mod pending_tests {
    use super::Pending;
    use super::TaskStatus;

    #[test]
    fn new() {
        let title = "expected title";
        let pending_item = Pending::new(title);

        assert_eq!(title.to_string(), pending_item.base.title);
        assert_eq!(TaskStatus::Pending, pending_item.base.status);
    }
}
