pub struct Parameter {
    pub shortcut: String,
    pub name: String,
    pub description: String,
    pub flag: bool,
    pub input: String,
    pub main_argument: bool,
}

pub struct Command {
    pub name: &'static str,
    pub desc: &'static str,
    pub parameters: Vec<Parameter>,
}

impl Command {
    pub fn process_args(self: &Self, args: &[String]) -> Result<Vec<String>, String> {
        let mut user_input: Vec<String> = vec!["".to_string(); self.parameters.len()];
        let mut i = 1;

        let main_argument = self.parameters.iter().position(|x| x.main_argument);

        while i < args.len() {
            let arg = &args[i];
            let mut used_arg = false;
            for j in 0..self.parameters.len() {
                if arg == "-h" {
                    self.print_help();
                } else if arg == &self.parameters[j].shortcut {
                    if self.parameters[j].flag {
                        user_input[j] = "true".to_string();
                    } else if i < args.len() - 1 {
                        user_input[j] = args[i + 1].clone();
                        i += 1;
                    } else {
                        return Err("Missing argument following parameter: ".to_owned()
                            + &self.parameters[j].shortcut.to_string());
                    }
                    used_arg = true;
                    break;
                }
            }
            if !used_arg {
                if let Some(main_argument_i) = main_argument {
                    user_input[main_argument_i] = args[i].clone();
                } else {
                    return Err("Misplaced argument: ".to_owned() + &args[i].to_string());
                }
            }
            i += 1;
        }
        return Ok(user_input);
    }

    fn print_help(self: &Self) {
        println!(
            ">> [{command_name}]\
            \n|\t> {command_desc}",
            command_name = self.name,
            command_desc = self.desc
        );
        for i in 0..self.parameters.len() {
            let param = &self.parameters[i];
            println!(
                "| @ {name}\
                \n|\t{shortcut}: {description}\
                \n|\t :{input}",
                name = param.name,
                shortcut = param.shortcut,
                description = param.description,
                input = if param.flag { "flag" } else { &param.input },
            )
        }
    }
}
