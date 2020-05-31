use std::collections::HashSet;
use std::iter::FromIterator;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Serialize, Deserialize};
use serde_json::Number;

use crate::{Pole, RoutePattern};
use crate::model;
use crate::model::{ApiKey, RdfType};
use crate::model::timetable::{Timetable};

const RDF_TYPE: RdfType = "odpt:Bus";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bus {
    #[serde(rename = "dct:valid")]
    pub valid: String,
    #[serde(rename = "odpt:busNumber")]
    pub number: String,
    #[serde(rename = "odpt:busroute")]
    pub route: String,
    #[serde(rename = "odpt:busroutePattern")]
    pub route_pattern: String,
    #[serde(rename = "odpt:frequency")]
    pub frequency: Number,
    #[serde(rename = "odpt:fromBusstopPole")]
    pub from_pole: String,
    #[serde(rename = "odpt:fromBusstopPoleTime")]
    pub from_pole_time: DateTime<FixedOffset>,
    #[serde(rename = "odpt:note")]
    pub note: Option<String>,
    #[serde(rename = "odpt:operator")]
    pub operator: Option<String>,
    #[serde(rename = "odpt:startingBusstopPole")]
    pub starting_pole: String,
    #[serde(rename = "odpt:terminalBusstopPole")]
    pub terminal_pole: String,
    #[serde(rename = "odpt:toBusstopPole")]
    pub to_pole: String,
    #[serde(rename = "owl:sameAs")]
    pub same_as: String,
}

impl Bus {
    pub fn get_date(&self) -> NaiveDate {
        self.from_pole_time.date().naive_local()
    }

    pub fn get_timetable_and_index<'a>(&self, timetables: &'a[Timetable]) -> Option<(&'a Timetable, usize)> {
        let from_time = self.from_pole_time.naive_local().time();
        let timetable = timetables.iter()
            .find(|t| self.route == t.route
                && self.from_pole == t.pole)?;
        timetable.timetable_object.iter().enumerate()
            .filter(|(_, t)| t.destination_pole == self.terminal_pole)
            .min_by(|(_, t1), (_, t2)| {
                let d1 = (t1.departure_time - from_time).num_milliseconds().abs();
                let d2 = (t2.departure_time - from_time).num_milliseconds().abs();
                d1.cmp(&d2)
            }).map(|(i, _)| (timetable, i))
    }

    pub fn get_pole<'a>(&self, bus_stop: &'a [Pole]) -> Option<&'a Pole> {
        bus_stop.iter()
            .find(|p| p.route_pattern.contains(&self.route_pattern))
    }
}

pub fn get_approaching_bus<'a>(bus: &'a[Bus], poles: &[Pole], routes: &[RoutePattern]) -> Vec<&'a Bus> {
    bus.iter()
        .map(|b| {
            let pole = b.get_pole(poles)?;
            let route = routes.iter().find(|r| r.same_as == b.route_pattern)?;
            let to_index = route.pole_order.iter()
                .find(|o| o.pole == b.to_pole)
                .map(|o| o.index.as_i64())
                .flatten();
            let target_index = route.pole_order.iter()
                .find(|o| o.pole == pole.same_as)
                .map(|o| o.index.as_i64())
                .flatten();
            if to_index <= target_index {
                Some(b)
            } else {
                None
            }
        }).filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

pub fn get_bus_list(api_key: ApiKey, routes: Vec<RoutePattern>) -> Vec<Bus> {
    let result = model::get_data(api_key, RDF_TYPE, vec![("odpt:busroutePattern", &routes.iter()
        .map(|r| r.same_as.to_string())
        .collect::<Vec<String>>()
        .join(","))]);
    serde_json::from_str(&result.unwrap()).unwrap()
}

pub fn get_date_set(bus_list: Vec<Bus>) -> HashSet<NaiveDate> {
    HashSet::from_iter(bus_list.iter()
        .map(Bus::get_date))
}