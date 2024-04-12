use memoverlay::MemOverlay;
use std::io::Cursor;
use std::io::{self, Read, Seek};

/// test what happens without patches
#[test]
fn test_range0() {
    let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut overlay = MemOverlay::from(Cursor::new(input));
    let mut output = Vec::new();
    io::copy(&mut overlay, &mut output).unwrap();
    assert_eq!(output.as_slice(), &expected);
}

/// test a single one byte patch
#[test]
fn test_range1() {
    let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let expected = [0, 1, 2, 3, 255, 254, 6, 7, 8, 9];
    let mut overlay = MemOverlay::from(Cursor::new(input));
    overlay.add_bytes_at(4, [255, 254]).unwrap();
    let mut output = Vec::new();
    io::copy(&mut overlay, &mut output).unwrap();
    assert_eq!(output.as_slice(), &expected);

    let mut dst = [0; 4];
    overlay.seek(io::SeekFrom::Start(2)).unwrap();
    overlay.read_exact(&mut dst).unwrap();
    assert_eq!(dst, [2, 3, 255, 254]);

    let mut dst = [0; 4];
    overlay.seek(io::SeekFrom::Start(0)).unwrap();
    overlay.read_exact(&mut dst).unwrap();
    assert_eq!(dst, [0, 1, 2, 3]);

    // double patch
    let expected = [0, 1, 2, 3, 0xcc, 254, 6, 7, 8, 9];
    overlay.add_bytes_at(4, [0xcc]).unwrap();
    let mut output = Vec::new();
    overlay.seek(io::SeekFrom::Start(0)).unwrap();
    io::copy(&mut overlay, &mut output).unwrap();
    assert_eq!(output.as_slice(), &expected);
}


/// test a patch that starts within the original stream but reaches
/// over the end
#[test]
fn test_range2() {
    let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 254, 255];
    let mut overlay = MemOverlay::from(Cursor::new(input));
    overlay.add_bytes_at(9, [254, 255]).unwrap();
    let mut output = Vec::new();
    io::copy(&mut overlay, &mut output).unwrap();
    assert_eq!(output.as_slice(), &expected);
}
