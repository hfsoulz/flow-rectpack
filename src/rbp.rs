// flow-rectpack: A library for packing rectangles into two-dimensional finite bins.
// zlib License (see LICENSE)

/// Specifies the different heuristic rules that can be used when deciding where to place a new
/// rectangle.
#[derive(Clone, Debug)]
pub enum FreeRectHeuristic {
    /// Choose to pack `R` into such `Fi` that `min(wf - w, hf - h)` is the smallest. In other words, we
    /// minimize the length of the shorter leftover side.
    ShortSideFit,

    /// Pack `R` into an `Fi` such that `max(wf - w, hf - h)` is the smallest. That is, we minimize
    /// the length of the longer leftover side.
    LongSideFit,

    /// Pick the `Fi ∈ F` that is smallest in area to place the next rectangle `R` into. If there is a
    /// tie, we use the [`ShortSideFit`](crate::rbp::FreeRectHeuristic::ShortSideFit) rule to break it.
    AreaFit,

    /// Orient and place each rectangle to the position where the y-coordinate of the top side of the
    /// rectangle is the smallest and if there are several such valid positions, pick the one that has
    /// the smallest x-coordinate value.
    BottomLeft,

    /// Place `R` into a position where the length of the perimeter of `R` that is touched by the bin
    /// edge or by a previously packed rectangle is maximized.
    ContactPoint,
}

/// Specifies the different error types that can occur.
#[derive(PartialEq, Clone, Debug)]
pub enum RectsBinPackError {
    /// Invalid argument
    InvalidArg,
}

/// Specifies the properties of a 2D rectangle.
#[derive(Default, Clone, Debug)]
pub struct Rect2D {
    /// is the x offset
    pub x: i32,

    /// is the y offset
    pub y: i32,

    /// is the width
    pub width: i32,

    /// is the height
    pub height: i32,
}

impl Rect2D {
    /// Instantiates a 2D rectangle with given size properties.
    ///
    /// # Arguments
    ///
    /// * `x` - is the x offset.
    /// * `y` - is the y offset.
    /// * `width` - is the width.
    /// * `height` - is the height.
    ///
    /// # Errors
    ///
    /// [`InvalidArg`](crate::rbp::RectsBinPackError::InvalidArg)
    /// is returned if `x < 0 || y < 0 || width <= 0 || height <= 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_rectpack::Rect2D;
    ///
    /// let rect = Rect2D::with_details(0, 0, 32, 16).unwrap();
    /// assert_eq!(rect.x, 0);
    /// assert_eq!(rect.y, 0);
    /// assert_eq!(rect.width, 32);
    /// assert_eq!(rect.height, 16);
    ///
    /// // this should fail:
    /// assert_eq!(Rect2D::with_details(0, 0, 0, 0).is_err(), true);
    /// ```
    pub fn with_details(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, RectsBinPackError> {
        if x >= 0 && y >= 0 && width > 0 && height > 0 {
            Ok(Self {
                x,
                y,
                width,
                height,
            })
        } else {
            Err(RectsBinPackError::InvalidArg)
        }
    }
}

/// Specifies the properties of a rectangle bin.
#[derive(Clone, Debug)]
pub struct RectsBinPack {
    /// is the width of the bin
    width: i32,

    /// is the height of the bin
    height: i32,

    /// is the flag indicating whether rotation is allowed or not
    allow_flip: bool,

    /// is the vector holding the used rects
    used_rects: Vec<Rect2D>,

    /// is the vector holding the free rects
    free_rects: Vec<Rect2D>,
}

impl RectsBinPack {
    /// Instantiates a empty bin of given size.
    ///
    /// # Arguments
    ///
    /// * `width` - is the width of the bin
    /// * `height` - is the height of the bin
    /// * `allow_flip` - is the flag indicating whether the packing algorithm is allowed to rotate
    ///   the input rectangle 90 degrees clockwise to consider a better placement.
    ///
    /// # Errors
    ///
    /// [`RectsBinPackError::InvalidArg`](crate::rbp::RectsBinPackError)
    /// is returned if `width <= 0 || height <= 0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_rectpack::RectsBinPack;
    /// use flow_rectpack::FreeRectHeuristic;
    ///
    /// let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    /// assert_eq!(rbp.insert(33, 33, FreeRectHeuristic::BottomLeft).is_none(), true);
    /// ```
    pub fn new(width: i32, height: i32, allow_flip: bool) -> Result<Self, RectsBinPackError> {
        if width > 0 && height > 0 {
            Ok(Self {
                width,
                height,
                allow_flip,
                used_rects: Vec::new(),
                free_rects: vec![Rect2D::with_details(0, 0, width, height).unwrap()],
            })
        } else {
            Err(RectsBinPackError::InvalidArg)
        }
    }

