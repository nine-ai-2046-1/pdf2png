//! grouper.rs — Groups consecutive PDF pages by identical pixel dimensions.
//! Used in smart-group (-s) mode to determine which pages are stacked together.
/// Represents a contiguous run of pages sharing the same pixel dimensions.
#[derive(Debug, PartialEq)]
pub struct PageGroup {
    /// 0-based index of the first page in this group.
    pub start_index: usize,
    /// Number of pages in this group.
    pub page_count: usize,
    /// Pixel dimensions (width, height) shared by all pages in this group.
    pub dimensions: (u32, u32),
}

/// Group consecutive pages with identical dimensions into PageGroup slices.
/// A new group starts whenever a page's dimensions differ from the previous page.
/// Non-consecutive same-size pages are NOT merged.
pub fn group_pages(dimensions: &[(u32, u32)]) -> Vec<PageGroup> {
    if dimensions.is_empty() {
        return Vec::new();
    }

    let mut groups: Vec<PageGroup> = Vec::new();
    let mut current_start = 0;
    let mut current_dim = dimensions[0];
    let mut current_count = 1;

    // Walk through each page dimension, flushing a group when dimensions change
    for (i, &dim) in dimensions.iter().enumerate().skip(1) {
        if dim == current_dim {
            // Same dimension — extend the current group
            current_count += 1;
        } else {
            // Dimension changed — flush the current group and start a new one
            groups.push(PageGroup {
                start_index: current_start,
                page_count: current_count,
                dimensions: current_dim,
            });
            current_start = i;
            current_dim = dim;
            current_count = 1;
        }
    }

    // Flush the final group
    groups.push(PageGroup {
        start_index: current_start,
        page_count: current_count,
        dimensions: current_dim,
    });

    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_same_dimension_produces_one_group() {
        let dims = vec![(800, 1131), (800, 1131), (800, 1131)];
        let groups = group_pages(&dims);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].start_index, 0);
        assert_eq!(groups[0].page_count, 3);
        assert_eq!(groups[0].dimensions, (800, 1131));
    }

    #[test]
    fn test_all_different_dimensions_produces_one_group_per_page() {
        let dims = vec![(800, 1131), (1131, 800), (595, 842)];
        let groups = group_pages(&dims);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].page_count, 1);
        assert_eq!(groups[1].page_count, 1);
        assert_eq!(groups[2].page_count, 1);
    }

    #[test]
    fn test_mixed_dimensions_groups_correctly() {
        // portrait, portrait, landscape, landscape, portrait
        let dims = vec![
            (800, 1131),
            (800, 1131),
            (1131, 800),
            (1131, 800),
            (800, 1131),
        ];
        let groups = group_pages(&dims);
        assert_eq!(groups.len(), 3);
        assert_eq!(
            groups[0],
            PageGroup {
                start_index: 0,
                page_count: 2,
                dimensions: (800, 1131)
            }
        );
        assert_eq!(
            groups[1],
            PageGroup {
                start_index: 2,
                page_count: 2,
                dimensions: (1131, 800)
            }
        );
        assert_eq!(
            groups[2],
            PageGroup {
                start_index: 4,
                page_count: 1,
                dimensions: (800, 1131)
            }
        );
    }

    #[test]
    fn test_non_consecutive_same_size_not_merged() {
        // portrait, landscape, portrait — should produce 3 groups, not 2
        let dims = vec![(800, 1131), (1131, 800), (800, 1131)];
        let groups = group_pages(&dims);
        assert_eq!(groups.len(), 3);
    }

    #[test]
    fn test_empty_input_returns_empty() {
        let groups = group_pages(&[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_single_page_produces_one_group() {
        let dims = vec![(800, 1131)];
        let groups = group_pages(&dims);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].page_count, 1);
    }
}
