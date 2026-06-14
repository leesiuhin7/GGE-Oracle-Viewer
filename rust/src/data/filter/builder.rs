use crate::data::{
    block::{AllianceField, BasicField, Data, FactionField, Field, TimerField},
    filter::{
        Filter, Interval, IntervalSet,
        field_filters::{FieldFilter, Predicate},
    },
    structures::{delta_rle, rle},
};

fn filter_timestamps<F: Fn(i64) -> bool>(timestamp_deltas: Vec<i64>, predicate: F) -> IntervalSet {
    let mut interval_set = Vec::new();

    let mut start: Option<u32> = None;
    let mut end: u32 = 0;
    let mut accumulator: i64 = 0;

    for delta in timestamp_deltas {
        accumulator += delta;
        let result = predicate(accumulator);

        match (result, start) {
            (true, None) => start = Some(end),
            (false, Some(s)) => {
                if let Ok(interval) = Interval::new(s, end) {
                    interval_set.push(interval);
                }
                start = None;
            }

            _ => {}
        }
        end += 1;
    }
    if let Some(s) = start
        && let Ok(interval) = Interval::new(s, end)
    {
        interval_set.push(interval);
    }
    interval_set
}

fn filter_rle_delta<F: Fn(Option<i64>) -> bool>(
    rle_delta: Vec<delta_rle::Run>,
    predicate: F,
) -> IntervalSet {
    let mut interval_set = Vec::new();

    let mut start: Option<u32> = None;
    let mut end: u32 = 0;
    let mut accumulator: i64 = 0;

    for delta_rle::Run { delta, count } in rle_delta {
        if let Some(d) = delta {
            for _ in 0..count {
                accumulator += d;
                match (predicate(Some(accumulator)), start) {
                    (true, None) => start = Some(end),
                    (false, Some(s)) => {
                        if let Ok(interval) = Interval::new(s, end) {
                            interval_set.push(interval);
                        }
                        start = None;
                    }
                    _ => {}
                }
                end += 1;
            }
        } else {
            match (predicate(None), start) {
                (true, None) => start = Some(end),
                (false, Some(s)) => {
                    if let Ok(interval) = Interval::new(s, end) {
                        interval_set.push(interval);
                    }
                    start = None;
                }
                _ => {}
            }
            end += u32::try_from(count).unwrap();
        }
    }
    if let Some(s) = start
        && let Ok(interval) = Interval::new(s, end)
    {
        interval_set.push(interval);
    }
    interval_set
}

fn filter_rle<T, F: Fn(T) -> bool>(rle_data: Vec<rle::Run<T>>, predicate: F) -> IntervalSet {
    let mut interval_set = Vec::new();

    let mut start: Option<u32> = None;
    let mut end: u32 = 0;

    for rle::Run { value, count } in rle_data {
        match (predicate(value), start) {
            (true, None) => start = Some(end),
            (false, Some(s)) => {
                if let Ok(interval) = Interval::new(s, end) {
                    interval_set.push(interval);
                }
                start = None;
            }
            _ => {}
        }
        // Not expecting count to be very large
        end += u32::try_from(count).unwrap();
    }
    if let Some(s) = start
        && let Ok(interval) = Interval::new(s, end)
    {
        interval_set.push(interval);
    }
    interval_set
}

