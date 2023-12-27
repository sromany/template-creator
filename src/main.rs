use clap::{arg, command, value_parser, ArgAction, Command};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command as OsCmd;

struct ProjectManager {
    pub package_manager: (&'static str, &'static str),
    pub command: &'static str,

}
static mut PROCESS_COMMAND_MAP: HashMap<&str, &str> =
    HashMap::from([("svelte", "npm create vite --template sveltet")]);

fn main() {
    // Parse arguments from cmdline
    let matches = command!() // requires `cargo` feature
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
        // SubCommands
        .subcommand(
            Command::new("create")
                .about("create svelte project with parameters")
                .arg(arg!([name] "Type of project"))
                .arg(
                    arg!(
                        -f --file <FILE> "From a custom describer file"
                    )
                    // We don't have syntax yet for optional options, so manually calling `required`
                    .required(false)
                    .value_parser(value_parser!(PathBuf)),
                ),
        )
        .get_matches();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches
        .get_one::<u8>("debug")
        .expect("Count's are defaulted")
    {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    if let Some(matches) = matches.subcommand_matches("create") {
        // "$ myapp create" was run
        // You can check the value provided by positional arguments, or option arguments
        if let Some(name) = matches.get_one::<String>("name") {
            println!("Project type: {name}");
            if let Some(config_file) = matches.get_one::<PathBuf>("file") {
                println!("Value for config: {}", config_file.display());
            }
            // Check if binaries are installed in order to create
            //      map[name] -> (&Nodejs, &Pm(packet-manager), &Command, &Args)
            //      for e in map[name] { e.exist() ? }.all()
            // Run command vite create [svelte] [default:javascript]
            if cfg!(target_os = "linux") {
                let output =
                // Process::Command::new(map[name].&Command).args([...map[name].&Args])
                OsCmd::new("sh")
                    .arg("-c")
                    .arg(format!("echo {} {}", PROCESS_COMMAND_MAP[].StrCommand, PROCESS_COMMAND_MAP["name"].))
                    .output()
                    .expect("failed to execute process");
                println!("{:?}", output.stdout);
            };
        } else {

        }

    }

    // install (p)npm svelte-routing/svelte-motion/svelte-lucide
    // manage build/run
}
