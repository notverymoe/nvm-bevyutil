use super::{Overlap, OverlapCase, Projection};

    struct TestCase(
        &'static str,
        Projection,      // Test
        Option<Overlap>, // Separation 
        Option<Overlap>, // Penetration
        bool,            // sign_opposite
    );

    const WIDTH: i32 = 4;
    const  FROM: i32 = -2*WIDTH;
    const    TO: i32 =  2*WIDTH;

    #[test]
    fn test_separation() {
        for i in FROM..TO { test_separation_case(i); }
    }

    #[test]
    fn test_penetration() {
        for i in FROM..TO { test_penetration_case(i); }
    }

    fn test_separation_case(offset: i32) {
        let (target, cases) = create_cases(offset);
        let mut fails = String::new();
        for case in cases {
            let separation = target.get_separation(case.1);
            test_overlap(&mut fails,  case.0, separation, case.2);
            test_symmetry(&mut fails, case.0, separation, case.1.get_separation(target));
        }
        assert!(fails.is_empty(), "Failed separation tests {}: {}\n", offset, fails);
    }

    fn test_penetration_case(offset: i32) {
        let (target, cases) = create_cases(offset);
        let mut fails = String::new();
        for case in cases {
            let penetration = target.get_penetration(case.1);
            test_overlap(&mut fails,  case.0, penetration, case.3);
            test_symmetry(&mut fails, case.0, penetration, case.1.get_penetration(target));
        }
        assert!(fails.is_empty(), "Failed penetration tests {}: {}\n", offset, fails);
    }

    fn create_cases(offset: i32) -> (Projection, [TestCase; 15]) {
        let (bound_l, bound_r)  = (offset, offset + WIDTH);
        let create     = |l: i32, r: i32| Projection::new_unchecked(l as f32, r as f32);
        let gap_from_l = |dist: i32| create(bound_l - dist,        bound_l);
        let pen_from_l = |dist: i32| create(       bound_l, bound_l + dist);
        let gap_from_r = |dist: i32| create(       bound_r, bound_r + dist);
        let pen_from_r = |dist: i32| create(bound_r - dist,        bound_r);
        let touch      = |  at: i32| create(            at,             at);

        (create(bound_l, bound_r), [
            TestCase("gap_l",       create(bound_l - 2, bound_l - 1), Some(Overlap(gap_from_l(1), OverlapCase::Negative)), None,                                                true),       
            TestCase("gap_r",       create(bound_r + 1, bound_r + 2), Some(Overlap(gap_from_r(1), OverlapCase::Positive )), None,                                                true),
            TestCase("touch_l",     create(bound_l - 2, bound_l    ), None,                                                Some(Overlap(     touch(bound_l  ), OverlapCase::Positive )), true),     
            TestCase("touch_r",     create(bound_r,     bound_r + 2), None,                                                Some(Overlap(     touch(bound_r  ), OverlapCase::Negative)), true),
            TestCase("penetrate_l", create(bound_l - 2, bound_l + 1), None,                                                Some(Overlap(pen_from_l(1        ), OverlapCase::Positive )), true), 
            TestCase("penetrate_r", create(bound_r - 1, bound_r + 2), None,                                                Some(Overlap(pen_from_r(1        ), OverlapCase::Negative)), true),
            TestCase("contained_l", create(bound_l + 1, bound_r - 2), None,                                                Some(Overlap(pen_from_l(WIDTH - 2), OverlapCase::Positive )), true), 
            TestCase("contained_r", create(bound_l + 2, bound_r - 1), None,                                                Some(Overlap(pen_from_r(WIDTH - 2), OverlapCase::Negative)), true), 
            TestCase("contained_m", create(bound_l + 1, bound_r - 1), None,                                                Some(Overlap(pen_from_l(WIDTH - 1), OverlapCase::Positive )), true),
            TestCase("contains_l",  create(bound_l - 2, bound_r + 1), None,                                                Some(Overlap(pen_from_l(WIDTH + 1), OverlapCase::Positive )), true),  
            TestCase("contains_r",  create(bound_l - 1, bound_r + 2), None,                                                Some(Overlap(pen_from_r(WIDTH + 1), OverlapCase::Negative)), true),  
            TestCase("contains_m",  create(bound_l - 1, bound_r + 1), None,                                                Some(Overlap(pen_from_r(WIDTH + 1), OverlapCase::Negative)), true),
            TestCase("overlap_l",   create(bound_l - 1, bound_r    ), None,                                                Some(Overlap(pen_from_l(WIDTH    ), OverlapCase::Positive )), true),   
            TestCase("overlap_r",   create(bound_l,     bound_r + 1), None,                                                Some(Overlap(pen_from_r(WIDTH    ), OverlapCase::Negative)), true),   
            TestCase("overlap_m",   create(bound_l,     bound_r    ), None,                                                Some(Overlap(pen_from_l(WIDTH    ), OverlapCase::Unsigned )), true),
        ])
    }

    fn test_overlap(fails: &mut String, name: &str, orig: Option<Overlap>, target: Option<Overlap>) {
        if orig != target {
            *fails = format!("{}\n- {}: Got ({:?}), expected ({:?})", fails, name, orig, target);
        }
    }

    fn test_symmetry(fails: &mut String, name: &str, a: Option<Overlap>, b: Option<Overlap>) {
        if let Some(a) = a {
            if let Some(b) = b {
                if a != b.negate() {
                    *fails = format!("{}\n- {}: ({:?}) does not have symmetry with reversed self & other ({:?})", fails, name, a, b);
                }
            }
        }
    }