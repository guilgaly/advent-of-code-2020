use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Document {
    pub byr: String,         // Birth Year
    pub iyr: String,         // Issue Year
    pub eyr: String,         // Expiration Year
    pub hgt: String,         // Height
    pub hcl: String,         // Hair Color
    pub ecl: String,         // Eye Color
    pub pid: String,         // Passport ID
    pub cid: Option<String>, // Country ID
}

impl Document {
    pub fn parse(doc_text: &str) -> Result<Document, String> {
        let doc_entries = doc_text
            .split_whitespace()
            .map(|entry| {
                lazy_static! {
                    static ref REGEX: Regex = Regex::new(r"([a-z]{3}):(.+)").unwrap();
                }
                REGEX
                    .captures(entry)
                    .and_then(|cap| {
                        let key = cap.get(1)?.as_str();
                        let value = cap.get(2)?.as_str();
                        Some((key, value))
                    })
                    .ok_or(format!("Failed to parse entry {}", entry))
            })
            .collect::<Result<HashMap<&str, &str>, String>>()?;

        let get_or_err = |key: &str| {
            doc_entries
                .get(key)
                .copied()
                .ok_or(format!("Missing field {}", key))
        };

        Ok(Document {
            byr: get_or_err("byr")?.to_owned(),
            iyr: get_or_err("iyr")?.to_owned(),
            eyr: get_or_err("eyr")?.to_owned(),
            hgt: get_or_err("hgt")?.to_owned(),
            hcl: get_or_err("hcl")?.to_owned(),
            ecl: get_or_err("ecl")?.to_owned(),
            pid: get_or_err("pid")?.to_owned(),
            cid: doc_entries.get("cid").map(|v| (*v).to_owned()),
        })
    }

    pub fn validate(doc: &Document) -> Result<Document, String> {
        doc.byr
            .parse::<u32>()
            .map_err(|e| e.to_string())
            .and_then(|y| {
                if y < 1920 || y > 2002 {
                    Err(format!("illegal byr {}", y))
                } else {
                    Ok(())
                }
            })?;

        doc.iyr
            .parse::<u32>()
            .map_err(|e| e.to_string())
            .and_then(|y| {
                if y < 2010 || y > 2020 {
                    Err(format!("illegal iyr {}", y))
                } else {
                    Ok(())
                }
            })?;

        doc.eyr
            .parse::<u32>()
            .map_err(|e| e.to_string())
            .and_then(|y| {
                if y < 2020 || y > 2030 {
                    Err(format!("illegal eyr {}", y))
                } else {
                    Ok(())
                }
            })?;

        lazy_static! {
            static ref HGT_REGEX: Regex = Regex::new(r"([0-9]+)(in|cm)").unwrap();
        }
        let (hgt_value, hgt_unit) = HGT_REGEX
            .captures(&doc.hgt)
            .and_then(|cap| {
                let value = cap.get(1)?.as_str().parse::<u32>().ok()?.to_owned();
                let unit = cap.get(2)?.as_str().to_owned();
                Some((value, unit))
            })
            .ok_or(format!("Failed to parse hgt {}", &doc.hgt))?;
        if hgt_unit == "cm" {
            if hgt_value < 150 || hgt_value > 193 {
                Err(format!("Illegal hgt {}", &doc.hgt))
            } else {
                Ok(())
            }
        } else if hgt_unit == "in" {
            if hgt_value < 59 || hgt_value > 76 {
                Err(format!("Illegal hgt {}", &doc.hgt))
            } else {
                Ok(())
            }
        } else {
            Err(format!("Illegal hgt unit {}", hgt_unit))
        }?;

        lazy_static! {
            static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        HCL_REGEX
            .find(&doc.hcl)
            .ok_or(format!("Illegal hcl {}", &doc.hcl))?;

        static VALID_ECLS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        if VALID_ECLS.contains(&doc.ecl.as_ref()) {
            Ok(())
        } else {
            Err(format!("Illegal ecl {}", &doc.ecl))
        }?;

        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        PID_REGEX
            .find(&doc.pid)
            .ok_or(format!("Illegal pid {}", &doc.pid))?;

        Ok(doc.clone()) // Should ideally go to a properly typed document struct
    }
}