#[allow(clippy::too_many_lines)]
pub fn build_filter(filter_field: FieldFilter) -> Filter {
    match filter_field {
        FieldFilter::Header(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Header,
                Box::new(move |data| {
                    let Data::Header(header) = data else {
                        panic!();
                    };
                    if predicate(header) {
                        vec![Interval::new(0, u32::MAX).unwrap()]
                    } else {
                        Vec::new()
                    }
                }),
            )
        }
        FieldFilter::Timestamp(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Timestamp,
                Box::new(move |data| {
                    let Data::Timestamps(timestamp_deltas) = data else {
                        panic!();
                    };
                    filter_timestamps(timestamp_deltas, &predicate)
                }),
            )
        }
        FieldFilter::BasicName(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Name),
                Box::new(move |data| {
                    let Data::RleString(rle_name) = data else {
                        panic!();
                    };
                    filter_rle(rle_name, &predicate)
                }),
            )
        }
        FieldFilter::BasicLevel(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Level),
                Box::new(move |data| {
                    let Data::RleI64(rle_level) = data else {
                        panic!();
                    };
                    filter_rle(rle_level, &predicate)
                }),
            )
        }
        FieldFilter::BasicLegendaryLevel(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::LegendaryLevel),
                Box::new(move |data| {
                    let Data::RleI64(rle_level) = data else {
                        panic!();
                    };
                    filter_rle(rle_level, &predicate)
                }),
            )
        }
        FieldFilter::BasicMight(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Might),
                Box::new(move |data| {
                    let Data::RleDelta(rle_might) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_might, &predicate)
                }),
            )
        }
        FieldFilter::BasicHonor(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Honor),
                Box::new(move |data| {
                    let Data::RleI64(rle_honor) = data else {
                        panic!();
                    };
                    filter_rle(rle_honor, &predicate)
                }),
            )
        }
        FieldFilter::BasicAchievement(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Achievement),
                Box::new(move |data| {
                    let Data::RleI64(rle_achievement) = data else {
                        panic!();
                    };
                    filter_rle(rle_achievement, &predicate)
                }),
            )
        }
        FieldFilter::BasicGlory(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Glory),
                Box::new(move |data| {
                    let Data::RleDelta(rle_glory) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_glory, &predicate)
                }),
            )
        }
        FieldFilter::BasicRuins(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Basic(BasicField::Ruins),
                Box::new(move |data| {
                    let Data::RleI64(rle_ruins) = data else {
                        panic!();
                    };
                    filter_rle(rle_ruins, &predicate)
                }),
            )
        }
        FieldFilter::AllianceId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Alliance(AllianceField::Id),
                Box::new(move |data| {
                    let Data::RleI64(rle_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_id, &predicate)
                }),
            )
        }
        FieldFilter::AllianceName(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Alliance(AllianceField::Name),
                Box::new(move |data| {
                    let Data::RleString(rle_name) = data else {
                        panic!();
                    };
                    filter_rle(rle_name, &predicate)
                }),
            )
        }
        FieldFilter::AllianceRankId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Alliance(AllianceField::RankId),
                Box::new(move |data| {
                    let Data::RleI64(rle_rank_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_rank_id, &predicate)
                }),
            )
        }
        FieldFilter::AllianceSearching(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Alliance(AllianceField::Searching),
                Box::new(move |data| {
                    let Data::RleI64(rle_searching) = data else {
                        panic!();
                    };
                    filter_rle(rle_searching, &predicate)
                }),
            )
        }
        FieldFilter::TimerProtectionTime(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Timer(TimerField::ProtectionTime),
                Box::new(move |data| {
                    let Data::RleDelta(rle_time) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_time, &predicate)
                }),
            )
        }
        FieldFilter::TimerRelocateTime(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Timer(TimerField::RelocateTime),
                Box::new(move |data| {
                    let Data::RleDelta(rle_time) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_time, &predicate)
                }),
            )
        }
        FieldFilter::Location(_) => {
            todo!();
            #[allow(unreachable_code, unused_variables)]
            let predicate = move |value| false;
            Filter::new(
                Field::Location,
                Box::new(move |data| {
                    let Data::RleLocations(rle_locations) = data else {
                        panic!()
                    };
                    filter_rle(rle_locations, predicate)
                }),
            )
        }
        FieldFilter::CoatOfArms(_) => {
            todo!();
            #[allow(unreachable_code, unused_variables)]
            let predicate = move |value| false;
            Filter::new(
                Field::CoatOfArms,
                Box::new(move |data| {
                    let Data::RleCoatOfArms(rle_coat_of_arms) = data else {
                        panic!()
                    };
                    filter_rle(rle_coat_of_arms, predicate)
                }),
            )
        }
        FieldFilter::FactionFactionId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::FactionId),
                Box::new(move |data| {
                    let Data::RleI64(rle_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_id, &predicate)
                }),
            )
        }
        FieldFilter::FactionTitleId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::TitleId),
                Box::new(move |data| {
                    let Data::RleI64(rle_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_id, &predicate)
                }),
            )
        }
        FieldFilter::FactionSelfProtectionTime(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::SelfProtectionTime),
                Box::new(move |data| {
                    let Data::RleDelta(rle_time) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_time, &predicate)
                }),
            )
        }
        FieldFilter::FactionGroupProtectionStatus(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::GroupProtectionStatus),
                Box::new(move |data| {
                    let Data::RleI64(rle_status) = data else {
                        panic!();
                    };
                    filter_rle(rle_status, &predicate)
                }),
            )
        }
        FieldFilter::FactionGroupProtectionTime(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::GroupProtectionTime),
                Box::new(move |data| {
                    let Data::RleDelta(rle_time) = data else {
                        panic!();
                    };
                    filter_rle_delta(rle_time, &predicate)
                }),
            )
        }
        FieldFilter::FactionMainCampId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::MainCampId),
                Box::new(move |data| {
                    let Data::RleI64(rle_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_id, &predicate)
                }),
            )
        }
        FieldFilter::FactionSpecialCampId(filter) => {
            let predicate = move |value| filter.test(value);
            Filter::new(
                Field::Faction(FactionField::SpecialCampId),
                Box::new(move |data| {
                    let Data::RleI64(rle_id) = data else {
                        panic!();
                    };
                    filter_rle(rle_id, &predicate)
                }),
            )
        }
    }
}
