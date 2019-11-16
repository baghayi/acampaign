extern crate json;
extern crate addr;
use addr::Email;
use json::JsonValue::Null;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidEmail,
    EmailIsMissing,
}

pub struct Contact {
    pub email: String
}

fn is_email_valid(email: &str) -> bool {
    let email: Result<Email, _> = email.parse();
    match email {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl Contact {

    pub fn from_json(data: &str) -> Result<Contact, Error> {
        let data = json::parse(data).unwrap();

        if data["email"] == Null {
            return Err(Error::EmailIsMissing);
        }

        if !is_email_valid(data["email"].as_str().unwrap()) {
            return Err(Error::InvalidEmail);
        }

        Ok(Contact{
            email: data["email"].to_string()
        })
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
            Err(error) => assert_eq!(error, Error::InvalidEmail),
        }
    }

    #[test]
    fn email_is_required() {
        let contact = Contact::from_json("{}");
        match contact {
            Ok(_) => panic!("should not create contact when email is missing"),
            Err(error) => assert_eq!(error, Error::EmailIsMissing),
        }

    }
}
