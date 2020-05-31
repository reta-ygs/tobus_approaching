extern crate tobus_approaching;

use tobus_approaching::*;
use tobus_approaching::model::bus::*;
use tobus_approaching::model::route_pattern::get_route_patterns;
use chrono::{NaiveDate};
use std::collections::HashSet;
use std::iter::FromIterator;
use tobus_approaching::model::calendar::Calendar;

const API_KEY: model::ApiKey = std::env!("ACL_CONSUMER_KEY");

#[test]
fn it_works() {

    let bus_stop1 = get_bus_stop(API_KEY, "本郷三丁目駅前");
    let bus_stop2 = get_bus_stop(API_KEY, "春日二丁目");
    let route_ids = get_common_route(&bus_stop1, &bus_stop2);
    let route_patterns = get_route_patterns(API_KEY, route_ids);
    let route_for_destination = get_route_for_destination(&route_patterns, &bus_stop1, &bus_stop2);
    let bus_list = get_bus_list(API_KEY, route_for_destination);
    let approaching_bus = get_approaching_bus(&bus_list, &bus_stop2, &route_patterns);

    let calendar = model::calendar::get_calendars(API_KEY);
    let bus_date: HashSet<NaiveDate> = HashSet::from_iter(approaching_bus.iter().map(|b| b.get_date()));
    let filtered_date = calendar.iter()
        .filter(move |c| c.contains_date(&bus_date))
        .collect::<Vec<&Calendar>>();

    let approaching_pole_list = approaching_bus.iter()
        .map(|b| b.get_pole(&bus_stop1))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect::<Vec<&Pole>>();
    let timetable = crate::model::timetable::get_timetable(
        API_KEY,
        &bus_list,
        &approaching_pole_list,
        &filtered_date);
    let result = approaching_bus.iter()
        .map(|b| {
            let timetable = &timetable[..];
            let (current_timetable, i) = b.get_timetable_and_index(timetable)?;

            let target_pole = b.get_pole(&bus_stop1)?;

            let target_timetable = timetable.iter()
                .find(|t| t.pole == target_pole.same_as)?;

            Some((
                target_pole.name.to_string(),
                b.terminal_pole.to_string(),
                b.from_pole.to_string(),
                b.from_pole_time,
                current_timetable.timetable_object.get(i).unwrap(),
                target_timetable.timetable_object.get(i).unwrap()
            ))
        })
        .collect::<Vec<_>>();
    dbg!(result);
}