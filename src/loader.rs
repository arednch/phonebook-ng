use csv::Reader;
use reqwest::blocking::get;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Cursor};

static ACTIVE_PFX: &str = "*";

static HEADER_FIRST_NAME: &str = "first_name";
static HEADER_LAST_NAME: &str = "name";
static HEADER_CALLSIGN: &str = "callsign";
static HEADER_PHONE_NUMBER: &str = "telephone";
static HEADER_PRIVATE: &str = "privat";

#[derive(Debug)]
pub struct Record {
    pub firstname: String,
    pub lastname: String,
    pub callsign: String,
    pub phonenumber: String,
    pub isactive: bool,
}

impl Record {
    pub fn name_for_entry(&self) -> String {
        let mut pfx = String::new();
        if self.isactive {
            pfx = ACTIVE_PFX.to_string();
        }

        match (
            self.lastname.as_str(),
            self.firstname.as_str(),
            self.callsign.as_str(),
            self.phonenumber.as_str(),
        ) {
            ("", "", "", "") => String::new(),
            ("", "", "", _) => format!("{}{}", pfx, &self.phonenumber),
            ("", "", _, _) => format!("{}{}", pfx, &self.callsign),
            ("", _, _, _) => format!("{}{} ({})", pfx, &self.firstname, &self.callsign),
            (_, "", _, _) => format!("{}{} ({})", pfx, &self.lastname, &self.callsign),
            (_, _, _, _) => format!(
                "{}{}, {} ({})",
                pfx, &self.lastname, &self.firstname, &self.callsign
            ),
        }
    }
}

pub fn load_phonebook(
    source: &str,
    hostmap: &HashMap<String, String>,
) -> Result<Vec<Record>, Box<dyn Error>> {
    let reader: Box<dyn std::io::Read> = if source.contains("://") {
        println!("loading csv from web address {:?}", source);
        let response = get(source)?.bytes()?;
        Box::new(Cursor::new(response))
    } else {
        println!("loading csv from file {:?}", source);
        let file = File::open(source)?;
        Box::new(BufReader::new(file))
    };

    let mut csv_reader = Reader::from_reader(reader);
    // Get headers and find required column indices
    let headers = csv_reader.headers()?.clone();
    let mut indices = [None; 4];

    let column_names = [
        HEADER_FIRST_NAME,
        HEADER_LAST_NAME,
        HEADER_CALLSIGN,
        HEADER_PHONE_NUMBER,
    ];
    let mut private_index: Option<usize> = None;
    for (i, header) in headers.iter().enumerate() {
        for (j, &col) in column_names.iter().enumerate() {
            if header.eq_ignore_ascii_case(col) {
                indices[j] = Some(i);
            }
        }
        if header.eq_ignore_ascii_case(HEADER_PRIVATE) {
            private_index = Some(i);
        }
    }

    // Ensure all required columns were found
    if indices.iter().any(|&index| index.is_none()) {
        return Err("missing one or more required columns".into());
    }

    let indices: Vec<usize> = indices.iter().map(|&i| i.unwrap()).collect();

    let mut records = Vec::new();
    for result in csv_reader.records() {
        let record = result?;

        // Check "private" column, if present
        if let Some(private_idx) = private_index {
            if record[private_idx].trim().eq_ignore_ascii_case("y") {
                continue; // Skip this record
            }
        }

        records.push(Record {
            firstname: record[indices[0]].to_string(),
            lastname: record[indices[1]].to_string(),
            callsign: record[indices[2]].to_string(),
            phonenumber: record[indices[3]].to_string(),
            isactive: hostmap.contains_key(record[indices[3]].to_lowercase().as_str()),
        });
    }

    // Sort the records by lastname
    records.sort_by(|a, b| a.name_for_entry().cmp(&b.name_for_entry()));
    Ok(records)
}
