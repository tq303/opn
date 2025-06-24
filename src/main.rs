mod cli;
mod command;
mod config;

fn main() {
    let cli = cli::parse();

    // check
    match config::load_config(cli.config) {
        Ok(cfg) => match config::resolve_option(&cli.context, &cli.command, &cfg) {
            Some(cmd) => {
                println!();
                println!("Context:\n{}\n", cmd.key);

                match cli.print {
                    true => {
                        match cli.command {
                            Some(command) => {
                                println!("Command:\n{}\n", command);
                                println!("Run:\n{}", cmd.run);
                            }
                            None => {
                                println!("Run default:\n{}\n", cmd.run);
                            }
                        }
                    }
                    false => {
                        let result = command::run_command(&cmd.run, &cmd.context);

                        match result {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Failed to execute: {}", e);
                            }
                        }
                    }
                }
            }
            None => {
                eprintln!("opn for {} not found.", cli.context);
            }
        },
        Err(e) => {
            eprintln!("Failed to load config: {}", e);
        }
    }
}
