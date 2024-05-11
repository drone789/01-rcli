use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64Subcommand, Opts,
    SubCommand,
};

// rcli csv -i input.csv -o output.csv --header -d ','
// rcli base64 encode -i xxx.toml --format urlsafe

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
        SubCommand::Base64(subcmd) => match subcmd {
            Base64Subcommand::Encode(opts) => {
                // println!("encodeOpts: {:?}", opts);
                process_encode(&opts.input, opts.format)?;
            }
            Base64Subcommand::Decode(opts) => {
                // println!("decode: {:?}", opts);
                process_decode(&opts.input, opts.format)?;
            }
        },
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
