use std::collections::HashMap;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub enum Error {
    UndefinedCommand,
}

pub trait Command {
    fn name(&self) -> &'static str;
    fn run(&mut self);
}

pub struct Commands
{
    list: HashMap<&'static str, Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Commands
    {
        let commands: HashMap<_, _> = HashMap::new();
        Commands {
            list: commands
        }
    }

    pub fn register(&mut self, command: Box<dyn Command>)
    {
        self.list.insert(command.name(), command);
    }

    pub fn run(&mut self, command_name: &str) -> Result<(), Error> {
        let result = self.list.get_mut(command_name);
        return match result {
            Some(cmd) => {
                cmd.run();
                Ok(())
            },
            None => Err(Error::UndefinedCommand),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{RefCell, RefMut};
    use std::rc::Rc;
    use super::*;
    struct ACommand {
        buffer: Rc<RefCell<String>>
    }
    impl Command for ACommand {
        fn name(&self) -> &'static str {
            "command-name"
        }

        fn run(&mut self) {
            let mut text: RefMut<String> = self.buffer.borrow_mut();
            text.push_str("ran command");
        }
    }

    struct BCommand;
    impl Command for BCommand {
        fn name(&self) -> &'static str {
            "command-b"
        }

        fn run(&mut self) {
        }
    }

    struct CCommand;
    impl Command for CCommand {
        fn name(&self) -> &'static str {
            "command-c"
        }

        fn run(&mut self) {
        }
    }

    #[test]
    fn register_a_command() {
        let mut commands = Commands::new();
        commands.register(Box::new(BCommand));
    }

    #[test]
    fn run_a_registered_command() {
        let buffer = Rc::new(RefCell::new(String::new()));
        let command = ACommand{buffer: Rc::clone(&buffer)};
        let mut commands = Commands::new();
        commands.register(Box::new(command));
        let _ = commands.run("command-name");
        assert_eq!(*buffer.borrow(), "ran command");
    }

    #[test]
    fn not_found_command_error_when_trying_to_run_an_undefined_command() {
        let mut commands = Commands::new();
        commands.register(Box::new(BCommand));
        let result = commands.run("undefined");
        match result {
            Err(e) => assert_eq!(Error::UndefinedCommand, e),
            Ok(_) => panic!("cannot run undefined command"),
        }
    }

    #[test]
    fn register_different_commands() {
        let command_b = BCommand;
        let command_c = CCommand;
        let mut commands = Commands::new();
        commands.register(Box::new(command_b));
        commands.register(Box::new(command_c));
    }
}