use serde::Serialize;

use crate::to_do::enums::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}

#[cfg(test)]
mod base_tests {

    use super::Base;
    use super::TaskStatus;

    #[test]
    fn new() {
        let expected_title = String::from("test title");
        let expected_status = TaskStatus::Done;
        let new_base_struct = Base {
            title: expected_title.clone(),
            status: expected_status.clone(),
        };

        assert_eq!(expected_title, new_base_struct.title);
        assert_eq!(expected_status, new_base_struct.status);
    }
}
