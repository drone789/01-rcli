use clap::Parser;
use rcli::{process_csv, process_genpass, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts)
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                format!("{}.{}", opts.input, opts.format)
            };
            process_csv(&opts.input, output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            process_genpass(
                opts.length,
                opts.upper,
                opts.lower,
                opts.number,
                opts.symbol,
            )?;
        }
    }
    Ok(())
}

// {
// let mut reader = Reader::from_path(csv_opts.input).unwrap();
// let records = reader
//     .deserialize()
//     .map(|record| record.unwrap())
//     .collect::<Vec<Player>>();
// println!("{:?}", records);

// }
