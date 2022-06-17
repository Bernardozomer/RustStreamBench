use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::net::IpAddr;
use std::ops::Add;

use anyhow::{Result, Ok, Context};
use chrono::{prelude::*, DurationRound, Duration};
use maxminddb::geoip2::{City, Country};
use rayon::prelude::*;

/// Parses a logfile in the [Common Log Format](https://en.wikipedia.org/wiki/Common_Log_Format).
pub fn parse_log(filename: &str, ip_db_path: &str, threads: usize) -> Result<()> {
    rayon::ThreadPoolBuilder::new().num_threads(threads).build_global()?;

    let fp = File::open(filename)?;

    // Extract relevant data fields from each entry in file.
    BufReader::new(fp).lines()
        .into_iter()
        .par_bridge()
        .for_each(|res| {
            let entry = res.unwrap();
            let fields: Vec<&str> = entry.split_whitespace().collect();

            // Number of occurrences of each city.
            let mut city_freq = HashMap::<u32, usize>::new();
            // Number of occurrences of each country.
            let mut country_freq = HashMap::<u32, usize>::new();
            // Number of visits per minute.
            let mut visits_per_min = HashMap::<DateTime<FixedOffset>, usize>::new();
            // Number of occurrences of each status code.
            let mut status_freq = HashMap::<String, usize>::new();
    
            if fields.len() != 10 {
                println!("Illegal entry in file: {}", entry);
            }
    
            // Ignore any errors found during parsing of the ip address
            // because it may not be found in the database.
            let _ = parse_ip(
                ip_db_path, &fields, &mut city_freq, &mut country_freq
            );
    
            // Count the number of visits per minute.
            let timestamp: DateTime<FixedOffset> = DateTime::parse_from_str(
                format!("{} {}", fields[3], fields[4]).as_str(),
                "[%d/%b/%Y:%H:%M:%S %z]"
            ).unwrap().duration_round(Duration::minutes(1)).unwrap();
    
            visits_per_min.insert(
                timestamp,
                visits_per_min.get(&timestamp).map_or(1, |v| v.add(1))
            );
    
            // Count the number of occurrences of each status code.
            let status_code = fields[8];
    
            status_freq.insert(
                status_code.to_string(),
                status_freq.get(status_code).map_or(1, |v| v.add(1))
            );
        });

    Ok(())
}

fn parse_ip(
    ip_db_path: &str, fields: &Vec<&str>,
    city_freq: &mut HashMap<u32, usize>, country_freq: &mut HashMap<u32, usize>
) -> Result<()> {
    let reader = maxminddb::Reader::open_readfile(ip_db_path)?;

    // Get the location of this IP address in order to
    // count the number of occurrences of each city and country.
    let ip: IpAddr = fields[0].parse()?;

    let city = reader.lookup::<City>(ip)?.city
        .context(format!("Failed to lookup city for {}", ip))?;

    let city_id = city.geoname_id
        .context("Failed to obtain city id")?;

    city_freq.insert(
        city_id,
        city_freq.get(&city_id).map_or(1, |v| v.add(1))
    );

    let country = reader.lookup::<Country>(ip)?.country
        .context(format!("Failed to lookup country for {}", ip))?;

    let country_id = country.geoname_id
        .context("Failed to obtain city id")?;

    country_freq.insert(
        country_id,
        country_freq.get(&country_id).map_or(1, |v| v.add(1))
    );

    Ok(())
}
