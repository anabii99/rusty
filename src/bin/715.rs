#[derive(Debug)]
struct RangeModule {
    ranges: std::collections::VecDeque<(i32, i32)>,
}

impl RangeModule {
    fn new() -> Self {
        Self {
            ranges: std::collections::VecDeque::new(),
        }
    }

    fn add_range(&mut self, left: i32, right: i32) {
        if self._add_range(left, right).is_none() {
            println!("add_range({}, {}) failed", left, right);
        }
    }
    fn _add_range(&mut self, left: i32, right: i32) -> Option<()> {
        let mut left = left;
        let mut right = right;
        let mut i = 0;
        while i < self.ranges.len() {
            let (l, r) = self.ranges.get(i)?;
            let (l, r) = (*l, *r);
            if right < l {
                break;
            } else if left > r {
                i += 1;
            } else {
                left = left.min(l);
                right = right.max(r);
                self.ranges.remove(i);
            }
        }
        self.ranges.insert(i, (left, right));
        Some(())
    }

    fn query_range(&self, left: i32, right: i32) -> bool {
        self._query_range(left, right).unwrap_or_default()
    }
    fn _query_range(&self, left: i32, right: i32) -> Option<bool> {
        let mut i = 0;
        let mut j = self.ranges.len() as i32 - 1;
        while i <= j {
            let mid = (i + j) / 2;
            let (l, r) = self.ranges.get(mid as usize)?;
            let (l, r) = (*l, *r);
            if right < l {
                j = mid - 1;
            } else if left > r {
                i = mid + 1;
            } else {
                return Some(left >= l && right <= r);
            }
        }
        Some(false)
    }

    fn remove_range(&mut self, left: i32, right: i32) {
        if self._remove_range(left, right).is_none() {
            println!("remove_range({}, {}) failed", left, right);
        }
    }
    fn _remove_range(&mut self, left: i32, right: i32) -> Option<()> {
        let mut i = 0;
        while i < self.ranges.len() {
            let (l, r) = self.ranges.get(i)?;
            let (l, r) = (*l, *r);
            if right < l {
                break;
            } else if left > r {
                i += 1;
            } else {
                if left > l {
                    self.ranges.insert(i, (l, left));
                    i += 1;
                }
                if right < r {
                    self.ranges.insert(i, (right, r));
                    i += 1;
                }
                self.ranges.remove(i);
            }
        }
        Some(())
    }
}

fn main() {
    let mut range_module = RangeModule::new();

    // Add ranges
    range_module.add_range(5, 10);
    println!("Added range (5, 10): {:?}", range_module);

    range_module.add_range(15, 20);
    println!("Added range (15, 20): {:?}", range_module);

    range_module.add_range(8, 18); // This will merge the previous ranges into one (5, 20)
    println!("Added range (8, 18) and merged ranges: {:?}", range_module);

    // Query ranges
    let query1 = range_module.query_range(5, 10); // Should return true
    println!("Query range (5, 10): {}", query1);

    let query2 = range_module.query_range(10, 15); // Should return true, as it overlaps the merged range (5, 20)
    println!("Query range (10, 15): {}", query2);

    let query3 = range_module.query_range(20, 25); // Should return false, outside the existing ranges
    println!("Query range (20, 25): {}", query3);

    // Remove ranges
    range_module.remove_range(6, 9);
    println!("Removed range (6, 9): {:?}", range_module);

    range_module.remove_range(10, 12);
    println!("Removed range (10, 12): {:?}", range_module);

    let query4 = range_module.query_range(5, 10); // Should return false, as part of it was removed
    println!("Query range (5, 10): {}", query4);

    let query5 = range_module.query_range(15, 20); // Should return true, as it still exists
    println!("Query range (15, 20): {}", query5);
}
