// TODO: Re-implement `Ticket`'s accessor methods. This time return a `&str` rather than a `&String`.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }

        Ticket {
            title,
            description,
            status,
        }
    }

    /*

    self.title is a String
    &self.title is, therefore, a &String
    The output of the (modified) title method is &str... Why no compile error?
    Answer:
    The Deref trait is the mechanism behind the language feature known as deref coercion.
    String implements the Deref trait, which allows it to be automatically converted to a &str when needed.
    https://rust-exercises.com/100-exercises/04_traits/07_deref
    */
    pub fn title(&self) -> &str {
        &self.title //self.title.as_str()
    }

    pub fn description(&self) -> &str {
        &self.description //self.description.as_str()
    }

    pub fn status(&self) -> &str {
        &self.status //self.status.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{valid_description, valid_title};
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // Some dark magic to verify that you used the expected return types
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
