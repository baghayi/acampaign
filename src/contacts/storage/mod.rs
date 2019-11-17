use crate::contacts::Contact;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use crate::contacts::errors::Error;

pub struct Contacts(&'static str);

impl Contacts {
    pub fn store(&self, contact: &Contact) -> Result<(), Error> {
        if self.is_contact_duplicate(&contact) {
            return Err(Error::DuplicateContact);
        }
        
        let _ = self.get_storage().write(contact.email.as_bytes());
        self.go_next_line();
        Ok(())
    }

    fn get_storage(&self) -> File {
        OpenOptions::new()
            .append(true)
            .open(self.0)
            .unwrap_or_else(|_| {
                let mut file = File::create(self.0).unwrap();
                let _ = file.write(b"email\n");
                file
            })
    }

    fn go_next_line(&self) {
        let _ = self.get_storage().write(b"\n");
    }

    fn is_contact_duplicate(&self, contact: &Contact) -> bool {
        let _ = self.get_storage();
        let data = fs::read_to_string(&self.0).unwrap();
        let list: Vec<_> = data
            .lines()
            .filter(|line| line.to_lowercase().contains(&contact.email.to_lowercase()))
            .collect();
        list.len() > 0
    }
 }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contacts::Contact;
    use std::fs;

    fn get_file_contents(filepath: &str) -> String {
        fs::read_to_string(filepath).unwrap()
    }

    fn clean_file(filepath: &str) {
        let _ = fs::remove_file(filepath);
    }

    #[test]
    fn save_new_contact() {
        let filepath = "/tmp/contacts_list_1.txt";
        clean_file(filepath);
        let contact = Contact::from_json("{\"email\":\"husen@gmail.com\"}").unwrap();
        let contacts = Contacts(filepath);
        let _ = contacts.store(&contact);
        assert_eq!("email\nhusen@gmail.com\n", get_file_contents(filepath));
        clean_file(filepath);
    }

    #[test]
    fn only_one_header() {
        let filepath = "/tmp/contacts_list_2.txt";
        clean_file(filepath);
        let contact_1 = Contact::from_json("{\"email\":\"husen@gmail.com\"}").unwrap();
        let contact_2 = Contact::from_json("{\"email\":\"mamaly@gmail.com\"}").unwrap();
        let contacts = Contacts(filepath);
        let _ = contacts.store(&contact_1);
        let _ = contacts.store(&contact_2);
        assert_eq!("email\nhusen@gmail.com\nmamaly@gmail.com\n", get_file_contents(filepath));
        clean_file(filepath);

    }

    #[test]
    fn cannot_store_duplicate_contact() {
        let filepath = "/tmp/contacts_list_3.txt";
        clean_file(filepath);
        let contact_1 = Contact::from_json("{\"email\":\"husen@gmail.com\"}").unwrap();
        let contact_2 = Contact::from_json("{\"email\":\"husen@gmail.com\"}").unwrap();
        let contacts = Contacts(filepath);
        let _ = contacts.store(&contact_1);
        match contacts.store(&contact_2) {
            Err(e) => assert_eq!(crate::contacts::errors::Error::DuplicateContact, e),
            Ok(_) => panic!("cannot store duplicate contact"),
        }
    }

    #[test]
    fn cannot_store_duplicate_contact_with_different_cases() {
        let filepath = "/tmp/contacts_list_3.txt";
        clean_file(filepath);
        let contact_1 = Contact::from_json("{\"email\":\"husen@gmail.com\"}").unwrap();
        let contact_2 = Contact::from_json("{\"email\":\"HUSEN@gmail.com\"}").unwrap();
        let contacts = Contacts(filepath);
        let _ = contacts.store(&contact_1);
        match contacts.store(&contact_2) {
            Err(e) => assert_eq!(crate::contacts::errors::Error::DuplicateContact, e),
            Ok(_) => panic!("cannot store duplicate contact"),
        }
    }
}