extern crate roaring;

use roaring::RoaringBitmap;

#[test]
fn array() {
    let mut bitmap1: RoaringBitmap<u32> = (0..2000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (1000..3000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..1000u32).chain(2000..3000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn array_and_bitmap() {
    let mut bitmap1: RoaringBitmap<u32> = (0..2000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (1000..8000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..1000u32).chain(2000..8000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn bitmap_to_bitmap() {
    let mut bitmap1: RoaringBitmap<u32> = (0..12000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (6000..18000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..6000u32).chain(12000..18000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn bitmap_to_array() {
    let mut bitmap1: RoaringBitmap<u32> = (0..6000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (2000..7000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..2000u32).chain(6000..7000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn bitmap_and_array_to_bitmap() {
    let mut bitmap1: RoaringBitmap<u32> = (0..12000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (11000..14000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..11000u32).chain(12000..14000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn bitmap_and_array_to_array() {
    let mut bitmap1: RoaringBitmap<u32> = (0..6000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (3000..7000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..3000u32).chain(6000..7000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn arrays() {
    let mut bitmap1: RoaringBitmap<u32> = (0..2000u32).chain(1000000..1002000u32).chain(3000000..3001000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (1000..3000u32).chain(1001000..1003000u32).chain(2000000..2000001u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..1000u32).chain(1000000..1001000u32).chain(2000..3000u32).chain(1002000..1003000u32).chain(2000000..2000001u32).chain(3000000..3001000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}

#[test]
fn bitmaps() {
    let mut bitmap1: RoaringBitmap<u32> = (0..6000u32).chain(1000000..1012000u32).chain(3000000..3010000u32).collect();
    let bitmap2: RoaringBitmap<u32> = (3000..7000u32).chain(1006000..1018000u32).chain(2000000..2010000u32).collect();
    let bitmap3: RoaringBitmap<u32> = (0..3000u32).chain(1000000..1006000u32).chain(6000..7000u32).chain(1012000..1018000u32).chain(2000000..2010000u32).chain(3000000..3010000u32).collect();

    bitmap1.symmetric_difference_with(&bitmap2);

    assert_eq!(bitmap1, bitmap3);
}
