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
    ts: u64,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct PathWeightVec {
    weights: Vec<PathWeight>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let to_path = &args[1];
    dbg!(&to_path);

    let mut weights_buf = my_home().unwrap().expect("to get user home dir");
    weights_buf.push(".cd2");
    dbg!(&weights_buf);

    let weights_path = weights_buf.to_str().expect("to get str from path");

    let res = File::options().append(true).open(weights_path);

    match res {
        Ok(_) => (),
        Err(_err) => {
            let mut f = File::create(weights_path).expect("could not create weights file");
            f.write_all("".as_bytes()).unwrap();
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

fn read_weights(weights_path: &str) -> Vec<PathWeight> {
    let contents = fs::read_to_string(weights_path).unwrap();

    let v: PathWeightVec =
        toml::from_str(&contents).unwrap_or_else(|_| PathWeightVec { weights: vec![] });

    v.weights
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

    let weights: Vec<PathWeight> = read_weights(weights_path);

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
    if now() - max_ts < 5 {
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
    let mut weights: Vec<PathWeight> = read_weights(weights_path);
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

    sort_by_count(&mut weights);

    let t = toml::to_string(&PathWeightVec { weights: weights }).unwrap();

    let mut f = File::options().write(true).open(weights_path)?;
    f.write_all(t.as_bytes())?;

    Ok(to_path.to_string())
}

fn sort_by_count(weights: &mut Vec<PathWeight>) {
    // sort by count descending across all entries
    weights.sort_by(|a, b| b.count.cmp(&a.count));
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now() {
        assert!(now() > 0);
    }

    #[test]
    fn test_sort_by_count() {
        let mut weights = vec![
            PathWeight {
                count: 0,
                ts: 0,
                path: String::from("bogus"),
            },
            PathWeight {
                count: 1,
                ts: 0,
                path: String::from("bogus"),
            },
        ];
        assert_eq!(sort_by_count(&mut weights), weights.reverse());
    }
}
