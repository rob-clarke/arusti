use test_env_log::test;

use arusti;

use arusti::{Attitude, Element, ElementType, RollType, Sequence};

fn compare_elements_in_figure(sequence: &Sequence, figure_idx: usize, expectation: &Vec<Element>) {
    let result = &sequence.figures[figure_idx].elements;
    assert_eq!(
        result.len(),
        expectation.len(),
        "Figure {} has wrong number of elements",
        figure_idx
    );
    for (index, (result, expected)) in result.iter().zip(expectation.iter()).enumerate() {
        assert_eq!(
            result, expected,
            "Element {} of Figure {} does not match expectation",
            index, figure_idx
        );
    }
}

#[test]
fn full_sequence() {
    let sequence_str = "/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);
    compare_elements_in_figure(
        &sequence,
        0,
        &vec![
            Element::line(0),
            Element::radius(45),
            Element::line(45),
            Element::radius(90),
            Element::invline(45),
            Element::radius(90),
            Element::invline(-45),
            Element::radius(90),
            Element::line(-45),
            Element::radius(45),
            Element::line(0),
        ],
    );
    compare_elements_in_figure(
        &sequence,
        1,
        &vec![
            Element::line(0),
            Element::radius(90),
            Element::line(90),
            Element::radius(-90),
            Element::line(0),
        ],
    );
    compare_elements_in_figure(
        &sequence,
        2,
        &vec![
            Element::line(0),
            Element::radius(-90),
            Element::line(-90),
            Element {
                aux_angle: 360,
                roll_type: RollType::Spin,
                ..Element::line(-90)
            },
            Element::line(-90),
            Element::radius(270),
            Element::invline(0),
        ],
    );
    compare_elements_in_figure(
        &sequence,
        3,
        &vec![
            Element::invline(0),
            Element {
                main_angle: 180,
                attitude: Attitude::Inverted,
                ..Element::new(ElementType::Turn)
            },
            Element::invline(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        4,
        &vec![
            Element::invline(0),
            // Expect reversed roll due to separator
            Element {
                aux_angle: -180,
                roll_type: RollType::HesitationQuarters,
                ..Element::invline(0)
            },
            Element::line(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        5,
        &vec![
            Element::line(0),
            Element::radius(225),
            Element::invline(-45),
            // Expect reversed roll due to separator
            Element {
                aux_angle: -180,
                roll_type: RollType::HesitationQuarters,
                ..Element::invline(-45)
            },
            Element::line(-45),
            Element::radius(45),
            Element::line(0),
        ],
    );
    compare_elements_in_figure(
        &sequence,
        6,
        &vec![
            Element::line(0),
            Element::radius(90),
            Element::line(90),
            Element::radius(90),
            Element::invline(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        7,
        &vec![
            Element::invline(0),
            Element::radius(45),
            Element::invline(-45),
            Element {
                aux_angle: 180,
                roll_type: RollType::Standard,
                ..Element::invline(-45)
            },
            Element::line(-45),
            Element::radius(45),
            Element::line(0),
        ],
    );
    compare_elements_in_figure(
        &sequence,
        8,
        &vec![
            Element::line(0),
            Element {
                aux_angle: 360,
                roll_type: RollType::Standard,
                ..Element::line(0)
            },
            Element::radius(180),
            Element {
                aux_angle: 180,
                roll_type: RollType::Standard,
                ..Element::invline(0)
            },
            Element::line(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        9,
        &vec![
            Element::line(0),
            Element {
                main_angle: 180,
                aux_angle: 720,
                ..Element::new(ElementType::Turn)
            },
            Element::line(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        10,
        &vec![
            Element::line(0),
            Element {
                aux_angle: 360,
                roll_type: RollType::Flick,
                ..Element::line(0)
            },
            Element {
                aux_angle: -180,
                roll_type: RollType::Standard,
                ..Element::line(0)
            },
            Element::invline(0),
        ],
    );

    compare_elements_in_figure(
        &sequence,
        11,
        &vec![
            Element::invline(0),
            Element {
                aux_angle: 360,
                roll_type: RollType::HesitationHalves,
                ..Element::invline(0)
            },
            Element::radius(180),
            Element {
                aux_angle: 360,
                roll_type: RollType::HesitationQuarters,
                ..Element::line(0)
            },
            Element::line(0),
        ],
    );
}
