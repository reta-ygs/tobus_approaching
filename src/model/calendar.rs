use std::collections::HashSet;

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

use crate::model::{RdfType, get_data, ApiKey};

const RDF_TYPE: RdfType = "odpt:Calendar";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Calendar {
    #[serde(rename = "dc:title")]
    pub title: Option<String>,
    #[serde(rename = "odpt:day")]
    pub days: Vec<NaiveDate>,
    #[serde(rename = "owl:sameAs")]
    pub same_as: String,
    #[serde(rename = "odpt:duration")]
    pub duration: String,
    #[serde(rename = "odpt:operator")]
    pub operator: String,
}

impl Calendar {
    pub fn contains_date(&self, date_list: &HashSet<chrono::NaiveDate>) -> bool {
        self.days.iter().any(|d| date_list.contains(d))
    }
}

pub fn get_calendars(api_key: ApiKey) -> Vec<Calendar> {
    serde_json::from_str(get_data(api_key, RDF_TYPE, vec![("odpt:operator", "odpt.Operator:Toei")])
        .unwrap().as_str())
        .unwrap()
}