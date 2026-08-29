#[cfg(test)]
mod tests {
    use tpt_l_pager_thermo::*;

    #[test]
    fn heatmap_records_and_classifies() {
        let mut h = PageHeatmap::new(4);
        for _ in 0..10 {
            h.record_access(1).unwrap();
        }
        h.record_access(3).unwrap();
        assert_eq!(h.accesses(1), Some(10));
        assert_eq!(h.hot_pages(5), vec![1]);
        assert_eq!(h.cold_pages(5), vec![0, 2, 3]);
    }

    #[test]
    fn out_of_range_access_errors() {
        let mut h = PageHeatmap::new(2);
        assert!(h.record_access(5).is_err());
    }

    #[test]
    fn policy_classification() {
        let p = TieringPolicy::new(8, 2);
        assert_eq!(p.classify(10), Tier::Hot);
        assert_eq!(p.classify(1), Tier::Cold);
        assert_eq!(p.classify(5), Tier::Neutral);
    }

    #[test]
    fn sparse_heatmap() {
        let mut s = SparseHeatmap::default();
        s.record_access(42);
        s.record_access(42);
        assert_eq!(s.accesses(42), 2);
        assert_eq!(s.accesses(7), 0);
    }
}
