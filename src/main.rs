// main.rs - USS 1:47-6/11/25 Dorset Company
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("polling-station.Sheet1.csv")?;
    let reader = BufReader::new(file);
    let mut writer = csv::Writer::from_path("polling_stations_adamaoua.csv")?;

    writer.write_record(&[
        "Region",
        "Commune",
        "PollingStationId",
        "PollingStationName",
        "VillageOrQuarter",
        "Effectif",
    ])?;

    let mut current_region = String::new();
    let mut current_commune = String::new();
    let mut ps_cunter = 1;

    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split(",").map(|c| c.trim()).collect();

        //     Region detection
        if columns.get(0) == Some(&"REGION:") {
            if let Some(region) = columns.get(1) {
                current_region = region.to_string();
            }
            continue;
        }
        //     Commune detection
        if columns.get(0) == Some(&"COMMUNE:") || columns.get(0) == Some(&"COUNCIL") {
            if let Some(commune) = columns.get(1) {
                current_commune = commune.to_string();
            }
            continue;
        }
        //     ADAMAOUA Filter only
        if current_region != "ADAMAOUA" {
            continue;
        }
        //     Polling station list
        if columns.len() >= 6 {
            let name = columns[1];
            let village = columns[2];
            let effectif = columns[5];

            if !name.is_ascii() && !village.is_empty() && !effectif.is_empty() {
                let ps_id = format!("ps{}", ps_cunter);
                ps_cunter += 1;
                writer.write_record(&[
                    &current_region,
                    &current_commune,
                    &ps_id,
                    name,
                    village,
                    effectif,
                ])?;
            }
        }
    }
    writer.flush()?;
    println!("✅ Data for adamoua extracted\nOutput: polling_stations_adamaoua.csv");
    Ok(())
}