    /// Insert a single rectangle into the bin, possibly rotated.
    ///
    /// # Arguments
    ///
    /// * `width` - is the rectangle width
    /// * `height` - is the rectangle height
    /// * `heuristic` - is the heuristic method to use when packing
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_rectpack::RectsBinPack;
    /// use flow_rectpack::FreeRectHeuristic;
    ///
    /// let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    /// assert_eq!(rbp.insert(33, 33, FreeRectHeuristic::BottomLeft).is_none(), true);
    /// ```
    pub fn insert(
        &mut self,
        width: i32,
        height: i32,
        heuristic: FreeRectHeuristic,
    ) -> Option<Rect2D> {
        let output = match heuristic {
            FreeRectHeuristic::ShortSideFit => self.get_rect_for_best_short_side_fit(width, height),
            FreeRectHeuristic::BottomLeft => self.get_rect_for_bottom_left(width, height),
            FreeRectHeuristic::ContactPoint => self.get_rect_for_contact_point(width, height),
            FreeRectHeuristic::LongSideFit => self.get_rect_for_best_long_side_fit(width, height),
            FreeRectHeuristic::AreaFit => self.get_rect_for_best_area_fit(width, height),
        };

        let new_rect = output?;
        let mut i: usize = 0;
        while i < self.free_rects.len() {
            if let Some(free_rect) = self.free_rects.get(i)
                && self.is_split_free_node(&free_rect.clone(), &new_rect)
            {
                self.free_rects.remove(i);
                continue;
            }

            i += 1;
        }

        self.prune_free_list();
        self.used_rects.push(new_rect.clone());

        Some(new_rect)
    }

    /// Computes the ratio of used surface area to the total bin area.
    ///
    /// # Examples
    ///
    /// ```
    /// use flow_rectpack::RectsBinPack;
    /// use flow_rectpack::FreeRectHeuristic;
    ///
    /// let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
    ///
    /// // occupancy should be 0.0 initially:
    /// assert_eq!(rbp.get_occupancy(), 0.0);
    ///
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    /// assert_eq!(rbp.get_occupancy(), 0.25);
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    /// assert_eq!(rbp.get_occupancy(), 0.5);
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    /// assert_eq!(rbp.get_occupancy(), 0.75);
    /// assert_eq!(rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(), true);
    ///
    /// // occupancy should now be full as in 1.0:
    /// assert_eq!(rbp.get_occupancy(), 1.0);
    /// ```
    pub fn get_occupancy(&self) -> f32 {
        let mut used_surface_area: i32 = 0;

        for i in 0..self.used_rects.len() {
            if let Some(rect) = self.used_rects.get(i) {
                used_surface_area += rect.width * rect.height;
            }
        }

        // return occupancy:
        used_surface_area as f32 / (self.width * self.height) as f32
    }

    /// Computes the placement score for the contact point variant.
    fn get_score_for_contact_point(&self, x: i32, y: i32, width: i32, height: i32) -> i32 {
        let mut score: i32 = 0;

        if x == 0 || x + width == self.width {
            score += height;
        }

        if y == 0 || y + height == self.height {
            score += width;
        }

        for i in 0..self.used_rects.len() {
            if let Some(used_rect) = self.used_rects.get(i) {
                if used_rect.x == x + width || used_rect.x + used_rect.width == x {
                    score += self.get_common_interval_len(
                        used_rect.y,
                        used_rect.y + used_rect.height,
                        y,
                        y + height,
                    );
                }

                if used_rect.y == y + height || used_rect.y + used_rect.height == y {
                    score += self.get_common_interval_len(
                        used_rect.x,
                        used_rect.x + used_rect.width,
                        x,
                        x + width,
                    );
                }
            }
        }

        score
    }

