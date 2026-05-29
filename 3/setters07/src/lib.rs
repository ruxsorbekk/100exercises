pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        validate_title(&title);
        validate_description(&description);
        validate_status(&status);
        
        Ticket { 
            title,
            description,
            status 
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
    pub fn set_title(&mut self, title: String) {
        validate_title(&title);
        self.title = title;
     }
    pub fn set_description(&mut self, descrip: String) {
        validate_description(&descrip);
        self.description = descrip
    }
    pub fn set_status(&mut self, status: String) {
        validate_status(&status);
        self.status = status
    }
}

fn validate_title(title: &String) {
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 bytes");
    }
}
fn validate_description(description: &String) {
    if description.is_empty() {
        panic!("Description cannot be empty");
    }
    if description.len() > 500 {
        panic!("Description cannot be longer than 500 bytes");
    }
}
fn validate_status(status: &String) {
    if status != "To-Do" && status != "In Progress" && status != "Done" {
        panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
    }        
}



#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};
    
    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A title".into());
        ticket.set_description("A description".into());
        ticket.set_status("To-Do".into());
        
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
    
    #[test]
    #[should_panic(expected="Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }
    
    #[test]
    #[should_panic(expected="Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }
    
    #[test]
    #[should_panic(expected="Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title(overly_long_title());
    }
    
    #[test]
    #[should_panic(expected="Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description(overly_long_description());
    }
    
    #[test]
    #[should_panic(expected="Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
    
}