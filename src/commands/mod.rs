pub trait Command {

}

pub struct Commands<T: Command> {
    registered_commands: Vec<T>,
}

impl<T: Command> Commands<T> {
    pub fn new() -> Commands<T> {
        let commands: Vec<_> = Vec::new();
        Commands {
            registered_commands: commands
        }
    }

    pub fn register(&self, command: T) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ACommand;
    impl Command for ACommand {

    }

    #[test]
    fn register_a_command() {
        let command = ACommand;
        let commands = Commands::new();
        commands.register(command);
    }
}