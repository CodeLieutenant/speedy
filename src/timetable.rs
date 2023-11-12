use std::fmt::Display;

use human_time::ToHumanTimeString;
use nom::character::complete::{char, i64, line_ending, multispace0, one_of, u8};
use nom::combinator::{map, opt};
use nom::error::{context, ContextError, ParseError};
use nom::multi::many1;
use nom::Parser;
use nom::{sequence::Tuple, IResult};

#[derive(Debug, Clone)]

pub struct Table {
    pub start_hour: u8,
    pub end_hour: u8,
    pub duration: time::Duration,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = std::time::Duration::new(
            self.duration.whole_seconds() as u64,
            self.duration.whole_nanoseconds() as u32,
        );

        f.write_fmt(format_args!(
            "{}-{} {}",
            self.start_hour,
            self.end_hour,
            d.to_human_time_string(),
        ))
    }
}

#[derive(Debug, Clone)]

enum Time {
    Second,
    Minute,
    Hour,
}

fn hour_span<'a, E>(content: &'a str) -> IResult<&'a str, (u8, u8), E>
where
    E: ParseError<&'a str> + nom::error::ContextError<&'a str>,
{
    map(
        context("hour_span", |val| (u8, char('-'), u8).parse(val)),
        |(start, _, end)| (start, end),
    )
    .parse(content)
}

fn duration<'a, E>(content: &'a str) -> IResult<&'a str, (i64, Time), E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    (
        i64,
        map(one_of("smh"), |val| match val {
            's' => Time::Second,
            'm' => Time::Minute,
            'h' => Time::Hour,
            _ => panic!("nom parser broke"),
        }),
    )
        .parse(content)
}

fn duration_map<'a, E>(content: &'a str) -> IResult<&'a str, time::Duration, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
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
    E: ParseError<&'a str> + ContextError<&'a str>,
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
            end_hour: match end {
                0 => 24,
                res => res,
            },
            duration,
        },
    )
    .parse(content)
}
pub fn parse(content: &str) -> IResult<&str, Vec<Table>, nom::error::Error<&str>> {
    many1(map(
        context("line", |line| (table, opt(line_ending)).parse(line)),
        |(t, _)| t,
    ))
    .parse(content)
}

#[cfg(test)]
mod tests {

    use super::parse;

    #[test]
    fn test_parse_single_line() {
        let result = parse("    0-1 5m");
        assert!(result.is_ok());
        let data = result.unwrap().1;

        assert_eq!(1, data.len());
        assert_eq!(0_u8, data[0].start_hour);
        assert_eq!(1_u8, data[0].end_hour);
    }
}
