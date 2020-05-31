use serde::{Serialize, Deserialize};

use crate::PoleOrder;

const RDF_TYPE: super::RdfType = "odpt:BusroutePattern";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoutePattern {
    #[serde(rename = "dc:title")]
    pub name: String,
    #[serde(rename = "odpt:kana")]
    pub kana: Option<String>,
    #[serde(rename = "odpt:busroute")]
    pub route: String,
    #[serde(rename = "odpt:busstopPoleOrder")]
    pub pole_order: Vec<PoleOrder>,
    #[serde(rename = "odpt:direction")]
    pub direction: String,
    #[serde(rename = "odpt:note")]
    pub note: Option<String>,
    #[serde(rename = "odpt:operator")]
    pub operator: String,
    #[serde(rename = "odpt:pattern")]
    pub pattern: String,
    #[serde(rename = "owl:sameAs")]
    pub same_as: String,
    #[serde(rename = "type")]
    pub types: Option<String>
}

impl RoutePattern {
    pub fn get_pole_index(&self, poles: &[crate::Pole]) -> Option<i64> {
        self.pole_order.iter()
            .find(|o| poles.iter().any(|p| o.pole == p.same_as))
            .map(|p| p.index.as_i64())
            .flatten()
    }
}

pub fn get_route_patterns(api_key: super::ApiKey, route_ids: Vec<String>) -> Vec<RoutePattern> {
    let result = super::get_data(api_key, RDF_TYPE, vec![("owl:sameAs", &route_ids.join(","))]);
    serde_json::from_str(&result.unwrap()).unwrap()
}