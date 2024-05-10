use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    // for result in reader.deserialize() {
    //      let record: Player = result?;

    //     ret.push(record);

    // }

    let headers = reader.headers()?.clone(); //读取表头
    for result in reader.records() {
        let record = result?;
        // zip 将两个迭代器合并为元组迭代器[(header, record),...]
        // collect 将元组迭代器转换为 Vec<(header, record)>

        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    // let json = serde_json::to_string_pretty(&ret)?;
    std::fs::write(output, content)?;

    Ok(())
}
