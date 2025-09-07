// Copyright (c) 2025 Arc Asumity
// Licensed under the GPLv3 or later License.
// See LICENSE file for details.
//
// src/conf.rs
// Parse and create the configuration file.

use std::collections::{HashMap, BTreeMap, BTreeSet};
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnsConfig {
    config: HashMap<String, AnsConfigDomain>
}

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
struct AnsConfigDomain {
    records: BTreeSet<AnsConfigDomainRecord>,
    safe_rule: BTreeSet<IpTreeMap<AnsConfigDomainRule>>
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
struct AnsConfigDomainRecord {
    record_type: String,
    record_rule: BTreeSet<IpTreeMap<BTreeMap<String, String>>>
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
struct AnsConfigDomainRule {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    frequency: u32
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
struct IpTreeMap<T: std::cmp::Ord> {
    root: BTreeSet<IpTreeMapEntry<T>>,
    ip_type: String
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Clone)]
enum IpTreeMapEntry<T: std::cmp::Ord> {
    LastLayer(BTreeSet<IpTreeMapEntry<T>>),
    Content { mask: u16, content: T}
}

impl AnsConfig {
    pub fn create(path: &str) -> Self {
        println!("Test: path:{}", path);
        // TODO Create
        AnsConfig {
            config: HashMap::new()
        }
    }
}
