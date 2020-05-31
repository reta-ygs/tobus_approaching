pub mod calendar;
pub mod bus;
pub mod route_pattern;
pub mod timetable;

pub type ApiKey = &'static str;

type RdfType = &'static str;

fn get_data(
    api_key: ApiKey,
    rdf_type: RdfType,
    key_value_params: Vec<(&str, &str)>
) -> std::io::Result<String> {
    let entry = format!("https://api.odpt.org/api/v4/{}", rdf_type);
    ureq::get(&entry)
        .query("acl:consumerKey", api_key)
        .query_str(&key_value_params.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&"))
       .call()
       .into_string()
}