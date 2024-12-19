#[derive(Debug)]
struct Event {
    time: i32,
    value: i32,
    is_start: i32,
}

impl Event {
    fn new(time: i32, value: i32, is_start: i32) -> Self {
        Event {
            time,
            value,
            is_start,
        }
    }
}

struct Solution;

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {


        let mut ans = 0;
        let mut max_value = 0;
        let mut evts: Vec<Event> = Vec::new();

        for event in events {
            let start = event[0];
            let end = event[1];
            let value = event[2];
            evts.push(Event::new(start, value, 1));
            evts.push(Event::new(end + 1, value, 0));
        }

        evts.sort_by(|a, b| {
            if a.time == b.time {
                a.is_start.cmp(&b.is_start)
            } else {
                a.time.cmp(&b.time)
            }
        });

        for evt in evts {
            if evt.is_start == 1 {
                ans = ans.max(evt.value + max_value);
            } else {
                max_value = max_value.max(evt.value);
            }
        }

        ans
    }
}

fn main() {
    let events = vec![
        vec![1, 3, 2],
        vec![4, 5, 2],
        vec![2, 4, 3],
    ];
    println!("{}", Solution::max_two_events(events)); // Output should be 4
}
