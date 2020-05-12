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
        Element::radius(180.0),
        Element::radius(180.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_leading_roll() {
    let sequence_str = "1o".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element::radius(180.0),
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
        Element::radius(180.0),
        Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_all_rolls() {
    let sequence_str = "1o1".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0.0),
        Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element { angle: 360.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_inverted_flight() {
    let sequence_str = "-o-".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::invline(0.0),
        Element::radius(-180.0),
        Element::radius(-180.0),
        Element::invline(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_inverted_entry_and_inverting_leading_roll() {
    let sequence_str = "-2o".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::invline(0.0),
        Element { angle: 180.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element::radius(180.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }

#[test]
fn loop_with_inverted_entry_and_inverting_combining_roll() {
    let sequence_str = "-o2".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::invline(0.0),
        Element::radius(-180.0),
        Element { angle: 180.0, argument: 1.0, .. Element::new(ElementType::Roll) },
        Element::radius(180.0),
        Element::line(0.0),
        ];
    
    compare_elements(&sequence.figures[0].elements, &expected_elements);
    }