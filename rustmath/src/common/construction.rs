use super::Glyph;

pub struct ConstructionPart<G: Glyph> {
    pub glyph: G,
    pub size: f32,
    pub max_start_overlap: f32,
    pub max_end_overlap: f32,
    pub is_extender: bool,
}

pub struct Construction<G: Glyph> {
    pub min_overlap: f32,
    pub parts: Vec<ConstructionPart<G>>,
}

pub struct PartIterator<'a, G: Glyph> {
    iter: core::slice::Iter<'a, ConstructionPart<G>>,
    extenders: u32,
    current: Option<&'a ConstructionPart<G>>,
    replicate_current: u32,
}

impl<'a, G: Glyph> Iterator for PartIterator<'a, G> {
    type Item = &'a ConstructionPart<G>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.replicate_current > 0 {
            self.replicate_current -= 1;
            return self.current;
        }

        loop {
            self.current = self.iter.next();
            if let Some(current) = self.current {
                if current.is_extender {
                    if self.extenders == 0 {
                        continue;
                    }

                    self.replicate_current = self.extenders;
                }
            }

            return self.current;
        }
    }
}

struct OverlappingPartIterator<'a, G: Glyph> {
    iter: PartIterator<'a, G>,
    min_overlap: f32,
    max_overlap: f32,
}

impl<'a, G: Glyph> Iterator for OverlappingPartIterator<'a, G> {
    /// (min_overlap, max_overlap)
    type Item = (f32, f32, &'a ConstructionPart<G>);

    fn next(&mut self) -> Option<Self::Item> {
        let part = self.iter.next()?;
        let max_overlap = self.max_overlap;

        self.max_overlap = part.max_end_overlap;

        // Note that `min_overlap > max_overlap` is possible, for example for
        // the first part: max_overlap is 0.0 in the beginning
        let min_overlap = self.min_overlap.min(max_overlap);
        let max_overlap = part.max_start_overlap.min(max_overlap);

        Some((min_overlap, max_overlap, part))
    }
}

pub struct SizedPartIterator<'a, G: Glyph> {
    iter: OverlappingPartIterator<'a, G>,
    len: u32,
    overlap_factor: f32,
}

impl<'a, G: Glyph> Iterator for SizedPartIterator<'a, G> {
    /// Overlap + part. Note that the overlap is positive but means negative shift
    type Item = (f32, &'a ConstructionPart<G>);

    fn next(&mut self) -> Option<Self::Item> {
        let (min_overlap, max_overlap, part) = self.iter.next()?;
        debug_assert!(min_overlap <= max_overlap);
        self.len -= 1;

        let overlap = (1.0 - self.overlap_factor) * min_overlap + self.overlap_factor * max_overlap;

        Some((overlap, part))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len as usize;
        (len, Some(len))
    }
}

impl<'a, G: Glyph> ExactSizeIterator for PartIterator<'a, G> {}

impl<G: Glyph> Construction<G> {
    pub fn iter_parts(&self, extenders: u32) -> PartIterator<'_, G> {
        PartIterator {
            iter: self.parts.iter(),
            extenders,
            current: None,
            replicate_current: 0,
        }
    }

    fn overlapping(&self, extenders: u32) -> OverlappingPartIterator<'_, G> {
        let iter = self.iter_parts(extenders);
        OverlappingPartIterator {
            iter,
            min_overlap: self.min_overlap,
            max_overlap: 0.0,
        }
    }

    pub fn construct(&self, min_size: f32) -> (f32, SizedPartIterator<'_, G>) {
        for extenders in 0u32..100000u32 {
            // We should not need more extenders than this
            let (min, max, len) = self.size_bounds(extenders);
            assert!(min <= max);
            if max >= min_size {
                let goal = min_size.max(min);
                let overlap_factor = if min == max {
                    0.0
                } else {
                    (max - goal) / (max - min)
                };

                let iter = SizedPartIterator {
                    iter: self.overlapping(extenders),
                    len,
                    overlap_factor,
                };

                return (goal, iter);
            }
        }
        unreachable!();
    }

    pub fn size_bounds(&self, extenders: u32) -> (f32, f32, u32) {
        let mut min_size = 0.0;
        let mut max_size = 0.0;
        let mut len = 0u32;

        for (min_overlap, max_overlap, part) in self.overlapping(extenders) {
            debug_assert!(min_overlap <= part.size);
            debug_assert!(max_overlap <= part.size);
            min_size += part.size - max_overlap;
            max_size += part.size - min_overlap;
            len += 1;
        }

        (min_size, max_size, len)
    }
}
