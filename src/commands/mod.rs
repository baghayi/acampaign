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

pub struct Commands<T: Command> {
    list: HashMap<&'static str, T>,
}

impl<T: Command> Commands<T> {
    pub fn new() -> Commands<T> {
        let commands: HashMap<_, _> = HashMap::new();
        Commands {
            list: commands
        }
    }

    pub fn register(&mut self, command: T) {
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
    use super::*;
    struct ACommand<'a> {
        buffer: &'a mut String
    }
    impl Command for ACommand<'_> {
        fn name(&self) -> &'static str {
            "command-name"
        }

        fn run(&mut self) {
            self.buffer.push_str("ran command");
        }
    }

    #[test]
    fn register_a_command() {
        let mut buffer = String::new();
        let command = ACommand{buffer: &mut buffer};
        let mut commands = Commands::new();
        commands.register(command);
    }

    #[test]
    fn run_a_registered_command() {
        let mut buffer = String::new();
        let command = ACommand{buffer: &mut buffer};
        let mut commands = Commands::new();
        commands.register(command);
        let _ = commands.run("command-name");
        assert_eq!(buffer, "ran command");
    }

    #[test]
    fn not_found_command_error_when_trying_to_run_an_undefined_command() {
        let mut buffer = String::new();
        let command = ACommand{buffer: &mut buffer};
        let mut commands = Commands::new();
        commands.register(command);
        let result = commands.run("undefined");
        match result {
            Err(e) => assert_eq!(Error::UndefinedCommand, e),
            Ok(_) => panic!("cannot run undefined command"),
        }
    }
}