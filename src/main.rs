use std::{io, vec};
use std::io::Write;

#[derive(Debug)]
struct Contact {
    first_name: String,
    last_name: String,
    phone_number: String,
    email: String,
    address: String
}


impl PartialEq for Contact {
    fn eq(&self, other: &Self) -> bool {
        self.first_name == other.first_name && self.last_name == other.last_name
    }
}

impl Contact {
    fn new() -> Self {
        let prompts = vec!["Enter first name: ", "Enter last name: ", "Enter phone number: ", "Enter email address: ", "Enter physical address: "];
        let mut contact_info: Vec<String> = vec![String::new(); 5];

        for (index, &prompt) in prompts.iter().enumerate() {
            print!("{prompt}");
            io::stdout().flush().unwrap();

            io::stdin()
                .read_line(&mut contact_info[index])
                .expect("Failed to read the line.");

            contact_info[index] = contact_info[index].trim().to_string();
        }

        Self {
            first_name: std::mem::replace(&mut contact_info[0], String::new()),
            last_name: std::mem::replace(&mut contact_info[1], String::new()),
            phone_number: std::mem::replace(&mut contact_info[2], String::new()),
            email: std::mem::replace(&mut contact_info[3], String::new()),
            address: std::mem::replace(&mut contact_info[4], String::new()),
        }
    }
}


struct AddressBook {
    contacts: Vec<Contact>
}


impl AddressBook {
    fn new() -> Self {
        Self {
            contacts: vec![]
        }
    }

    fn add_contact(&mut self) {
        let contact = Contact::new();
        self.contacts.push(contact);
        println!("Contact added successfully!")
    }

    fn display_all(&self) {
        for contact in &self.contacts {
            print!("{:#?}\n", contact);
        }
    }

    fn search_contact(&self) -> Option<&Contact> {
        let contact_index = self.get_index_search();

        match contact_index {
            Some(index) => {
                return Some(&self.contacts[index]);
            },
            None => {
                None
            }
        }
    }

    fn get_index_search(&self) -> Option<usize> {
        print!("Enter a first name: ");
        io::stdout().flush().unwrap();

        let mut first_name = String::new();

        io::stdin()
            .read_line(&mut first_name)
            .expect("Failed to read the line.");

        let first_name = first_name.trim().to_string();

        print!("Enter a last name: ");
        io::stdout().flush().unwrap();

        let mut last_name = String::new();
        io::stdin()
            .read_line(&mut last_name)
            .expect("Failed to read the line");

        let last_name = last_name.trim().to_string();

        for (index, contact) in self.contacts.iter().enumerate() {
            if contact.first_name.to_lowercase() == first_name.to_lowercase() && contact.last_name.to_lowercase() == last_name.to_lowercase() {
                println!("Found!");
                return Some(index);
            }
        }

        None
    }

    fn update_contact(&mut self) {
        let contact_index = self.get_index_search();

        match contact_index {
            Some(index) => {
                let updated_contact = Contact::new();
                self.contacts[index] = updated_contact;
            },
            None => {
                println!("Such contact does not exist!")
            }
        }
    }

    fn delete_contact(&mut self) {
        let contact_index = self.get_index_search();

        match contact_index {
            Some(index) => {
                self.contacts.remove(index);
            },
            None => {
                println!("Such contact does not exist!")
            }
        }
    }

}


fn main() {
    println!("Welcome to Address Book!");
    println!("Available comands:");
    println!("1. Add Contact");
    println!("2. Search Contact");
    println!("3. Display All Contacts");
    println!("4. Update Contact");
    println!("5. Delete Contact");
    println!("6. Exit");

    let mut address_book = AddressBook::new();

    loop {
        print!("\nEnter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line.");

        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        match input {
            1 => address_book.add_contact(),
            2 => {
                let found_countact = address_book.search_contact();
                match found_countact {
                    Some(contact) => println!("{:#?}", contact),
                    None => println!("Contact not found!"),
                };
            },
            3 => address_book.display_all(),
            4 => address_book.update_contact(),
            5 => address_book.delete_contact(),
            6 => break,
            _ => {
                println!("Exiting the Address Book!");
                break;
            }
        }
    }
}
