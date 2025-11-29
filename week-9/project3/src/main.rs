use std::fs::File;
use std::io::{Write, Result};

fn main() -> Result<()> {
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Eteye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];
    let mut combined: Vec<(usize, &str, &str, &str)> = Vec::new();

    for i in 0..names.len() {
        combined.push((i + 1, names[i], ministries[i], zones[i]));
    }
    let mut file = File::create("EFCC_Report.txt")?;
    writeln!(file, "S/N\t| NAME\t\t\t\t| MINISTRY\t\t| GEOPOLITICAL ZONE")?;
    writeln!(file, "--------------------------------------------------------------------------------------")?;

    for record in combined {
        writeln!(
            file,
            "{}\t| {}\t| {}\t| {}",
            record.0, record.1, record.2, record.3
        )?;
    }

    println!("EFCC_Report.txt has been generated successfully!");
    Ok(())
}