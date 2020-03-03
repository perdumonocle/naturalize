# naturalize

Convert a string to a convenient view for natural sorting.
E.g., output string may be stored into database for ordering by.

## Examples:

```rust
use naturalize::to_natural;

let nat = to_natural("abc123def").unwrap();
assert_eq!(nat, "abc0000000123def");
```

```rust
use naturalize::to_natural;

let nat = to_natural("").unwrap();
assert_eq!(nat, "");
```

```rust
use naturalize::to_natural;

let nat = to_natural("1020").unwrap();
assert_eq!(nat, "0000001020");
```

```rust
use naturalize::to_natural;

let nat = to_natural("102030405060708090").unwrap();
assert_eq!(nat, "102030405060708090");
```

```rust
use naturalize::to_natural;

let nat = to_natural("Hello").unwrap();
assert_eq!(nat, "Hello");
```

```rust
use naturalize::to_natural;

let nat = to_natural("10 apples").unwrap();
assert_eq!(nat, "0000000010 apples");
```

```rust
use naturalize::to_natural;

let nat = to_natural("apples 10").unwrap();
assert_eq!(nat, "apples 0000000010");
```

```rust
use naturalize::to_natural;

let nat = to_natural("172.29.21.151").unwrap();
assert_eq!(nat, "0000000172.0000000029.0000000021.0000000151");
```

```rust
use naturalize::to_natural;

let nat = to_natural("IP = 172.29.21.151").unwrap();
assert_eq!(nat, "IP = 0000000172.0000000029.0000000021.0000000151");
```
