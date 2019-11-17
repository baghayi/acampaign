extern crate json;
extern crate addr;
use addr::Email;
use json::JsonValue::Null;

pub mod errors {
    #[derive(PartialEq, Eq, Debug)]
    pub enum Error {
        InvalidEmail,
        EmailIsMissing,
    }
}

pub struct Contact {
    pub email: String
}

pub mod storage;


impl Contact {

    pub fn from_json(data: &str) -> Result<Contact, errors::Error> {
        let data = json::parse(data).unwrap();

        if data["email"] == Null {
            return Err(errors::Error::EmailIsMissing);
        }

        if !Contact::is_email_valid(data["email"].as_str().unwrap()) {
            return Err(errors::Error::InvalidEmail);
        }

        Ok(Contact{
            email: data["email"].to_string()
        })
    }

    fn is_email_valid(email: &str) -> bool {
        let email: Result<Email, _> = email.parse();
        match email {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_contact_by_json() {
        let contact = Contact::from_json("{\"email\": \"husen@gmail.com\"}");
        match contact {
            Ok(contact) => assert_eq!(contact.email, "husen@gmail.com"),
            Err(_) => panic!("invalid email"),
        }
    }

    #[test]
    fn returns_invalid_email_error_on_invalid_email()  {
        let contact = Contact::from_json("{\"email\": \"husen@gmail..com\"}");
        match contact {
            Ok(_) => panic!("should not create contact with invalid email address"),
            Err(error) => assert_eq!(error, errors::Error::InvalidEmail),
        }
    }

    #[test]
    fn email_is_required() {
        let contact = Contact::from_json("{}");
        match contact {
            Ok(_) => panic!("should not create contact when email is missing"),
            Err(error) => assert_eq!(error, errors::Error::EmailIsMissing),
        }

    }
}