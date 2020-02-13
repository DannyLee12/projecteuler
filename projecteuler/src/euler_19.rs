/*


You are given the following information, but you may prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

*/
use std::collections::HashMap;
use std::time::Instant;

pub fn main() {
    let febs = febs();
    let now = Instant::now();
    assert_eq!(febs[&1901], 28);
    assert_eq!(febs[&1912], 29);
    assert_eq!(febs[&1928], 29);
    assert_eq!(febs[&1988], 29);
    assert_eq!(febs[&2000], 29);
    let mut sundays = 0;
    let mut weekday = 1;
    for year in 1901..2001 {
        for month in 1..13 {
            let mut day = 1;
            loop {
                weekday += 1;
                if weekday == 7 {
                    weekday = 0
                }
                if day == 1 && weekday == 0 {
                    sundays += 1;
                    println!("{}, {}, {}, {}", year, month, day, weekday);
                }
                if month == 1 || month == 3 || month == 5 || month == 7 || month == 8 || month == 10 || month == 12 {
                    if day == 31 {
                        break
                    }
                }
                else if month == 2 {
                    if day == febs[&year] { break }
                }
                else {
                    if day == 30 { break }
                }
                day += 1;
            }
        }
        println!("Total Sundays: {}", sundays)
    }
    println!("Script took {} seconds to run", now.elapsed().as_secs());
}

fn febs() -> HashMap<i32, i32> {
    let mut f = HashMap::new();
    let mut v: i32;
    for x in 1900..2001{
        if x % 100 == 0 {
            if x % 400 == 0 {
                v = 29;
            }
            else {
                v = 28;
            }
        }
        else if x % 4 == 0 {
            v = 29;
        }
        else
        {
            v = 28;
        }
        f.insert(x, v);
    }
    f
}