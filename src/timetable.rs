use nom::character::complete::{char, digit1, line_ending, multispace0, one_of};
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::multi::many1;
use nom::Parser;
use nom::{combinator::map_res, sequence::Tuple, IResult};

pub struct Table {
    start_hour: u8,
    end_hour: u8,
    duration: time::Duration,
}

enum Time {
    Second,
    Minute,
    Hour,
}

fn hour_span<'a, E>(content: &'a str) -> IResult<&'a str, (u8, u8), E>
where
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    map(
        context("hour_span", |val| {
            (
                map_res(context("start_hour", digit1), |start: &str| {
                    start.parse::<u8>()
                }),
                char('-'),
                map_res(context("end_hour", digit1), |end: &str| end.parse::<u8>()),
            )
                .parse(val)
        }),
        |(start, _, end)| (start, end),
    )
    .parse(content)
}

fn duration<'a, E>(content: &'a str) -> IResult<&'a str, (i64, Time), E>
where
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + nom::error::FromExternalError<&'a str, &'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    (
        map_res(context("value", digit1), |val: &str| val.parse::<i64>()),
        map_res(one_of("smh"), |val| match val {
            's' => Ok(Time::Second),
            'm' => Ok(Time::Minute),
            'h' => Ok(Time::Hour),
            _ => Err(""),
        }),
    )
        .parse(content)
}

fn duration_map<'a, E>(content: &'a str) -> IResult<&'a str, time::Duration, E>
where
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + nom::error::FromExternalError<&'a str, &'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    (map(context("value", duration), |(val, time)| match time {
        Time::Second => time::Duration::seconds(val),
        Time::Minute => time::Duration::minutes(val),
        Time::Hour => time::Duration::hours(val),
    }))
    .parse(content)
}

fn table<'a, E>(content: &'a str) -> IResult<&'a str, Table, E>
where
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + nom::error::FromExternalError<&'a str, &'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    map(
        context("table", |val| {
            (
                multispace0,
                context("hours", hour_span),
                multispace0,
                context("duration", duration_map),
            )
                .parse(val)
        }),
        |(_, (start, end), _, duration)| Table {
            start_hour: start,
            end_hour: end,
            duration,
        },
    )
    .parse(content)
}
pub fn parse<'a, E>(content: &'a str) -> IResult<&'a str, Vec<Table>, E>
where
    E: ParseError<&'a str>
        + ContextError<&'a str>
        + nom::error::FromExternalError<&'a str, &'a str>
        + nom::error::FromExternalError<&'a str, std::num::ParseIntError>,
{
    many1(map(
        context("line", |line| (table, opt(line_ending)).parse(line)),
        |(t, _)| t,
    ))
    .parse(content)
}
