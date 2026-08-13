// flow-rectpack: A library for packing rectangles into two-dimensional finite bins.
// public domain License

use flow_rectpack::FreeRectHeuristic;
use flow_rectpack::RectsBinPack;

#[test]
fn insert_short_side_fit() {
    let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ShortSideFit)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.25);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ShortSideFit)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.5);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ShortSideFit)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.75);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ShortSideFit)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::ShortSideFit).is_none());
}

#[test]
fn insert_short_side_fit_rotated() {
    let mut rbp = RectsBinPack::new(32, 16, true).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(
        rbp.insert(16, 32, FreeRectHeuristic::ShortSideFit)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::ShortSideFit).is_none());
}

#[test]
fn insert_long_side_fit() {
    let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 16, FreeRectHeuristic::LongSideFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.25);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::LongSideFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.5);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::LongSideFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.75);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::LongSideFit).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::LongSideFit).is_none());
}

#[test]
fn insert_long_side_fit_rotated() {
    let mut rbp = RectsBinPack::new(32, 16, true).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 32, FreeRectHeuristic::LongSideFit).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::LongSideFit).is_none());
}

#[test]
fn insert_area_fit() {
    let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 16, FreeRectHeuristic::AreaFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.25);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::AreaFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.5);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::AreaFit).is_some());
    assert_eq!(rbp.get_occupancy(), 0.75);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::AreaFit).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::AreaFit).is_none());
}

#[test]
fn insert_area_fit_rotated() {
    let mut rbp = RectsBinPack::new(32, 16, true).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 32, FreeRectHeuristic::AreaFit).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::AreaFit).is_none());
}

#[test]
fn insert_bottom_left() {
    let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
    assert_eq!(rbp.get_occupancy(), 0.25);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
    assert_eq!(rbp.get_occupancy(), 0.5);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
    assert_eq!(rbp.get_occupancy(), 0.75);
    assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::BottomLeft).is_none());
}

#[test]
fn insert_bottom_left_rotated() {
    let mut rbp = RectsBinPack::new(32, 16, true).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(rbp.insert(16, 32, FreeRectHeuristic::BottomLeft).is_some());
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::BottomLeft).is_none());
}

#[test]
fn insert_contact_point() {
    let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ContactPoint)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.25);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ContactPoint)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.5);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ContactPoint)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 0.75);
    assert!(
        rbp.insert(16, 16, FreeRectHeuristic::ContactPoint)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::ContactPoint).is_none());
}

#[test]
fn insert_contact_point_rotated() {
    let mut rbp = RectsBinPack::new(32, 16, true).unwrap();
    assert_eq!(rbp.get_occupancy(), 0.0);

    assert!(
        rbp.insert(16, 32, FreeRectHeuristic::ContactPoint)
            .is_some()
    );
    assert_eq!(rbp.get_occupancy(), 1.0);

    assert!(rbp.insert(1, 1, FreeRectHeuristic::ContactPoint).is_none());
}
