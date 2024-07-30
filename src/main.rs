// use serde::de::value::Error;
use serde::{Deserialize, Serialize};
// use serde_json::{json, Result};
// use std::collections::HashMap;
use homedir::my_home;

use std::fs::{canonicalize, File};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};
// use std::process::exit;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PathWeight {
    path: String,
    count: u16,
    ts: u128,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let to_path = &args[1];
    dbg!(to_path);

    let mut weights_buf = my_home().unwrap().expect("to get user home dir");
    weights_buf.push(".cd2");
    dbg!(&weights_buf);

    let weights_path = weights_buf.to_str().expect("to get str from path");

    let res = File::options().append(true).open(weights_path);

    match res {
        Ok(_) => (),
        Err(_err) => {
            let mut f = File::create(weights_path).expect("could not create weights file");
            f.write_all("[]".as_bytes()).unwrap();
        }
    };

    let matched = match_partial_path(weights_path, to_path);
    dbg!(&matched);

    let s = match matched {
        Some(w) => update_weights(weights_path, &w.path).expect("to update weights file"),
        None => update_weights(weights_path, to_path).expect("to update weights file"),
    };

    std::io::stdout().write_all(s.as_bytes())?;

    Ok(())
}

fn read_weights(weights_path: &str) -> Result<Vec<PathWeight>, serde_json::Error> {
    let contents = fs::read_to_string(weights_path).unwrap();

    serde_json::from_str(&contents)
}

fn match_partial_path(weights_path: &str, to_path: &str) -> Option<PathWeight> {
    let _to_path = Path::new(to_path);

    // if full path return a shell struct with path set to input
    if _to_path.exists() {
        let abs_path = canonicalize(_to_path)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        return Some(PathWeight {
            path: abs_path,
            count: 0,
            ts: 0,
        });
    }

    let weights: Vec<PathWeight> = read_weights(weights_path).unwrap();

    let matched: Vec<PathWeight> = weights
        .into_iter()
        .filter(|w| w.path.contains(to_path))
        .collect();

    if matched.len() == 0 {
        return None;
    }

    if matched.len() == 1 {
        return Some(matched[0].clone());
    }

    // get max ts in matched group
    let max_ts = matched.iter().max_by_key(|i| i.ts).unwrap().ts;

    // if max ts for the group is within X seconds, get next highest weight, then restart
    if now() - max_ts < 5000 {
        let idx = matched.iter().position(|w| w.ts == max_ts).unwrap();
        if matched.len() <= idx + 1 {
            return Some(matched[0].clone()); // vector is exhausted, start over
        }
        return Some(matched[idx + 1].clone());
    }

    // not within X seconds so use entry with most weight
    Some(matched[0].clone())
}

fn update_weights(weights_path: &str, to_path: &str) -> std::io::Result<String> {
    let mut weights: Vec<PathWeight> = read_weights(weights_path).unwrap();
    // println!("{:?}", d);

    let found = weights.iter().position(|w| w.path == to_path);
    match found {
        None => {
            if Path::new(to_path).exists() {
                weights.push(PathWeight {
                    path: to_path.to_string(),
                    count: 1,
                    ts: now(),
                });
            }
        }
        Some(n) => {
            weights[n].count += 1;
            weights[n].ts = now();
        }
    }

    // sort by count descending across all entries
    weights.sort_by(|a, b| b.count.cmp(&a.count));

    let j = serde_json::to_string(&weights)?;

    let mut f = File::options().write(true).open(weights_path)?;
    f.write_all(j.as_bytes())?;
    Ok(to_path.to_string())
}

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}