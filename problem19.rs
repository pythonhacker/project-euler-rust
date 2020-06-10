// 1 Jan 1900 was a Monday.
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

// Return which day 1st of a month fell
// 0 -> Sunday, 1 -> Monday ... 6 -> Saturday

// use std::env;

// Return number of days on a year in feb
fn num_days_feb( year: u32 ) -> u32 {
    // println!("num days feb asked for {}", year );
    
    if year % 100 == 0 {
        if year % 400 == 0 { return 29; }
    } else if year % 4 == 0 {
        // println!("Returning 29 for {}",year);
        return 29;
    }

    // println!("Returning 28 for {}",year);    

    return 28;
}

// Return dow which 1st of a month falls on
fn day1_of_month(month: u32, year: u32) -> u32 {
    let dom: u32;
    
    if month == 1 {
        if year == 1900 {
            return 1;
        }
        else {
            dom = ((337 + num_days_feb(year -1)) % 7 + day1_of_month(1, year-1)) % 7;
            // println!("DOM for {} {} is {}",year,month,dom);
            return dom;
        }
    } else {
        let mut total_days: u32 = 0;
        // Any other months

        if month >1 {
            total_days += 31;
        }
        if month >2 {
            total_days += num_days_feb(year);
        }
        if month > 3 {
            total_days += 31;
        }
        if month > 4 {
            total_days += 30;
        }
        if month > 5 {
            total_days += 31;
        }
        if month > 6 {
            total_days += 30;
        }
        if month > 7 {
            total_days += 31;
        }
        if month > 8 {
            total_days += 31;
        }
        if month > 9 {
            total_days += 30;
        }
        if month > 10 {
            total_days += 31;
        }
        if month > 11 {
            total_days += 30;
        }
        
        dom = (total_days % 7 + day1_of_month(1, year)) % 7;
        return dom;
    }
}

fn dom_to_dow(dom: u32) -> &'static str {

    match dom {
        0 => "Sunday",
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        _ => "No day"
    }

}

fn main() {
    // Uncomment commented lines if you want to run this to
    // read year and month as cmd-line arguments.
    
//    let args: Vec<String> = env::args().collect();
    
    let mut s_count: u32 = 0;
    // let year: u32 = args[1].parse::<u32>().unwrap();
    // let month: u32 = args[2].parse::<u32>().unwrap();
    let year = 2000;
    let month = 12;
    println!("Calculating till {} {}", year, month);

    for y in 1901..year+1 {
        for m in 1..13 {
            if y == year && m == month {break;}
            let dom: u32 = day1_of_month(m, y);
            println!("{}-{}-01 is on a {}",y,m, dom_to_dow(dom));
            if dom == 0 {
                s_count += 1;
            }
        }
    }

    println!("Sunday count => {}",s_count);

}
