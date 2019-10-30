//#[macro_use]
extern crate json;
extern crate addr;
use std::env;
use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use addr::Email;
use std::process;

enum Module {
    Contacts(ModuleCommand),
}

enum ModuleCommand {
    New(String)
}

impl fmt::Display for ModuleCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command = match self {
            ModuleCommand::New(_) => "new",
        };
        write!(f, "{}", command)
    }
}

impl Module {
    fn from_string(module: &str, command: &str, input: String) -> Module {
        match module {
            "contacts" => {
                let command = match command {
                    "new" => ModuleCommand::New(input),
                    _ => panic!("unknown command"),
                };
                Module::Contacts(command)
            }
            _ => panic!("unknown module"),
        }
    }
}

fn main() {
    // acampaign contacts new '{"email": "husen@nowhere.com"}'
    let args: Vec<String> = env::args().collect();
    //let module_command_input = &args[3..];
    let module = Module::from_string(&args[1].to_lowercase(), &args[2].to_lowercase(), args[3].clone());
    match module {
        Module:: Contacts(command) => {
            match &command {
                ModuleCommand::New(input) => {
                    create_new_contact(&input);
                }
            }
        }
    }
}

fn create_new_contact(input: &String) {
    let input = json::parse(input).unwrap();
    let filename = "data/contacts.csv";
    let mut needs_header = false;
    let mut contacts = OpenOptions::new()
        .append(true)
        .open(filename)
        .unwrap_or_else(|_| {
            needs_header = true;
            File::create(filename).unwrap()
        });

    if needs_header {
        let _ = contacts.write(b"email\n");
    }

    let email = input["email"].as_str();
    match email {
        None => eprint!("email is required!"),
        Some(email) => {
            validate_email(email);
            let _ = contacts.write(email.as_bytes());
            let _ = contacts.write_all(b"\n");
        },
    }
}

fn validate_email(email: &str) {
    let _: Email = email.parse().unwrap_or_else(|_| {
        eprint!("email is not valid!\n");
        process::exit(1);
    });
}
