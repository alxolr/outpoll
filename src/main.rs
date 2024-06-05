use structopt::StructOpt;

mod notifier;

#[derive(Debug, structopt::StructOpt)]
struct Cli {
    #[structopt(short)]
    command: String,
    #[structopt(short)]
    watch: String,
    #[structopt(short, long, default_value = "5")]
    poll_interval: u64,
}

fn main() {
    let args = Cli::from_args();
    let command = args.command.split_whitespace().collect::<Vec<&str>>();

    loop {
        println!(
            "Checking command: '{:?}' for the output '{}' every {} seconds",
            command.join(" "),
            args.watch,
            args.poll_interval
        );
        let command_output = std::process::Command::new(command[0])
            .args(&command[1..])
            .output()
            .unwrap();

        let output_str = std::str::from_utf8(&command_output.stdout).unwrap();

        if output_str.contains(&args.watch) {
            notifier::Notifier::new().notify(&format!("Found the output: {}", &args.watch));
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(args.poll_interval));
    }
}
