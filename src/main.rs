// regchest_command_server main.rs
// use: gives access to zingo lib functions over tcp to create custom scenarios.
// authers: idky137
//

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use tokio::{runtime::Runtime, task};
use zingo_testutils::scenarios;
use zingolib::{
    commands::{self, do_user_command},
    lightclient::LightClient,
};

lazy_static! {
    static ref RT: Runtime = tokio::runtime::Runtime::new().unwrap();
}

#[derive(Debug)]
pub struct CommandData {
    com_name: &'static str,
    com_type: &'static str,
    com_req_inputs: &'static usize,
    com_opt_inputs: &'static usize,
}

pub trait CommandExec<I, O> {
    fn exec(&self, com_nametype: &str, com_inputs: I, com_options: Vec<&str>) -> O;
    fn get_command_data(&self) -> &CommandData;
}

// --- command_lib
// --- library of available functions

pub fn command_lib() -> HashMap<&'static str, Box<dyn CommandExec<CommandInput, CommandOutput>>> {
    #[allow(unused_mut)]
    let mut entries: Vec<(
        &'static str,
        Box<dyn CommandExec<CommandInput, CommandOutput>>,
    )> = vec![
        (
            "do_user_command",
            Box::new(DoUserCommand {
                command_data: CommandData {
                    com_name: "do_user_command",
                    com_type: "do_user_command",
                    com_req_inputs: &3,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::unfunded_client",
            Box::new(UnfundedClient {
                command_data: CommandData {
                    com_name: "unfunded_client",
                    com_type: "scenarios",
                    com_req_inputs: &1,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet",
            Box::new(Faucet {
                command_data: CommandData {
                    com_name: "faucet",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_recipient",
            Box::new(FaucetRecipient {
                command_data: CommandData {
                    com_name: "faucet_recipient",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_funded_recipient",
            Box::new(FaucetFundedRecipient {
                command_data: CommandData {
                    com_name: "faucet_funded_recipient",
                    com_type: "scenarios",
                    com_req_inputs: &5,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "generate_n_blocks_return_new_height",
            Box::new(GenerateNBlocksReturnNewHeight {
                command_data: CommandData {
                    com_name: "generate_n_blocks_return_new_height",
                    com_type: "generate_n_blocks_return_new_height",
                    com_req_inputs: &2,
                    com_opt_inputs: &0,
                },
            }),
        ),
        (
            "scenarios::faucet_recipient_default",
            Box::new(FaucetRecipientDefault {
                command_data: CommandData {
                    com_name: "faucet_recipient_default",
                    com_type: "scenarios",
                    com_req_inputs: &2,
                    com_opt_inputs: &2,
                },
            }),
        ),
    ];
    entries.into_iter().collect()
}

pub enum CommandInput {
    DoUserCommand((String, Vec<String>, LightClient)),
    UnfundedClient((String, String)),
    Faucet((String, String)),
    FaucetRecipient((String, String)),
    FaucetFundedRecipient((String, String)),
    GenerateNBlocksReturnNewHeight((String, String)),
    FaucetRecipientDefault((String, String)),
}

pub enum CommandOutput {
    DoUserCommand(String),
    UnfundedClient(String),
    Faucet(String),
    FaucetRecipient(String),
    FaucetFundedRecipient(String),
    GenerateNBlocksReturnNewHeight(String),
    FaucetRecipientDefault(String),
}

struct DoUserCommand {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for DoUserCommand {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::DoUserCommand((s, v, lc)) => {
                println!("test entry - DoUserCommand");
                com_out = "test entry - DoUserCommand".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::DoUserCommand(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct UnfundedClient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for UnfundedClient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::UnfundedClient((s_1, s_2)) => {
                println!("test entry - UnfundedClient");
                com_out = "test entry - UnfundedClient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::UnfundedClient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct Faucet {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for Faucet {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::Faucet((s_1, s_2)) => {
                println!("test entry - Faucet");
                com_out = "test entry - Faucet".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::Faucet(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetRecipient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetRecipient((s_1, s_2)) => {
                println!("test entry - FaucetRecipient");
                com_out = "test entry - FaucetRecipient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetRecipient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetFundedRecipient {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetFundedRecipient {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetFundedRecipient((s_1, s_2)) => {
                println!("test entry - FaucetFundedRecipient");
                com_out = "test entry - FaucetFundedRecipient".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetFundedRecipient(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct GenerateNBlocksReturnNewHeight {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for GenerateNBlocksReturnNewHeight {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::GenerateNBlocksReturnNewHeight((s_1, s_2)) => {
                println!("test entry - GenerateNBlocksReturnNewHeight");
                com_out = "test entry - GenerateNBlocksReturnNewHeight".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::GenerateNBlocksReturnNewHeight(com_out)
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

struct FaucetRecipientDefault {
    command_data: CommandData,
}
impl CommandExec<CommandInput, CommandOutput> for FaucetRecipientDefault {
    fn exec(
        &self,
        com_nametype: &str,
        com_inputs: CommandInput,
        com_options: Vec<&str>,
    ) -> CommandOutput {
        let mut com_out = String::new();
        match com_inputs {
            CommandInput::FaucetRecipientDefault((s_1, s_2)) => {
                println!("test entry - FaucetRecipientDefault");
                com_out = "test entry - FaucetRecipientDefault".to_string();
            }
            _ => {
                println!("Unexpected CommandInput variant");
                com_out = "Unexpected input".to_string();
            }
        }
        CommandOutput::FaucetRecipientDefault(com_out)
        // ---
        // let (_regtest_manager, _cph, _faucet, recipient, _txid) = RT.block_on(async move { scenarios::faucet_funded_recipient_default(100_000).await });
        // println!("test entry - FaucetRecipientDefault");
        // "test entry - FaucetRecipientDefault".to_string()
        // ---
    }
    fn get_command_data(&self) -> &CommandData {
        &self.command_data
    }
}

// --- run_com
// --- runs command received and returns output if exists
fn run_command(
    com_str: &str,
    com_nametype: &str,
    com_type: &str,
    com_name: &str,
    com_inputs_vec: Vec<&str>,
    com_options_vec: Vec<&str>,
) -> String {
    let com_lib = command_lib();
    let mut com_out = String::new();
    //let mut com_out2 = ;
    let num_com_inputs = com_inputs_vec.len();
    let num_com_options = com_options_vec.len();

    println!("In run_com:");

    println!("Loading test scenario");
    let (_regtest_manager, _cph, _faucet, recipient, _txid) =
        RT.block_on(async move { scenarios::faucet_funded_recipient_default(100_000).await });

    //let recipient2: LightClient;
    let command_inputs =
        CommandInput::DoUserCommand(("some_string".to_string(), vec![], recipient));

    for (key, value) in &com_lib {
        if com_nametype == *key {
            let com_out_3 = value.exec(&com_nametype, command_inputs, com_options_vec);
            break;
        } else {
            println!("Command not recognised");
            com_out = "Command not recognised".to_string();
        }
    }

    println!("Command complete");

    com_out
}

// --- command_handler
// --- Splits the command string into com_type, com_name, com_inputs and com_options, then checks that recieved command is valid and has correct number of inputs. If valid passes command to run_com.
fn command_handler(com_str: &str) -> String {
    let com_lib = command_lib();

    let com_reg_dot = Regex::new(r"([^\.]+)\.([^\.]+)\(([^()]*)\)(\..*)*").unwrap();
    let com_reg_col = Regex::new(r"([^\.]+)::([^\.]+)\(([^()]*)\)(\..*)*").unwrap();
    let com_reg_nak = Regex::new(r"([^\.]+)\(([^()]*)\)(\..*)*").unwrap();

    let mut run_com_out = String::new();
    let mut com_out = String::new();
    let mut com_out_trim = com_out.trim();

    println!("In command_handler:");

    if com_reg_col.is_match(com_str) {
        if let Some(captures) = com_reg_col.captures(com_str) {
            let com_type = captures.get(1).map_or("", |m| m.as_str());
            let com_name = captures.get(2).map_or("", |m| m.as_str());
            let com_inputs = captures.get(3).map_or("", |m| m.as_str());
            let com_options = captures.get(4).map_or("", |m| m.as_str());

            let com_inputs_vec: Vec<&str> = com_inputs.split(", ").collect();
            let num_com_inputs = com_inputs_vec.len();

            let com_options_vec: Vec<&str> = com_options
                .trim_start_matches('.')
                .split_terminator('.')
                .collect();

            let com_nametype = String::from(format!("{}::{}", com_type, com_name));

            for (key, value) in &com_lib {
                let command_data = value.get_command_data();
                if command_data.com_name == com_name {
                    if (num_com_inputs >= *command_data.com_req_inputs)
                        && (num_com_inputs
                            <= (*command_data.com_req_inputs + *command_data.com_opt_inputs))
                    {
                        run_com_out = run_command(
                            com_str,
                            &com_nametype,
                            com_type,
                            com_name,
                            com_inputs_vec,
                            com_options_vec,
                        );
                        com_out_trim = &run_com_out;
                        break;
                    } else {
                        com_out_trim = "Invalid number of inputs";
                        break;
                    }
                } else {
                    com_out_trim = "Invalid Command name or type";
                }
            }
        }
    } else if com_reg_dot.is_match(com_str) {
        if let Some(captures) = com_reg_dot.captures(com_str) {
            let com_type = captures.get(1).map_or("", |m| m.as_str());
            let com_name = captures.get(2).map_or("", |m| m.as_str());
            let com_inputs = captures.get(3).map_or("", |m| m.as_str());
            let com_options = captures.get(4).map_or("", |m| m.as_str());

            let com_inputs_vec: Vec<&str> = com_inputs.split(", ").collect();
            let num_com_inputs = com_inputs_vec.len();

            let com_options_vec: Vec<&str> = com_options
                .trim_start_matches('.')
                .split_terminator('.')
                .collect();

            let com_nametype = String::from(format!("{}.{}", com_type, com_name));

            for (key, value) in &com_lib {
                let command_data = value.get_command_data();
                if command_data.com_name == com_name {
                    if (num_com_inputs >= *command_data.com_req_inputs)
                        && (num_com_inputs
                            <= (*command_data.com_req_inputs + *command_data.com_opt_inputs))
                    {
                        run_com_out = run_command(
                            com_str,
                            &com_nametype,
                            com_type,
                            com_name,
                            com_inputs_vec,
                            com_options_vec,
                        );
                        com_out_trim = &run_com_out;
                        break;
                    } else {
                        com_out_trim = "Invalid number of inputs";
                        break;
                    }
                } else {
                    com_out_trim = "Invalid Command name or type";
                }
            }
        }
    } else if com_reg_nak.is_match(com_str) {
        if let Some(captures) = com_reg_nak.captures(com_str) {
            let com_name = captures.get(1).map_or("", |m| m.as_str());
            let com_inputs = captures.get(2).map_or("", |m| m.as_str());
            let com_options = captures.get(3).map_or("", |m| m.as_str());

            let com_inputs_vec: Vec<&str> = com_inputs.split(", ").collect();
            let num_com_inputs = com_inputs_vec.len();

            let com_options_vec: Vec<&str> = com_options
                .trim_start_matches('.')
                .split_terminator('.')
                .collect();

            let com_type = com_name.to_string();
            let com_nametype = com_name.to_string();

            for (key, value) in &com_lib {
                let command_data = value.get_command_data();
                if command_data.com_name == com_name {
                    if (num_com_inputs >= *command_data.com_req_inputs)
                        && (num_com_inputs
                            <= (*command_data.com_req_inputs + *command_data.com_opt_inputs))
                    {
                        run_com_out = run_command(
                            com_str,
                            &com_nametype,
                            &com_type,
                            com_name,
                            com_inputs_vec,
                            com_options_vec,
                        );
                        com_out_trim = &run_com_out;
                        break;
                    } else {
                        com_out_trim = "Invalid number of inputs";
                        break;
                    }
                } else {
                    com_out_trim = "Invalid Command name or type";
                }
            }
        }
    } else {
        com_out_trim = "The input string does not match the expected format --- <command_type>.<command>(<inputs>).<options>";
    }
    com_out_trim.to_string()
}

// --- client_handler
// --- Recieves commands and passes them to command_handler, then returns the output over TcpStream
fn client_handler(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    println!("In client_handle:");

    while let Ok(size) = stream.read(&mut buffer) {
        if size == 0 {
            break;
        }

        let command_str = String::from_utf8_lossy(&buffer[..size]);

        if command_str == "endsession" {
            println!("Client ended session, closing program");
            std::process::exit(1);
        }
        let com_response = command_handler(&command_str);
        let com_response_trim = com_response.trim();

        stream
            .write_all(com_response_trim.as_bytes())
            .expect("Errror: Failed to send response");

        let eor_flag = "\t\t";
        stream
            .write_all(eor_flag.as_bytes())
            .expect("Error: failed to send eor_flag");

        buffer = [0; 1024];
    }
}

// --- main
// --- Listens at specified IP address and port and sets up TcpStream.
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Listening at: 127.0.0.1.8080");
    print!(".. ");
    io::stdout().flush().expect("Error: failed to flush stdout");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                client_handler(stream);
                print!(".. ");
                io::stdout().flush().expect("Error: failed to flush stdout");
            }
            Err(e) => {
                eprintln!("Error: Failed to accept connection: {}", e);
            }
        }
    }
}
