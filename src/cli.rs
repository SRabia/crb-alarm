use std::{ops::Add, time::Duration};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[arg(short, long, value_name = "FLOAT", default_value_t = 60.0)]
    pub frame_rate: f64,

    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Timeout(DurationTmArg),
}

#[derive(Parser, Debug, Clone)]
pub struct DurationTmArg {
    duration: String,
}
//TODO: move this
//TODO: have HMS display
impl DurationTmArg {
    //TODO: this need love... improve code here.. low prio thought
    pub fn parse(&self) -> Option<Duration> {
        let parsable = self.duration.replace('h', "h ");
        let parsable = parsable.replace('s', "s ");
        let parsable = parsable.replace('m', "m ");
        let parsable: Vec<&str> = parsable.split_whitespace().collect();

        if parsable.is_empty() {
            return None;
        }

        let mut d: Duration = Duration::new(0, 0);
        for s in parsable {
            if s.ends_with('s') {
                let n = s.strip_suffix('s')?;
                let n: u64 = n.parse().ok()?;
                d = d.add(Duration::from_secs(n));
            } else if s.ends_with('m') {
                let n = s.strip_suffix('m')?;
                let n: u64 = n.parse().ok()?;
                d = d.add(Duration::from_secs(n * 60));
            } else if s.ends_with('h') {
                let n = s.strip_suffix('h')?;
                let n: u64 = n.parse().ok()?;
                d = d.add(Duration::from_secs(n * 60 * 60));
            } else {
                return None;
            }
        }

        Some(d)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn when_having_second_should_convert_second() {
        let d = DurationTmArg {
            duration: "1s".to_string(),
        };
        assert_eq!(Duration::from_secs(1), d.parse().unwrap());
    }

    #[test]
    fn when_having_min_should_convert_min() {
        let d = DurationTmArg {
            duration: "1m".to_string(),
        };
        assert_eq!(Duration::from_secs(60), d.parse().unwrap());
    }

    #[test]
    fn when_having_hours_should_convert_hours() {
        let d = DurationTmArg {
            duration: "1h".to_string(),
        };
        assert_eq!(Duration::from_secs(60 * 60), d.parse().unwrap());
    }

    #[test]
    fn when_having_hours_min_sec_should_convert() {
        let d = DurationTmArg {
            duration: "1h1m2s".to_string(),
        };
        let total = (60 * 60) + (60) + 2;
        assert_eq!(Duration::from_secs(total), d.parse().unwrap());
    }

    #[test]
    fn when_having_hours_min_sec_in_diff_order_should_convert() {
        let d = DurationTmArg {
            duration: "39m16s2h".to_string(),
        };
        let total = (2 * (60 * 60)) + 39 * (60) + 16;
        assert_eq!(Duration::from_secs(total), d.parse().unwrap());
    }

    #[test]
    fn when_no_unit_should_return_none() {
        let d = DurationTmArg {
            duration: "3962".to_string(),
        };

        assert_eq!(None, d.parse());
    }

    #[test]
    fn when_empty_should_return_none() {
        let d = DurationTmArg {
            duration: "".to_string(),
        };

        assert_eq!(None, d.parse());
    }

    #[test]
    fn when_no_value_should_return_none() {
        let d = DurationTmArg {
            duration: "ehllo".to_string(),
        };

        assert_eq!(None, d.parse());
    }
    #[test]
    fn when_having_multi_second_should_convert_second() {
        let d = DurationTmArg {
            duration: "1s2s3s".to_string(),
        };
        assert_eq!(Duration::from_secs(6), d.parse().unwrap());
    }
}
