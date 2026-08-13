// flow-rectpack: A library for packing rectangles into two-dimensional finite bins.
// zlib License (see LICENSE)

#![warn(missing_docs)]

//! This crates provides a library for packing rectangles into two-dimensional finite bins using
//! different heuristic methods for placement.
//!
//! The two-dimensional rectangle bin packing is a classical problem in combinatorial optimization.
//! In this problem, one is given a sequence of rectangles `(R1, R2, ... Rn), Ri = (wi, hi)` and
//! the task is to find a packing of these items into a minimum number of bins of size `(W, H)`. No two
//! rectangles may intersect or be contained inside one another. This library uses an algorithm
//! sometimes referred as `The Maximal Rectangles ALgorithm`. This algorithm stores a list of free
//! rectangles that represents the free area of the bin.
//!
//! Placement can be tweaked by using different heuristic methods such as
//! [`ShortSideFit`](crate::rbp::FreeRectHeuristic::ShortSideFit),
//! [`LongSideFit`](crate::rbp::FreeRectHeuristic::LongSideFit),
//! [`AreaFit`](crate::rbp::FreeRectHeuristic::AreaFit),
//! [`BottomLeft`](crate::rbp::FreeRectHeuristic::BottomLeft) and
//! [`ContactPoint`](crate::rbp::FreeRectHeuristic::ContactPoint).
//!
//! # Examples
//!
//! ```
//! use flow_rectpack::FreeRectHeuristic;
//! use flow_rectpack::RectsBinPack;
//!
//! // create a new bin of size 32x32 which allows rotation:
//! let mut rbp = RectsBinPack::new(32, 32, true).unwrap();
//!
//! // make sure occupancy is zero:
//! assert_eq!(rbp.get_occupancy(), 0.0);
//!
//! // add a few rects that should fit:
//! assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
//! assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
//! assert_eq!(rbp.get_occupancy(), 0.5);
//! assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
//! assert!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some());
//! assert_eq!(rbp.get_occupancy(), 1.0);
//!
//! // this rect will not fit and therefore returns None:
//! assert!(rbp.insert(1, 1, FreeRectHeuristic::BottomLeft).is_none());
//!
//! ```

#[doc(hidden)]
pub mod rbp;

// re-export types:
pub use crate::rbp::FreeRectHeuristic;
pub use crate::rbp::Rect2D;
pub use crate::rbp::RectsBinPack;
pub use crate::rbp::RectsBinPackError;
