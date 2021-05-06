use arusti;

use arusti::{ElementType,Element,Sequence};

fn compare_elements_in_figure(sequence: &Sequence, figure_idx: usize, expectation: &Vec<Element>) {
    let result = &sequence.figures[figure_idx].elements;
    assert_eq!(result.len(), expectation.len(), "Figure {} has wrong number of elements", figure_idx);
    for (index,(result,expected)) in result.iter().zip( expectation.iter() ).enumerate() {
        assert_eq!(result, expected, "Element {} of Figure {} does not match expectation", index, figure_idx);
        }
    }

#[test]
fn full_sequence() {
    let sequence_str = "/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);
    
    compare_elements_in_figure(
        &sequence, 0,
        &vec![
            Element::line(0.0),
            Element::radius(45.0),
            Element::line(45.0),
            Element::radius(90.0),
            Element::invline(45.0),
            Element::radius(90.0),
            Element::invline(-45.0),
            Element::radius(90.0),
            Element::line(-45.0),
            Element::radius(45.0),
            Element::line(0.0),
            ]
        );
    
    compare_elements_in_figure(
        &sequence, 1,
        &vec![
            Element::line(0.0),
            Element::radius(90.0),
            Element::line(90.0),
            Element::radius(-90.0),
            Element::line(0.0),
            ]
        );
    
    compare_elements_in_figure(
        &sequence, 2,
        &vec![
            Element::line(0.0),
            Element::radius(-90.0),
            Element::line(-90.0),
            Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Spin) },
            Element::line(-90.0),
            Element::radius(270.0),
            Element::invline(0.0),
            ]
        );
    
    compare_elements_in_figure(
        &sequence, 3,
        &vec![
            Element::invline(0.0),
            Element { angle: 180.0, inverted: true, .. Element::new(ElementType::Turn) },
            Element::invline(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 4,
        &vec![
            Element::invline(0.0),
            // Expect reversed roll due to separator
            Element { angle: -180.0, argument: 4.0, .. Element::new(ElementType::Roll) },
            Element::line(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 5,
        &vec![
            Element::line(0.0),
            Element::radius(225.0),
            Element::invline(-45.0),
            // Expect reversed roll due to separator
            Element { angle: -180.0, argument: 4.0, .. Element::new(ElementType::Roll) },
            Element::line(-45.0),
            Element::radius(45.0),
            Element::line(0.0),
            ]
        );
    
    compare_elements_in_figure(
        &sequence, 6,
        &vec![
            Element::line(0.0),
            Element::radius(90.0),
            Element::line(90.0),
            Element::radius(90.0),
            Element::invline(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 7,
        &vec![
            Element::invline(0.0),
            Element::radius(45.0),
            Element::invline(-45.0),
            Element { angle: 180.0, argument: 1.0, .. Element::new(ElementType::Roll) },
            Element::line(-45.0),
            Element::radius(45.0),
            Element::line(0.0),
            ]
        );
    
    compare_elements_in_figure(
        &sequence, 8,
        &vec![
            Element::line(0.0),
            Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Roll) },
            Element::radius(180.0),
            Element { angle: 180.0, argument: 1.0, .. Element::new(ElementType::Roll) },
            Element::line(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 9,
        &vec![
            Element::line(0.0),
            Element { angle: 180.0, argument: 720.0, .. Element::new(ElementType::Turn) },
            Element::line(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 10,
        &vec![
            Element::line(0.0),
            Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Flick) },
            Element { angle: -180.0, argument: 1.0, .. Element::new(ElementType::Roll) },
            Element::invline(0.0),
            ]
        );

    compare_elements_in_figure(
        &sequence, 11,
        &vec![
            Element::invline(0.0),
            Element { angle: 360.0, argument: 2.0, .. Element::new(ElementType::Roll) },
            Element::radius(180.0),
            Element { angle: 360.0, argument: 4.0, .. Element::new(ElementType::Roll) },
            Element::line(0.0),
            ]
        );

    }