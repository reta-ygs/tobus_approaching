extern crate wasm_bindgen;
extern crate ureq;
extern crate serde;
extern crate serde_json;
extern crate chrono;

use std::iter::FromIterator;
use std::collections::HashSet;

use serde::{Serialize, Deserialize};
use serde_json::{Value, Number};

use crate::model::route_pattern::RoutePattern;

pub mod model;

pub fn get_operators(api_key: &str) -> Value {
    let res = ureq::get("https://api.odpt.org/api/v4/odpt:Operator")
        .query("acl:consumerKey", api_key)
        .call();
    serde_json::from_str(res.into_string().unwrap().as_str()).unwrap()
}

pub fn get_bus_stop(api_key: &str, name: &str) -> Vec<Pole> {
    let res = ureq::get("https://api.odpt.org/api/v4/odpt:BusstopPole")
        .query("acl:consumerKey", api_key)
        .query("dc:title", name)
        .call();
    serde_json::from_str(res.into_string().unwrap().as_str()).unwrap()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pole {
    #[serde(rename = "dc:title")]
    pub name: String,
    #[serde(rename = "odpt:kana")]
    pub kana: String,
    #[serde(rename = "geo:long")]
    pub longitude: Option<Number>,
    #[serde(rename = "geo:lat")]
    pub latitude: Option<Number>,
    #[serde(rename = "odpt:busroutePattern")]
    pub route_pattern: Vec<String>,
    #[serde(rename = "odpt:operator")]
    pub operator: Vec<String>,
    #[serde(rename = "odpt:busstopPoleNumber")]
    pub pole_number: String,
    #[serde(rename = "odpt:busstopTimetable")]
    pub timetable: Option<Vec<String>>,
    #[serde(rename = "owl:sameAs")]
    pub same_as: String,
    #[serde(rename = "odpt:note")]
    pub note: Option<String>,
    #[serde(rename = "title")]
    pub title: Option<Value>
}

pub fn get_common_route(from: &Vec<Pole>, to: &Vec<Pole>) -> Vec<String> {
    let routes = |v: &Vec<Pole>| -> HashSet<String> {
        HashSet::from_iter(v.iter()
        .flat_map( | p| p.route_pattern.clone()))
    };
    routes(from).intersection(&routes(to)).cloned().collect()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoleOrder {
    #[serde(rename = "odpt:busstopPole")]
    pole: String,
    #[serde(rename = "odpt:index")]
    index: Number,
    #[serde(rename = "odpt:note")]
    note: Option<String>
}

pub fn get_route_for_destination(routes: &Vec<RoutePattern>, from: &Vec<Pole>, to: &Vec<Pole>) -> Vec<RoutePattern> {
    routes.iter()
        .filter(|r| {
            let from_index = r.get_pole_index(from);
            let to_index = r.get_pole_index(to);
            from_index < to_index
        }).map(RoutePattern::clone)
        .collect()
}