    /// Computes the rect for bottom left placement variant.
    fn get_rect_for_bottom_left(&self, width: i32, height: i32) -> Option<Rect2D> {
        let mut new_rect = Rect2D::default();

        let mut best_x = i32::MAX;
        let mut best_y = i32::MAX;

        for i in 0..self.free_rects.len() {
            let free_rect = self.free_rects.get(i)?;
            // true to place the rect in upright (non-flipped) orientation:
            if free_rect.width >= width && free_rect.height >= height {
                let top_side_y = free_rect.y + height;

                if top_side_y < best_y || (top_side_y == best_y && free_rect.x < best_x) {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = width;
                    new_rect.height = height;
                    best_x = free_rect.x;
                    best_y = top_side_y;
                }
            }

            if self.allow_flip && free_rect.width >= height && free_rect.height >= width {
                let top_side_y = free_rect.y + width;

                if top_side_y < best_y || (top_side_y == best_y && free_rect.x < best_x) {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = height;
                    new_rect.height = width;
                    best_x = free_rect.x;
                    best_y = top_side_y;
                }
            }
        }

        if new_rect.height == 0 || new_rect.width == 0 {
            return None;
        }

        Some(new_rect)
    }

    /// Computes the rect for short side fit variant.
    fn get_rect_for_best_short_side_fit(&self, width: i32, height: i32) -> Option<Rect2D> {
        let mut new_rect = Rect2D::default();

        let mut best_short_side_fit = i32::MAX;
        let mut best_long_side_fit = i32::MAX;

        for i in 0..self.free_rects.len() {
            let free_rect = self.free_rects.get(i)?;
            // try to place the rect in upright (non-flipped) orientation:
            if free_rect.width >= width && free_rect.height >= height {
                let left_over_horiz = free_rect.width - width;
                let left_over_vert = free_rect.height - height;
                let short_side_fit = std::cmp::min(left_over_horiz, left_over_vert);
                let long_side_fit = std::cmp::max(left_over_horiz, left_over_vert);

                if short_side_fit < best_short_side_fit
                    || (short_side_fit == best_short_side_fit && long_side_fit < best_long_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = width;
                    new_rect.height = height;
                    best_short_side_fit = short_side_fit;
                    best_long_side_fit = long_side_fit;
                }
            }

            if self.allow_flip && free_rect.width >= height && free_rect.height >= width {
                let flipped_left_over_horiz = free_rect.width - height;
                let flipped_left_over_vert = free_rect.height - width;
                let flipped_short_side_fit =
                    std::cmp::min(flipped_left_over_horiz, flipped_left_over_vert);
                let flipped_long_side_fit =
                    std::cmp::max(flipped_left_over_horiz, flipped_left_over_vert);

                if flipped_short_side_fit < best_short_side_fit
                    || (flipped_short_side_fit == best_short_side_fit
                        && flipped_long_side_fit < best_long_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = height;
                    new_rect.height = width;
                    best_short_side_fit = flipped_short_side_fit;
                    best_long_side_fit = flipped_long_side_fit;
                }
            }
        }

        if new_rect.height == 0 || new_rect.width == 0 {
            return None;
        }

        Some(new_rect)
    }

    /// Computes the rect for long side fit variant.
    fn get_rect_for_best_long_side_fit(&self, width: i32, height: i32) -> Option<Rect2D> {
        let mut new_rect = Rect2D::default();

        let mut best_short_side_fit = i32::MAX;
        let mut best_long_side_fit = i32::MAX;

        for i in 0..self.free_rects.len() {
            let free_rect = self.free_rects.get(i)?;
            // try to place the rect in upright (non-flipped) orientation:
            if free_rect.width >= width && free_rect.height >= height {
                let left_over_horiz = free_rect.width - width;
                let left_over_vert = free_rect.height - height;
                let short_side_fit = std::cmp::min(left_over_horiz, left_over_vert);
                let long_side_fit = std::cmp::max(left_over_horiz, left_over_vert);

                if long_side_fit < best_long_side_fit
                    || (long_side_fit == best_long_side_fit && short_side_fit < best_short_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = width;
                    new_rect.height = height;
                    best_short_side_fit = short_side_fit;
                    best_long_side_fit = long_side_fit;
                }
            }

            if self.allow_flip && free_rect.width >= height && free_rect.height >= width {
                let left_over_horiz = free_rect.width - height;
                let left_over_vert = free_rect.height - width;
                let short_side_fit = std::cmp::min(left_over_horiz, left_over_vert);
                let long_side_fit = std::cmp::max(left_over_horiz, left_over_vert);

                if long_side_fit < best_long_side_fit
                    || (long_side_fit == best_long_side_fit && short_side_fit < best_short_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = height;
                    new_rect.height = width;
                    best_short_side_fit = short_side_fit;
                    best_long_side_fit = long_side_fit;
                }
            }
        }

        if new_rect.height == 0 || new_rect.width == 0 {
            return None;
        }

        Some(new_rect)
    }

