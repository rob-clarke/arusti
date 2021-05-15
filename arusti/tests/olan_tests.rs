use arusti;

use arusti::{ElementType,Element};

fn compare_elements(result: &Vec<Element>, expectation: &Vec<Element>) {
    assert_eq!(result.len(), expectation.len(), "Figure has wrong number of elements");
    for (index,(result,expected)) in result.iter().zip( expectation.iter() ).enumerate() {
        assert_eq!(result, expected, "Element {} does not match expectation", index);
        }
    }

#[test]
fn loop_plain() {
    let sequence_str = "o".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element::radius(360.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn q_loop_with_leading_roll() {
    let sequence_str = "1q".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element { aux_angle: 360.0, .. Element::line(0.0) },
        Element::radius(315.0),
        Element::line(-45.0),
        Element::radius(45.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_combining_roll() {
    let sequence_str = "o1".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element::radius(170.0),
        Element { aux_angle: 360.0, .. Element::invradius(20.0) },
        Element::radius(170.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn humpty_with_all_rolls() {
    let sequence_str = "1b1".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element::radius(90.0),
        Element::line(90.0),
        Element { aux_angle: 360.0, .. Element::line(90.0) },
        Element::line(90.0),
        Element::radius(180.0),
        Element::line(-90.0),
        Element { aux_angle: 360.0, .. Element::line(-90.0) },
        Element::line(-90.0),
        Element::radius(90.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn diagonal_with_inverted_flight() {
    let sequence_str = "-d-".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::invline(0.0),
        Element::radius(-45.0),
        Element::invline(45.0),
        Element::radius(45.0),
        Element::invline(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn hammerhead_turn() {
    let sequence_str = "h".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),    
        Element::radius(90.0),
        Element::line(90.0),
        Element { angle: 180.0, argument: 0.0, .. Element::new(ElementType::Stall) },
        Element::line(-90.0),
        Element::radius(90.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn knifeedge_pass() {
    let sequence_str = "0!".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element { angle: 90.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::keline(0.0),
        Element { angle: -90.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::line(0.0),
        ]
    }