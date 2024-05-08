use clap::Parser;
use rcli::{process_csv, Opts, SubCommand};

// rcli csv -i input.csv -o output.csv --header -d ','

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts)
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
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
