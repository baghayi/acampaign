extern crate json;

pub struct Contact {
    pub email: String
}

impl Contact {

    pub fn from_json(data: &str) -> Contact {
        let data = json::parse(data).unwrap();
        Contact{
            email: data["email"].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contact_by_json() {
        let contact = Contact::from_json("{\"email\": \"husen@gmail.com\"}");
        assert_eq!(contact.email, "husen@gmail.com");
    }
}
