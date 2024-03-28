use std::ops::RangeInclusive;
use boolinator::Boolinator;

use clap::{ArgMatches, error::{Error, ErrorKind, DefaultFormatter}};

pub mod error;
pub mod subcommands;

pub struct DefaultParser;

impl ParseArgs for DefaultParser {}

pub trait ParseArgs {
    fn parse_option_string(args: &ArgMatches, name: &str) -> Option<String> {
        args
            .get_one::<String>(name)
            .map(String::from)
    }

    fn parse_option<T: Clone + Send + Sync + Copy + 'static>
        (args: &ArgMatches, name: &str) -> Option<T> 
    {
        args
            .get_one::<T>(name)
            .map(|val| *val)
    }

    fn parse_nullable_string(args: &ArgMatches, name: &str) -> Option<Option<String>> {
        let value = args
            .get_one::<String>(name)
            .map(String::from);

        if let Some(value) = value {
            if value.trim() == "-" {
                return Some(None)
            }
            
            Some(Some(value))
        }
        else {
            None
        }
    }

    fn parse_nullable_int(args: &ArgMatches, name: &str, range: RangeInclusive<i32>) -> Option<Option<i32>> {
        let value = args
            .get_one::<String>(name)
            .map(String::from);

        if value.is_none() {
            return None;
        }

        let value = value.unwrap();

        if value.trim() == "-" {
            return Some(None);
        }

        let value: i32 = value.parse()
            .map_err(|_| {
                let e: Error<DefaultFormatter> = Error::new(ErrorKind::ValueValidation);
                e.exit()
            })
            .unwrap();

        range.contains(&value)
            .ok_or(Error::new(ErrorKind::InvalidValue))
            .map_err(|e:  Error<DefaultFormatter>| e.exit())
        .ok();

        Some(Some(value))
    }

    fn parse_vector_int(args: &ArgMatches, name: &str) -> Option<Vec<i32>> {
        if let Some(values) = args.get_many::<i32>(name) {
            return Some(values.map(|v| *v).collect())
        }

        None
    }
}
