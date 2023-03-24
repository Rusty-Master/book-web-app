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

#[cfg(test)]
mod done_tests {
    use super::Done;
    use super::TaskStatus;

    #[test]
    fn new() {
        let title = "expected title";
        let done_item = Done::new(title);

        assert_eq!(title.to_string(), done_item.base.title);
        assert_eq!(TaskStatus::Done, done_item.base.status);
    }
}
