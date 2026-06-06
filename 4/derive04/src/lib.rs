use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_partial_eq() {
        let title = "title";
        let description = "description";
        let status = "to-do";
        let ticket1 = Ticket {
            title: title.into(),
            description: description.into(),
            status: status.into(),
        };
        let ticket2 = Ticket {
            title: title.into(),
            description: description.into(),
            status: status.into()
        };
        assert_eq!(ticket1, ticket2);
    }
    
    #[test]
    fn test_description_not_matching() {
        let title = "title";
        let status = "to-do";
        let ticket1 = Ticket {
            title: title.into(),
            description: "description".into(),
            status: status.into(),
        };
        let ticket2 = Ticket {
            title: title.into(),
            description: "des".into(),
            status: status.into(),
        };
        assert_ne!(ticket1, ticket2);
    }
    
    #[test]
    fn test_title_not_matching() {
        let title = "title";
        let description = "description";
        let status = "status";
        let ticket1 = Ticket {
            title: title.into(),
            description: description.into(),
            status: status.into(),
        };
        let ticket2 = Ticket {
            title: "1".into(),
            description: description.into(),
            status: status.into(),
        };
        assert_ne!(ticket1, ticket2);
    }
    
    #[test]
    fn test_status_not_matching() {
        let title = "title";
        let description = "description";
        let status = "In Progress";
        let ticket1 = Ticket {
            title: title.into(),
            description: description.into(),
            status: "to-do".into(),
        };
        let ticket2 = Ticket {
            title: title.into(),
            description: description.into(),
            status: status.into(),
        };
        assert_ne!(ticket1, ticket2);
    }
}