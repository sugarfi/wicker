use crate::error::error;
use std::collections::{HashMap, HashSet};

pub enum Validator {
    MustHave(String),
    CanHave(String),
}

pub fn validate_flags(validators: &[Validator], flags: &HashSet<String>) -> bool {
    let mut can = Vec::new();

    for i in validators.iter() {
        match i {
            Validator::MustHave(x) => {
                if !flags.contains(x) {
                    error(format!("expected flag {}", x));
                    return false;
                }
                can.push(x.clone());
            }

            Validator::CanHave(x) => {
                can.push(x.clone());
            }
        }
    }

    let i = can.iter().cloned().collect::<HashSet<String>>();

    if flags.difference(&i).collect::<Vec<&String>>().len() > 0 {
        let i = can.iter().cloned().collect::<HashSet<String>>();
        error("invalid flags".to_string());
        for n in flags.difference(&i) {
            println!("  {}", n);
        }
        return false;
    } else {
        return true;
    }
}

pub fn validate_vals(validators: &[Validator], vals: &HashMap<String, String>) -> bool {
    let mut can = Vec::new();
    let vals = vals.keys().cloned().collect::<HashSet<String>>();

    for i in validators.iter() {
        match i {
            Validator::MustHave(x) => {
                if !vals.contains(x) {
                    error(format!("expected flag {}", x));
                    return false;
                }
                can.push(x.clone());
            }

            Validator::CanHave(x) => {
                can.push(x.clone());
            }
        }
    }

    let i = can.iter().cloned().collect::<HashSet<String>>();

    if vals.difference(&i).collect::<Vec<&String>>().len() > 0 {
        error("invalid flags".to_string());
        for n in vals.difference(&i) {
            println!("  {}", n);
        }
        return false;
    } else {
        return true;
    }
}
