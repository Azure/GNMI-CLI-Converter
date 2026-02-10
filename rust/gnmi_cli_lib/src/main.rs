use clap::Parser;
use gnmi_cli_lib::parser::CommandParser;
use gnmi_cli_lib::registry::create_default_registry;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to JSON file containing gNMI data. If not provided, reads from stdin.
    #[arg(short, long)]
    input: Option<String>,

    /// CLI command to convert (e.g., "show interfaces status")
    #[arg(short, long)]
    command: Option<String>,

    /// Internal command key to use (e.g., "interfaces/status")
    #[arg(short, long)]
    key: Option<String>,

    /// List all supported CLI commands
    #[arg(long)]
    list_commands: bool,

    /// Filter commands by pattern
    #[arg(long, default_value = "")]
    pattern: String,

    /// Output file path
    #[arg(short, long)]
    output: Option<String>,

    /// Output format (text or json)
    #[arg(long, default_value = "text")]
    format: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn read_input(path: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(path) = path {
        Ok(fs::read_to_string(path)?)
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}

fn write_output(output: &str, path: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = path {
        fs::write(path, output)?;
    } else {
        print!("{}", output);
    }
    Ok(())
}

fn list_supported_commands(pattern: &str) -> String {
    let registry = create_default_registry();
    let cli_mappings = registry.list_cli_mappings();
    let mut commands: Vec<String> = cli_mappings.keys().cloned().collect();
    commands.sort();

    let mut lines = vec![
        "Supported CLI Commands:".to_string(),
        "=".repeat(60),
        String::new(),
    ];
    let mut count = 0usize;

    for command in commands {
        if !pattern.is_empty() && !command.to_lowercase().contains(&pattern.to_lowercase()) {
            continue;
        }
        count += 1;
        if let Some(mapping) = cli_mappings.get(&command) {
            lines.push(format!("  {command}"));
            lines.push(format!(
                "    Key: {}/{}",
                mapping.command, mapping.subcommand
            ));
        }
    }

    lines.push(String::new());
    lines.push(format!("Total: {count} commands"));
    lines.join("\n")
}

fn parse_key(key: &str) -> Option<(String, String)> {
    if key.contains(' ') {
        let parts: Vec<&str> = key.split_whitespace().collect();
        if parts.len() >= 2 {
            return Some((parts[0].to_string(), parts[1..].join(" ")));
        }
    }

    for delimiter in ['/', ':'] {
        if let Some((left, right)) = key.split_once(delimiter) {
            return Some((left.to_string(), right.to_string()));
        }
    }

    None
}

fn run_command(args: &Args, input_override: Option<String>) -> Result<String, String> {
    if args.list_commands {
        return Ok(list_supported_commands(&args.pattern));
    }

    if args.command.is_none() && args.key.is_none() {
        return Err("Must provide --command or --key".to_string());
    }

    let input_str = if let Some(input) = input_override {
        input
    } else {
        read_input(&args.input).map_err(|err| err.to_string())?
    };

    let json_data: Value = serde_json::from_str(&input_str).map_err(|err| err.to_string())?;

    let parser = CommandParser::new();

    let result = if let Some(cli_command) = args.command.as_deref() {
        parser
            .parse_and_convert(cli_command, json_data)
            .map_err(|e| e.to_string())
    } else if let Some(key) = args.key.as_deref() {
        let (command, subcommand) = parse_key(key)
            .ok_or_else(|| "Invalid --key format. Use command/subcommand".to_string())?;
        parser
            .convert(&command, &subcommand, json_data, None)
            .map_err(|e| e.to_string())
    } else {
        Err("Missing command".to_string())
    };

    match result {
        Ok(output) => {
            if args.format == "json" {
                serde_json::to_string_pretty(&serde_json::json!({ "result": output }))
                    .map_err(|err| err.to_string())
            } else {
                Ok(output)
            }
        }
        Err(err) => Err(err),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match run_command(&args, None) {
        Ok(output) => {
            let formatted = if args.list_commands {
                format!("{}\n", output)
            } else {
                output
            };
            write_output(&formatted, &args.output)?;
            Ok(())
        }
        Err(err) => {
            if args.verbose {
                eprintln!("Conversion error: {err}");
            } else {
                eprintln!("Error: {err}");
            }
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_key_variants() {
        assert_eq!(
            parse_key("interfaces/status"),
            Some(("interfaces".to_string(), "status".to_string()))
        );
        assert_eq!(
            parse_key("interfaces status"),
            Some(("interfaces".to_string(), "status".to_string()))
        );
        assert_eq!(
            parse_key("clock:time"),
            Some(("clock".to_string(), "time".to_string()))
        );
        assert_eq!(parse_key("invalid"), None);
    }

    #[test]
    fn list_supported_commands_output() {
        let output = list_supported_commands("");
        assert!(output.contains("Supported CLI Commands:"));
    }

    #[test]
    fn read_write_roundtrip() {
        let dir = std::env::temp_dir();
        let input_path = dir.join("gnmi_cli_lib_test_input.json");
        std::fs::write(&input_path, "{\"key\": \"value\"}").expect("write input");

        let input =
            read_input(&Some(input_path.to_string_lossy().to_string())).expect("read input");
        assert!(input.contains("value"));

        let output_path = dir.join("gnmi_cli_lib_test_output.txt");
        write_output(
            "output",
            &Some(output_path.to_string_lossy().to_string()),
        )
        .expect("write output");
        let output = std::fs::read_to_string(&output_path).expect("read output");
        assert_eq!(output, "output");
    }

    #[test]
    fn run_command_variants() {
        let args = Args {
            input: None,
            command: None,
            key: None,
            list_commands: true,
            pattern: "doesnotexist".to_string(),
            output: None,
            format: "text".to_string(),
            verbose: false,
        };
        let output = run_command(&args, None).expect("list commands");
        assert!(output.contains("Total: 0 commands"));

        let args = Args {
            input: None,
            command: None,
            key: None,
            list_commands: false,
            pattern: "".to_string(),
            output: None,
            format: "text".to_string(),
            verbose: false,
        };
        let err = run_command(&args, None).expect_err("missing command");
        assert!(err.contains("Must provide --command or --key"));

        let args = Args {
            input: None,
            command: None,
            key: Some("invalid".to_string()),
            list_commands: false,
            pattern: "".to_string(),
            output: None,
            format: "text".to_string(),
            verbose: false,
        };
        let err = run_command(&args, Some("{}".to_string())).expect_err("invalid key");
        assert!(err.contains("Invalid --key format"));

        let args = Args {
            input: None,
            command: Some("show clock".to_string()),
            key: None,
            list_commands: false,
            pattern: "".to_string(),
            output: None,
            format: "json".to_string(),
            verbose: false,
        };
        let output =
            run_command(&args, Some("{\"date\": \"x\"}".to_string())).expect("clock output");
        assert!(output.contains("\"result\""));
    }

    #[test]
    fn write_output_stdout() {
        write_output("output", &None).expect("stdout write");
    }
}
