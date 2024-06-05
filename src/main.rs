/// It is a cli tool which will have a poll on a specific command waiting for a specific
/// output to give results
mod notifier;

fn main() {
    let command_argument = std::env::args().nth(1).expect("Command not provided");

    let command = command_argument.split_whitespace().collect::<Vec<&str>>();
    let watch_for = std::env::args().nth(2).expect("Watch for not provided");

    loop {
        println!(
            "Checking command: {:?} | grep {watch_for}",
            command.join(" ")
        );
        let command_output = std::process::Command::new(command[0])
            .args(&command[1..])
            .output()
            .unwrap();

        let output_str = std::str::from_utf8(&command_output.stdout).unwrap();

        if output_str.contains(&watch_for) {
            notifier::Notifier::new().notify(&format!("Found the output: {}", watch_for));
            break;
        } else {
            println!("Output not found: {}", watch_for);
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
