// rcli csv -i input.json -o output.json --header -d ','

use clap::Parser;
use rcli::{process_csv, process_genpass, Cli, SubCommand};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("output.{}", opts.format)
            };

            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.no_uppercase,
                opts.no_lowercase,
                opts.no_number,
                opts.no_symbol,
            )?;
        }
    }

    Ok(())
}
