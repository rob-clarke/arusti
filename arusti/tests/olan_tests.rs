use test_env_log::test;

use arusti;

use arusti::{Attitude, Element, ElementType, RollType};

fn compare_elements(result: &Vec<Element>, expectation: &Vec<Element>) {
    assert_eq!(
        result.len(),
        expectation.len(),
        "Figure has wrong number of elements"
    );
    for (index, (result, expected)) in result.iter().zip(expectation.iter()).enumerate() {
        assert_eq!(
            result, expected,
            "Element {} does not match expectation",
            index
        );
    }
}

#[test]
fn loop_plain() {
    let sequence_str = "o".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![Element::line(0), Element::radius(360), Element::line(0)];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn q_loop_with_leading_roll() {
    let sequence_str = "1q".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0),
        Element {
            aux_angle: 360,
            roll_type: RollType::Standard,
            ..Element::line(0)
        },
        Element::radius(315),
        Element::line(-45),
        Element::radius(45),
        Element::line(0),
    ];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn loop_with_combining_roll() {
    let sequence_str = "o1".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0),
        Element::radius(170),
        Element {
            aux_angle: 360,
            roll_type: RollType::Standard,
            ..Element::invradius(20)
        },
        Element::invradius(170),
        Element::line(0),
    ];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn humpty_with_all_rolls() {
    let sequence_str = "1b1".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0),
        Element::radius(90),
        Element::line(90),
        Element {
            aux_angle: 360,
            roll_type: RollType::Standard,
            ..Element::line(90)
        },
        Element::line(90),
        Element::radius(180),
        Element::line(-90),
        Element {
            aux_angle: 360,
            roll_type: RollType::Standard,
            ..Element::line(-90)
        },
        Element::line(-90),
        Element::radius(90),
        Element::line(0),
    ];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn diagonal_with_inverted_flight() {
    let sequence_str = "-d-".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::invline(0),
        Element::radius(-45),
        Element::invline(45),
        Element::radius(45),
        Element::invline(0),
    ];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn hammerhead_turn() {
    let sequence_str = "h".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0),
        Element::radius(90),
        Element::line(90),
        Element {
            main_angle: 180,
            aux_angle: 0,
            ..Element::new(ElementType::Stall)
        },
        Element::line(-90),
        Element::radius(90),
        Element::line(0),
    ];
    compare_elements(&sequence.figures[0].elements, &expected_elements);
}

#[test]
fn knifeedge_pass() {
    let sequence_str = "0!".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);

    let expected_elements = vec![
        Element::line(0),
        Element {
            aux_angle: 90,
            roll_type: RollType::Standard,
            ..Element::line(0)
        },
        Element::keline(0),
        Element {
            aux_angle: -90,
            roll_type: RollType::Standard,
            attitude: Attitude::KnifeEdge,
            ..Element::line(0)
        },
        Element::line(0),
    ];

    compare_elements(&sequence.figures[0].elements, &expected_elements);
}
