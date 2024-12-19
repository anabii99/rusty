struct HashTableEntry {
    key: i32,
    value: i32,
    next: Option<usize>,
}

struct HashTable {
    buckets: Vec<Option<usize>>,
    entries: Vec<HashTableEntry>,
    size: usize,
    entry_count: usize,
}

impl HashTable {
    fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![None; size],
            entries: Vec::with_capacity(size * 2),
            size,
            entry_count: 0,
        }
    }

    fn custom_hash(&self, key: i32) -> usize {
        (key.abs() % self.size as i32) as usize
    }

    fn insert(&mut self, key: i32, value: i32) {
        let index = self.custom_hash(key);
        let mut entry_index = self.buckets[index];

        while let Some(entry) = entry_index {
            if self.entries[entry].key == key {
                self.entries[entry].value = value;
                return;
            }
            entry_index = self.entries[entry].next;
        }

        let new_entry_index = self.entry_count;
        self.entries.push(HashTableEntry {
            key,
            value,
            next: self.buckets[index],
        });
        self.buckets[index] = Some(new_entry_index);
        self.entry_count += 1;
    }

    fn search(&self, key: i32) -> Option<i32> {
        let index = self.custom_hash(key);
        let mut entry_index = self.buckets[index];

        while let Some(entry) = entry_index {
            if self.entries[entry].key == key {
                return Some(self.entries[entry].value);
            }
            entry_index = self.entries[entry].next;
        }

        None
    }
}

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let size = nums.len();
        let mut ht = HashTable::new(size * 2);

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(found_index) = ht.search(complement) {
                return vec![found_index, i as i32];
            }
            ht.insert(num, i as i32);
        }

        vec![]
    }
}

fn main() {
    let nums = vec![0, 4, 3, 0];
    let target = 0;

    let indices = Solution::two_sum(nums, target);

    if !indices.is_empty() {
        println!("Indices: {}, {}", indices[0], indices[1]);
    } else {
        println!("No solution found.");
    }
}