    /// Computes the rect for best area fit variant.
    fn get_rect_for_best_area_fit(&self, width: i32, height: i32) -> Option<Rect2D> {
        let mut new_rect = Rect2D::default();

        let mut best_area_fit = i32::MAX;
        let mut best_short_side_fit = i32::MAX;

        for i in 0..self.free_rects.len() {
            let free_rect = self.free_rects.get(i)?;
            let area_fit = free_rect.width * free_rect.height - width * height;

            // try to place rect in upright (non-flipped) orientation:
            if free_rect.width >= width && free_rect.height >= height {
                let left_over_horiz = free_rect.width - width;
                let left_over_vert = free_rect.height - height;
                let short_side_fit = std::cmp::min(left_over_horiz, left_over_vert);

                if area_fit < best_area_fit
                    || (area_fit == best_area_fit && short_side_fit < best_short_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = width;
                    new_rect.height = height;
                    best_short_side_fit = short_side_fit;
                    best_area_fit = area_fit;
                }
            }

            if self.allow_flip && free_rect.width >= height && free_rect.height >= width {
                let left_over_horiz = free_rect.width - height;
                let left_over_vert = free_rect.height - width;
                let short_side_fit = std::cmp::min(left_over_horiz, left_over_vert);

                if area_fit < best_area_fit
                    || (area_fit == best_area_fit && short_side_fit < best_short_side_fit)
                {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = height;
                    new_rect.height = width;
                    best_short_side_fit = short_side_fit;
                    best_area_fit = area_fit;
                }
            }
        }

        if new_rect.height == 0 || new_rect.width == 0 {
            return None;
        }

        Some(new_rect)
    }

    /// Computes the rect for contact point variant.
    fn get_rect_for_contact_point(&self, width: i32, height: i32) -> Option<Rect2D> {
        let mut new_rect = Rect2D::default();
        let mut best_contact_score = -1;

        for i in 0..self.free_rects.len() {
            let free_rect = self.free_rects.get(i)?;
            // try to place the rect in upright (non-flipped) orientation:
            if free_rect.width >= width && free_rect.height >= height {
                let contact_score =
                    self.get_score_for_contact_point(free_rect.x, free_rect.y, width, height);

                if contact_score > best_contact_score {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = width;
                    new_rect.height = height;
                    best_contact_score = contact_score;
                }
            }

            if self.allow_flip && free_rect.width >= height && free_rect.height >= width {
                let contact_score =
                    self.get_score_for_contact_point(free_rect.x, free_rect.y, height, width);

                if contact_score > best_contact_score {
                    new_rect.x = free_rect.x;
                    new_rect.y = free_rect.y;
                    new_rect.width = height;
                    new_rect.height = width;
                    best_contact_score = contact_score;
                }
            }
        }

        if new_rect.height == 0 || new_rect.width == 0 {
            return None;
        }

        Some(new_rect)
    }

    /// returns true if the free rect was split
    fn is_split_free_node(&mut self, free_rect: &Rect2D, used_rect: &Rect2D) -> bool {
        // test with SAT if the rects even intersect:
        if used_rect.x >= free_rect.x + free_rect.width
            || used_rect.x + used_rect.width <= free_rect.x
            || used_rect.y >= free_rect.y + free_rect.height
            || used_rect.y + used_rect.height <= free_rect.y
        {
            return false;
        }

        if used_rect.x < free_rect.x + free_rect.width
            && used_rect.x + used_rect.width > free_rect.x
        {
            // new node at the top side of the used node:
            if used_rect.y > free_rect.y && used_rect.y < free_rect.y + free_rect.height {
                let mut new_rect = free_rect.clone();
                new_rect.height = used_rect.y - new_rect.y;
                self.free_rects.push(new_rect);
            }

            // new node at the bottom side of the used node:
            if used_rect.y + used_rect.height < free_rect.y + free_rect.height {
                let mut new_rect = free_rect.clone();
                new_rect.y = used_rect.y + used_rect.height;
                new_rect.height = free_rect.y + free_rect.height - (used_rect.y + used_rect.height);
                self.free_rects.push(new_rect);
            }
        }

        if used_rect.y < free_rect.y + free_rect.height
            && used_rect.y + used_rect.height > free_rect.y
        {
            // new node at the left side of the used node:
            if used_rect.x > free_rect.x && used_rect.x < free_rect.x + free_rect.width {
                let mut new_rect = free_rect.clone();
                new_rect.width = used_rect.x - new_rect.x;
                self.free_rects.push(new_rect);
            }

            // new node at the right side of the used node:
            if used_rect.x + used_rect.width < free_rect.x + free_rect.width {
                let mut new_rect = free_rect.clone();
                new_rect.x = used_rect.x + used_rect.width;
                new_rect.width = free_rect.x + free_rect.width - (used_rect.x + used_rect.width);
                self.free_rects.push(new_rect);
            }
        }

        true
    }

