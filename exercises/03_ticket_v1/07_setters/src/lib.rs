// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

use std::fmt::format;

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {

        Self::valid_empty(&title, "Title");
        Self::valid_empty(&description, "Description");

        Self::validate_capacity(&title, 50, "Title");
        Self::validate_capacity(&description, 500, "Description");

        Self::validate_status(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_title(&mut self, new_title: String) {
        Self::valid_empty(&new_title, "Title");

        Self::validate_capacity(&new_title, 50, "Title");

        self.title = new_title;
    }

    pub fn set_description(&mut self, new_description: String) {
        Self::valid_empty(&new_description, "Description");

        Self::validate_capacity(&new_description, 500, "Description");
        self.description = new_description;
    }

    pub fn set_status(&mut self, new_status: String) {
        Self::validate_status(&new_status);
        self.status = new_status;
    }

    fn valid_empty(text: &str, param_name: &str) {
        if text.trim().is_empty() {
            panic!("{}", format!("{} cannot be empty", param_name));
        }
    }

    fn validate_capacity(text: &str, capacity: usize, param_name: &str) {
        if text.len() > capacity {
            panic!("{param_name} cannot be longer than {capacity} bytes");
        }
    }

    fn validate_status(text: &str) {
        if text != "To-Do" && text != "In Progress" && text != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title())
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description())
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
