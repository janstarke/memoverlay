//! 
//! ```txt
//! +-----------------+------------+-----------------+-------+----------------------+-------+-----------------+
//! |    chunk 0      |  chunk 1   |    chunk 2      |   3   |       chunk 4        |   5   |     chunk 6     |
//! +-----------------+------------+-----------------+-------+----------------------+-------+-----------------+
//!
//!                                +-----------------+       +----------------------+
//!                                +-----------------+       +----------------------+
//!                   +-------------------+                           +---------------------+
//!                   +-------------------+                           +---------------------+
//! +---------------------------------------------------------------------------------------------------------+
//! +---------------------------------------------------------------------------------------------------------+
//! ```
//! 
//! # Usage example
//! 
//! ```
//! # use std::error::Error;
//! use std::io::{Cursor, Read, Seek, SeekFrom};
//! use memoverlay::MemOverlay;
//! use memoverlay::overlay;
//! 
//! # fn main() -> Result<(), Box<dyn Error>> {
//! let message1 = "hello, world!";
//! let mut overlay = overlay!(message1.as_bytes());
//! overlay.add_bytes_at(7, "peter".as_bytes()).unwrap();
//!
//! let mut message2 = String::new();
//! let _ = overlay.read_to_string(&mut message2).unwrap();
//! overlay.seek(SeekFrom::Start(0)).unwrap();
//!
//! assert_eq!(message2, "hello, peter!");
//!
//! overlay.add_bytes_at(1, "a".as_bytes()).unwrap();
//! let mut message3 = String::new();
//! let _ = overlay.read_to_string(&mut message3).unwrap();
//! overlay.seek(SeekFrom::Start(0)).unwrap();
//! assert_eq!(message3, "hallo, peter!");
//!
//! overlay.add_bytes_at(1, "o".as_bytes()).unwrap();
//! overlay.add_bytes_at(5, "w".as_bytes()).unwrap();
//! let mut message4 = String::new();
//! let _ = overlay.read_to_string(&mut message4).unwrap();
//! assert_eq!(message4, "hollow peter!");
//! # Ok(())
//! # }
//! ```

mod memoverlay;
mod patch;
mod traits;
mod error;
mod patch_layer;
mod patch_search_result;

pub use crate::memoverlay::*;
pub use patch::*;
pub use traits::*;
pub use error::*;
pub use patch_layer::*;
pub use patch_search_result::*;

#[macro_export]
macro_rules! overlay {
    ($s: expr) => {
        MemOverlay::from(Cursor::new($s))
    };
}

#[cfg(test)]
mod tests {
    use crate::MemOverlay;
    use std::io::{Cursor, Read, SeekFrom, Seek};

    #[test]
    fn double_mod_single_byte() {
        let message1 = "hello, world!";
        let mut overlay = overlay!(message1.as_bytes());
        overlay.add_bytes_at(1, "a".as_bytes()).unwrap();
        overlay.add_bytes_at(1, "o".as_bytes()).unwrap();

        let mut message2 = String::new();
        let _ = overlay.read_to_string(&mut message2).unwrap();
        assert_eq!(message2, "hollo, world!");
    }

    #[test]
    fn double_mod_multiple_bytes() {
        let message1 = "hello, world!";
        let mut overlay = overlay!(message1.as_bytes());
        overlay.add_bytes_at(7, "peter".as_bytes()).unwrap();
        overlay.add_bytes_at(7, "claus".as_bytes()).unwrap();

        let mut message2 = String::new();
        let _ = overlay.read_to_string(&mut message2).unwrap();
        assert_eq!(message2, "hello, claus!");
    }


    #[test]
    fn double_mod_overlapping1() {
        let message1 = "hello, world!";
        let mut overlay = overlay!(message1.as_bytes());
        overlay.add_bytes_at(3, "XXXX".as_bytes()).unwrap();
        overlay.add_bytes_at(5, "YYYY".as_bytes()).unwrap();

        let mut message2 = String::new();
        let _ = overlay.read_to_string(&mut message2).unwrap();
        assert_eq!(message2, "helXXYYYYrld!");
    }

    #[test]
    fn double_mod_overlapping2() {
        let message1 = "hello, world!";
        let mut overlay = overlay!(message1.as_bytes());
        overlay.add_bytes_at(5, "YYYY".as_bytes()).unwrap();
        overlay.add_bytes_at(3, "XXXX".as_bytes()).unwrap();

        let mut message2 = String::new();
        let _ = overlay.read_to_string(&mut message2).unwrap();
        assert_eq!(message2, "helXXXXYYrld!");
    }

    #[test]
    fn doc_test() {
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
        overlay.seek(SeekFrom::Start(0)).unwrap();
        assert_eq!(message3, "hallo, peter!");
        
        overlay.add_bytes_at(1, "o".as_bytes()).unwrap();
        overlay.add_bytes_at(5, "w".as_bytes()).unwrap();
        let mut message4 = String::new();
        let _ = overlay.read_to_string(&mut message4).unwrap();
        assert_eq!(message4, "hollow peter!");
    }
}