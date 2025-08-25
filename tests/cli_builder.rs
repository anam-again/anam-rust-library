use hello_rust::cli_builder::{Command, Parameter};

#[test]
fn process_args() {
    const DEFAULT_MSG: &str = "Unknown error";
    {
        let cmd = Command {
            name: "",
            desc: "",
            parameters: vec![Parameter {
                shortcut: "-a".to_string(),
                name: "".to_string(),
                description: "".to_string(),
                flag: true,
                input: "".to_string(),
                main_argument: false,
            }],
        };
        {
            let inputs: &[String] = &["".to_string(), "-a".to_string()];
            let res = cmd.process_args(inputs).expect(DEFAULT_MSG);
            let exp = vec!["true"];
            assert_eq!(res, exp);
        }
        {
            let inputs: &[String] = &["".to_string()];
            let res = cmd.process_args(inputs).expect(DEFAULT_MSG);
            let exp = vec![""];
            assert_eq!(res, exp);
        }
        {
            let inputs: &[String] = &["".to_string(), "hello".to_string()];
            cmd.process_args(inputs).expect_err(DEFAULT_MSG);
        }
        {
            let inputs: &[String] = &[
                "".to_string(),
                "hello".to_string(),
                "-a".to_string(),
                "hello".to_string(),
            ];
            cmd.process_args(inputs).expect_err(DEFAULT_MSG);
        }
    }
    {
        let cmd = Command {
            name: "",
            desc: "",
            parameters: vec![Parameter {
                shortcut: "-a".to_string(),
                name: "".to_string(),
                description: "".to_string(),
                flag: false,
                input: "".to_string(),
                main_argument: false,
            }],
        };
        {
            let inputs: &[String] = &["".to_string(), "-a".to_string(), "input".to_string()];
            let res = cmd.process_args(inputs).expect(DEFAULT_MSG);
            let exp = vec!["input"];
            assert_eq!(res, exp);
        }
        {
            let inputs: &[String] = &[
                "".to_string(),
                "ok".to_string(),
                "-a".to_string(),
                "input".to_string(),
                "ok".to_string(),
            ];
            cmd.process_args(inputs).expect_err(DEFAULT_MSG);
        }
    }
    {
        let cmd = Command {
            name: "",
            desc: "",
            parameters: vec![Parameter {
                shortcut: "".to_string(),
                name: "".to_string(),
                description: "".to_string(),
                flag: false,
                input: "".to_string(),
                main_argument: true,
            }],
        };
        {
            let inputs: &[String] = &["".to_string(), "asdf".to_string()];
            let res = cmd.process_args(inputs).expect(DEFAULT_MSG);
            let exp = vec!["asdf"]; // sort of confusing; has to be final param for main
            assert_eq!(res, exp);
        }
    }
}
