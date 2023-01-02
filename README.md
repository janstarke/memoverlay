# memoverlay
Puts a writable layer of bytes over some byte stream
# memoverlay


```txt
+-----------------+------------+-----------------+-------+----------------------+-------+-----------------+
|    chunk 0      |  chunk 1   |    chunk 2      |   3   |       chunk 4        |   5   |     chunk 6     |
+-----------------+------------+-----------------+-------+----------------------+-------+-----------------+

                            +-----------------+       +----------------------+
                            +-----------------+       +----------------------+
                +-------------------+                           +---------------------+
                +-------------------+                           +---------------------+
+---------------------------------------------------------------------------------------------------------+
+---------------------------------------------------------------------------------------------------------+
```

## Usage example

```rust
use std::io::{Cursor, Read, Seek, SeekFrom};
use memoverlay::MemOverlay;
use memoverlay::overlay;

let message1 = "hello, world!";
let mut overlay = overlay!(message1.as_bytes());
overlay.add_bytes_at(7, "peter".as_bytes()).unwrap();

let mut message2 = String::new();
let _ = overlay.read_to_string(&mut message2).unwrap();
overlay.seek(SeekFrom::Start(0)).unwrap();

assert_eq!(message2, "hello, peter!");

overlay.add_bytes_at(1, "a".as_bytes()).unwrap();

let mut message3 = String::new();
let _ = overlay.read_to_string(&mut message3).unwrap();
assert_eq!(message3, "hallo, peter!");

assert_eq!(message1, "hello, world!");
