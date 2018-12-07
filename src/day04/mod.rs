use chrono::{NaiveDateTime, Timelike, Datelike};
use itertools::Itertools;

use std::io;
use std::fmt::{self, Display};
use std::collections::HashMap;


#[derive(Copy,Clone,Debug,PartialEq,Eq)]
enum GuardState{
    Awake,
    Asleep,
    Begin,
}

impl Default for GuardState {
    fn default()->Self {
        GuardState::Awake
    }
}

impl Display for GuardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuardState::Awake | GuardState::Begin => write!(f, "."),
            GuardState::Asleep => write!(f, "#"),
        }
    }
}

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
struct Event {
    timestamp: NaiveDateTime,
    guard_id: usize,
    state: GuardState,
}


fn parse_event(e: &str, guardhint: usize) -> Result<Event, super::StarError> {
    if e.len() < 27 {
        return Err(format!("Input too short \"{}\"", e).into())
    }
    let date = NaiveDateTime::parse_from_str(&e[..18],"[%Y-%m-%d %H:%M]").map_err(|e| e.to_string())?;
    let mut tokens = e[19..].split('#');

    match tokens.next() {
        Some("Guard ") => {
            let guard_id = tokens.next()
                .ok_or(format!("Unexpected line ending \"{}\"",e))?
                .split_whitespace()
                .next()
                .ok_or(format!("Expected another whitespace \"{}\"",e))?
                .parse::<usize>()?;
            Ok(Event{timestamp: date, guard_id: guard_id, state: GuardState::Begin})
        },
        Some("falls asleep") => Ok(Event{timestamp: date, guard_id: guardhint, state: GuardState::Asleep}),
        Some("wakes up") => Ok(Event{timestamp: date, guard_id: guardhint, state: GuardState::Awake}),
        Some(&_) => Err("?".into()),
        None => Err(format!("Unknown event \"{}\"", e).into()),
    }
}

fn parse_events(logs: impl Iterator<Item = String>) -> Result<Vec<Event>, super::StarError> {
    let mut events: Vec<Event> = Vec::new();
    let mut lastid: usize = 0;

    for l in logs {
        let e = parse_event(&l, lastid)?;
        lastid = e.guard_id;
        events.push(e);
    }
    Ok(events)
}

#[derive(Copy,Clone)]
struct Shift {
    guard_id: usize,
    month: u32,
    day: u32,
    minutes: [GuardState; 60],
}

impl Default for Shift {
    fn default() -> Self {
        Shift {
            guard_id: 0,
            month: 0,
            day: 0,
            minutes: [GuardState::Awake; 60],
        }
    }
}
impl Display for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}-{:02}  #{:4}  ", self.month, self.day, self.guard_id)?;
        for m in self.minutes.iter() {
            write!(f, "{}", m)?;
        }
        Ok(())
    }
}

impl Shift {
    fn asleep_for(&self) -> usize {
        self.minutes.iter().filter(|s| **s == GuardState::Asleep).count()
    }
}

impl<'a> From<&'a[Event]> for Shift {
    fn from(events: &[Event]) -> Self {
        if events.len() == 0 {
            panic!("Can't build shift from no events");
        }
        let mut s: Shift = Shift::default();
        let last = events.last().unwrap();
        s.guard_id = last.guard_id;
        s.month = last.timestamp.month() as u32;
        s.day = last.timestamp.day() as u32;
        for e in events {
            for m in s.minutes[e.timestamp.minute() as usize..].iter_mut() {
                *m = e.state;
            }
        }
        s
    }
}

fn group_shifts(events: &Vec<Event>) -> Vec<Shift> {
    let mut ret: Vec<Shift> = Vec::new();
    for group in events.iter().map(|e| vec!(*e)).coalesce(|mut xe, ye| {
        if ye[0].state == GuardState::Begin {
            Err((xe, ye))
        } else {
            xe.extend(ye);
            Ok(xe)
        }
    }) {
        ret.push(Shift::from(&group[..]));
    }
    ret
}

#[allow(dead_code)]
fn print_events(shifts: &Vec<Shift>) {
    println!("Date   ID     Minute");
    println!("              000000000011111111112222222222333333333344444444445555555555");
    println!("              012345678901234567890123456789012345678901234567890123456789");
    for s in shifts {
        println!("{}", s);
    }
}

fn find_worst_guard(s: &Vec<Shift>) -> (usize, usize) {
    let mut asleep = HashMap::new();

    for e in s {
        let stat = asleep.entry(e.guard_id).or_insert(0);
        *stat += e.asleep_for();
    }

    let (gid, slept) = asleep.iter().max_by(|(_,xasleep),(_,yasleep)| xasleep.cmp(yasleep)).expect("No maximum found");
    (*gid, *slept)
}

fn find_worst_minute(shifts: &Vec<Shift>, gid: usize) -> (usize, usize) {
    let mut minutes = [0usize; 60];
    for s in shifts.iter().filter(|e| e.guard_id == gid) {
        for (i, m) in minutes.iter_mut().enumerate() {
            if s.minutes[i] == GuardState::Asleep {
                *m += 1
            }
        }
    }
    let (minute, count) = minutes.iter().enumerate().max_by(|(_,xs),(_,ys)| xs.cmp(ys)).expect("Can't find worst minute");
    (minute, *count)
}

fn build_shifts(lines: impl Iterator<Item = io::Result<String>>) -> Result<Vec<Shift>,super::StarError> {
    let mut logs = lines.collect::<Result<Vec<_>,_>>()?;
    logs.sort();
    let events = parse_events(logs.into_iter())?;
    Ok(group_shifts(&events))
}

pub fn star1(lines: impl Iterator<Item = io::Result<String>>) -> super::StarResult {
    let shifts = build_shifts(lines)?;

    //print_events(&shifts);

    let (gid, slept) = find_worst_guard(&shifts);
    let (worst, _) = find_worst_minute(&shifts, gid);
    println!("Guard #{} was asleep for {} minutes. Most of the time at minute {}. Answer {}", gid, slept, worst, gid*worst);

    Ok(())
}

pub fn star2(lines: impl Iterator<Item = std::io::Result<String>>) -> super::StarResult {
    let mut shifts = build_shifts(lines)?;
    //print_events(&shifts);
    shifts.sort_by_key(|s| s.guard_id);
    //print_events(&shifts);

    let mut stat = HashMap::new();
    for (gid, ss) in shifts.iter().group_by(|s| s.guard_id).into_iter() {
        let (minute, count) = find_worst_minute(&ss.cloned().collect(), gid);
        stat.insert(gid, (minute, count));
    }

    let (gid, (minute, count)) = stat.iter().max_by(|(_,(_,xc)),(_,(_,yc))| xc.cmp(yc)).expect("Can't find favorite minute");
    println!("Guard #{} likes minute {} the most with {} occations. Answer {}", gid, minute, count, gid*minute);
    Ok(())
}
