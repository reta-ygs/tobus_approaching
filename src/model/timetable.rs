use serde::{Serialize, Deserialize};

use super::{bus, calendar, ApiKey, RdfType};
use serde_json::Number;
use chrono::NaiveTime;
use crate::Pole;

const RDF_TYPE: RdfType = "odpt:BusstopPoleTimetable";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimetableObject {
    #[serde(rename = "odpt:busroutePattern")]
    pub route_pattern: String,
    #[serde(rename = "odpt:busroutePatternOrder")]
    pub route_pattern_order: Number,
    #[serde(with = "departure_time_format")]
    #[serde(rename = "odpt:departureTime")]
    pub departure_time: NaiveTime,
    #[serde(rename = "odpt:destinationBusstopPole")]
    pub destination_pole: String,
    #[serde(rename = "odpt:destinationSign")]
    pub destination_sign: String,
    #[serde(rename = "odpt:isMidnight")]
    pub is_midnight: bool,
    #[serde(rename = "odpt:isNonStepBus")]
    pub is_non_step_bus: bool,
}

mod departure_time_format {
    use chrono::NaiveTime;
    use serde::{self, Deserialize, Serializer, Deserializer};
    use serde::de::Error;

    const FORMAT: &'static str = "%H:%M";

    pub fn serialize<S>(
        time: &NaiveTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let s = format!("{}", time.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveTime, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&s, FORMAT).map_err(Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timetable {
    #[serde(rename = "dc:title")]
    pub title: String,
    #[serde(rename = "odpt:busDirection")]
    pub bus_direction: String,
    #[serde(rename = "odpt:busroute")]
    pub route: String,
    #[serde(rename = "odpt:busstopPole")]
    pub pole: String,
    #[serde(rename = "odpt:busstopPoleTimetableObject")]
    pub timetable_object: Vec<TimetableObject>,
    #[serde(rename = "odpt:calendar")]
    pub calendar: String,
    #[serde(rename = "odpt:note")]
    pub note: Option<String>,
    #[serde(rename = "odpt:operator")]
    pub operator: String,
    #[serde(rename = "owl:sameAs")]
    pub same_as: String,
}

pub fn get_timetable(api_key: ApiKey, bus_list: &[bus::Bus], pole_list: &[&Pole], calendars: &[&calendar::Calendar]) -> Vec<Timetable> {
    let param_from_poles = bus_list.iter()
        .map(|b| b.from_pole.to_string())
        .chain(pole_list.iter().map(|p| p.same_as.to_string()))
        .collect::<Vec<String>>()
        .join(",");
    let param_routes = bus_list.iter()
        .map(|b| b.route.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let param_calendar = calendars.iter()
        .map(|c| c.same_as.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let result = super::get_data(api_key, RDF_TYPE, vec![
        ("odpt:busstopPole", &param_from_poles),
        ("odpt:busroute", &param_routes),
        ("odpt:calendar", &param_calendar)]).unwrap();
    serde_json::from_str(&result).unwrap()
}
