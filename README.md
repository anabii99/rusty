
### how to run
```
cargo run --bin 1
```

### linker using lld
```
sudo dnf install clang, lld


- add config in .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

```

### linker using mold
```

sudo dnf install mold

- add config in .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/usr/bin/mold"]

- check binary
readelf -p .comment target/debug/main
```




### notes
- 1,
    1-10, how handling index as i32 not usize
    1-dod, add data oriented design
    1-16, build your own hashmap
    1-sicp, add sicp style
- 2, optimize for someone hate .unwrap()
    - 2-4, get rid of unwrap() with Some()
    - 2-5, use pattern matching like match with Some()
    - 2-7, using pointer dereference. pattern match and Some()
    - 2-8, the fastest one
    - 2-16, fast, short without unwrap
    - 2-20, unsafe but clean enough
    - 2-24, end of our journey. it is fast, with error handling without unwrap
    - 2-31, definitely end of our journey
    - 2-28, it is using data oriented signature, without unwrap and lot of way to handle 
    - 2-33, handling as_mut
    - 2-35, most beautiful code i ever see. without unwrap, just straight forward 
- 16, nice handling test case.
    - try not use clone(). and let see move works.
    - using to_vec could get rid clone()
- 94, using some "Option<Rc<RefCell<TreeNode>>>"
- 98,
    - had function from_vec for better input and output
    - 98-1, modified input for take null input
- 427, even it didn't had rust input so we use c++
- 652, want see some troublesome "Vec<Option<Rc<RefCell<TreeNode>>>>"
- 731, nice way to use vector pair
- 895, nice struct
- 951, 
    - 951-2, it had clean input vector without using Some()
- 1028,
    - 1028-2, the most hard i ever seen
- 1367,
    - 1367-2, painful linked list in binary tree in rust
- 1405,
    -1405-3, fun with pattern matching
- 1438-2, how to handle unwrap with some
- 1545, bit manip 1 << (n - 1)
-1593, better editorial
- 1684
    - 1684-2, our first lifetime
    - 1684-3, if you hate to_string()
- 1792, no one want to solve this
    1792-2, stackoverflow notes is really good
- 1932, it takes a lot of time for solve this things
- 2058, nice idea for using Some()
- 2326, rust really wild in this questions, vector, linkedlist
    - 2326-4, our first unsafe
- 2416, it is using trie
- 2491, really nice how key value works in different language
- 2458, hard definitely painfully hard
    - 2458-3, handling unwrap using .map(Some)
    - 2458-7. our first STACK OVERFLOW
- 2583, tree
- 2601, first |&&p| it means reference to reference 
- 2641, still troublesome
    - 2641-10, peak of parsing input output tree without Some() and make it clean
    - 2641-11, end of our journey
- 2914, how to use for loop in different ways
- 3217,
  - 3217-3, completely remove all unwrap

### textbook that i use
- Code Like a Pro in Rust, Brenden Matthews
- Idomatic Rust, Brenden Matthews