    /// goes through the free rect list and removes any redundant entries
    fn prune_free_list(&mut self) {
        // go through each pair and remove any rects that are redundant:
        let mut keep: Vec<bool> = vec![true; self.free_rects.len()];
        for i in 0..self.free_rects.len() {
            for j in i + 1..self.free_rects.len() {
                if let Some(free_rect_i) = self.free_rects.get(i)
                    && let Some(free_rect_j) = self.free_rects.get(j)
                {
                    if self.is_contained_on(free_rect_i, free_rect_j) {
                        if let Some(value) = keep.get_mut(i) {
                            *value = false;
                        }
                        break;
                    }

                    if self.is_contained_on(free_rect_j, free_rect_i)
                        && let Some(value) = keep.get_mut(j)
                    {
                        *value = false;
                    }
                }
            }
        }

        // remove all items marked false:
        let mut iter = keep.iter();
        self.free_rects.retain(|_| *iter.next().unwrap());
    }

    /// determine whether rect A is contained on rect B
    fn is_contained_on(&self, a: &Rect2D, b: &Rect2D) -> bool {
        a.x >= b.x
            && a.y >= b.y
            && a.x + a.width <= b.x + b.width
            && a.y + a.height <= b.y + b.height
    }

    /// returns 0 if the two intervals i1 and i2 are disjoint, or the length of their
    /// overlap otherwise.
    fn get_common_interval_len(
        &self,
        i1_start: i32,
        i1_end: i32,
        i2_start: i32,
        i2_end: i32,
    ) -> i32 {
        if i1_end < i2_start || i2_end < i1_start {
            return 0;
        }

        std::cmp::min(i1_end, i2_end) - std::cmp::max(i1_start, i2_start)
    }
} // impl RectsBinPack

// unit tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect2d_basics() {
        let rect = Rect2D::default();

        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 0);
        assert_eq!(rect.height, 0);

        let rect = Rect2D::with_details(2, 4, 16, 32).unwrap();

        assert_eq!(rect.x, 2);
        assert_eq!(rect.y, 4);
        assert_eq!(rect.width, 16);
        assert_eq!(rect.height, 32);

        assert_eq!(
            Rect2D::with_details(-1, 4, 16, 32).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            Rect2D::with_details(0, -1, 16, 32).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            Rect2D::with_details(0, 0, 0, 32).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            Rect2D::with_details(2, 4, 16, 0).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            Rect2D::with_details(-1, -1, 0, 0).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
    }

    #[test]
    fn rbp_invalid_arg() {
        assert_eq!(
            RectsBinPack::new(0, 0, false).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            RectsBinPack::new(32, 0, false).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            RectsBinPack::new(0, 32, false).unwrap_err(),
            RectsBinPackError::InvalidArg
        );

        assert_eq!(
            RectsBinPack::new(0, 0, true).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            RectsBinPack::new(32, 0, true).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
        assert_eq!(
            RectsBinPack::new(0, 32, true).unwrap_err(),
            RectsBinPackError::InvalidArg
        );
    }

    #[test]
    fn rbp_basics() {
        let mut rbp = RectsBinPack::new(32, 32, false).unwrap();
        assert_eq!(rbp.get_occupancy(), 0.0);

        assert_eq!(
            rbp.insert(16, 16, FreeRectHeuristic::ShortSideFit)
                .is_some(),
            true
        );
        assert_eq!(
            rbp.insert(16, 16, FreeRectHeuristic::LongSideFit).is_some(),
            true
        );
        assert_eq!(
            rbp.insert(16, 16, FreeRectHeuristic::AreaFit).is_some(),
            true
        );
        assert_eq!(
            rbp.insert(16, 16, FreeRectHeuristic::BottomLeft).is_some(),
            true
        );
        assert_eq!(rbp.get_occupancy(), 1.0);

        assert_eq!(
            rbp.insert(1, 1, FreeRectHeuristic::ContactPoint).is_none(),
            true
        );
    }
}
