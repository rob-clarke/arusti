use crate::olan::parser::Rule;
use crate::types::{Element, ElementType, RollType};
use pest::iterators::Pair;

pub fn get_elements_for_single_line(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "0" => vec![
            Element::line(0),
            ],
        "d" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        "id" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "v" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "iv" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "z" => vec![
            Element::radius(135),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-135),
        ],
        "iz" => vec![
            Element::radius(-135),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(135),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_twin_line(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "t" => vec![
            Element::radius(45),
            Element::line(45),
            Element::radius(-135),
            Element::line(-90),
            Element::radius(90),
        ],
        "it" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::radius(135),
            Element::line(90),
            Element::radius(-90),
        ],
        "k" => vec![
            Element::radius(90),
            Element::line(90),
            Element::radius(-135),
            Element::line(-45),
            Element::radius(45),
        ],
        "ik" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::radius(135),
            Element::line(45),
            Element::radius(-45),
        ],
        "zt" => vec![
            Element::radius(135),
            Element::line(45),
            Element::radius(135),
            Element::line(-90),
            Element::radius(90),
        ],
        "izt" => vec![
            Element::radius(-135),
            Element::line(-45),
            Element::radius(-135),
            Element::line(90),
            Element::radius(-90),
        ],
        "kz" => vec![
            Element::radius(90),
            Element::line(90),
            Element::radius(-135),
            Element::line(-45),
            Element::radius(-135),
        ],
        "ikz" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::radius(135),
            Element::line(45),
            Element::radius(135),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_loop_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "a" => vec![
            Element::combining(-1, Element::line(0)),
            Element::radius(-180),
            Element::combining(0, Element::invline(0)),
            ],
        "m" => vec![
            Element::combining(-1, Element::line(0)),
            Element::radius(180),
            Element::combining(0, Element::invline(0)),
            ],
        "o" => vec![
            Element::radius(170),
            Element::combining(0, Element::invradius(20)),
            Element::invradius(170),
        ],
        "io" => vec![
            Element::radius(-170),
            Element::combining(0, Element::invradius(-20)),
            Element::invradius(-170),
        ],
        "qo" => vec![
            Element::radius(90),
            Element::line(90),
            Element::radius(90),
            Element::line(0),
            Element::radius(90),
            Element::line(-90),
            Element::radius(90),
            Element::line(0),
        ],
        "iqo" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::radius(-90),
            Element::line(0),
            Element::radius(-90),
            Element::line(90),
            Element::radius(-90),
            Element::line(0),
        ],
        "dq" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(90),
            Element::invline(45),
            Element::radius(90),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(90),
            Element::line(-45),
            Element::radius(45),
        ],
        "idq" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(-90),
            Element::invline(-45),
            Element::radius(-90),
            Element::invline(45),
            Element::combining(1, Element::invline(45)),
            Element::invline(45),
            Element::radius(-90),
            Element::line(45),
            Element::radius(-45),
        ],
        "qq" => vec![
            Element::radius(45),
            Element::line(45),
            Element::radius(45),
            Element::line(90),
            Element::radius(45),
            Element::invline(45),
            Element::radius(45),
            Element::invline(0),
            Element::radius(45),
            Element::invline(-45),
            Element::radius(45),
            Element::line(-90),
            Element::radius(45),
            Element::line(-45),
            Element::radius(45),
        ],
        "iqq" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::radius(-45),
            Element::line(-90),
            Element::radius(-45),
            Element::invline(-45),
            Element::radius(-45),
            Element::invline(0),
            Element::radius(-45),
            Element::invline(45),
            Element::radius(-45),
            Element::line(90),
            Element::radius(-45),
            Element::line(45),
            Element::radius(-45),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_loop_line_combo(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "c" => vec![
            Element::radius(225),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-45),
        ],
        "ic" => vec![
            Element::radius(-225),
            Element::invline(45),
            Element::combining(0, Element::invline(45)),
            Element::invline(45),
            Element::radius(45),
        ],
        "rc" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-225),
        ],
        "irc" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(225),
        ],
        "g" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(-270),
            Element::invline(45),
            Element::combining(0, Element::invline(45)),
            Element::invline(45),
            Element::radius(45),
        ],
        "ig" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::radius(270),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-45),
        ],
        "p" => vec![
            Element::radius(270),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "ip" => vec![
            Element::radius(-270),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "rp" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(270),
        ],
        "irp" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::combining(-1, Element::line(-90)),
            Element::line(-90),
            Element::radius(-270),
        ],
        "q" => vec![
            Element::combining(-1, Element::line(0)),
            Element::radius(315),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "iq" => vec![
            Element::radius(-315),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        "rq" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(315),
        ],
        "irq" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::radius(-315),
        ],
        "y" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(225),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "iy" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::radius(-225),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "ry" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(225),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(45),
            Element::radius(45),
        ],
        "iry" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::combining(-1, Element::line(-90)),
            Element::line(-90),
            Element::radius(-225),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_double_loop(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "cc" => vec![
            Element::line(0),
            Element::combining(-1, Element::line(0)),
            Element::radius(225),
            Element::invline(-45),
            Element::combining(1, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-270),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "rcc" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(-270),
            Element::invline(45),
            Element::combining(1, Element::invline(45)),
            Element::invline(45),
            Element::radius(225),
            Element::combining(0, Element::line(0)),
            Element::line(0),
        ],
        "oo" => vec![
            Element::radius(180),
            Element::radius(-360),
            Element::radius(180),
        ],
        "icc" => vec![
            Element::line(0),
            Element::combining(-1, Element::line(0)),
            Element::radius(-225),
            Element::invline(45),
            Element::combining(1, Element::invline(45)),
            Element::invline(45),
            Element::radius(270),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        "ircc" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::line(270),
            Element::invline(-45),
            Element::combining(1, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-225),
            Element::combining(0, Element::line(0)),
            Element::line(0),
        ],
        "ioo" => vec![
            Element::radius(-180),
            Element::radius(360),
            Element::radius(-180),
        ],
        "ooo" => vec![Element::radius(360), Element::radius(-360)],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_humpty(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "b" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(180),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "pb" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(-180),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "bb" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(180),
            Element::line(-90),
            Element::combining(1, Element::line(-90)),
            Element::line(-90),
            Element::radius(180),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "pbb" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(-180),
            Element::line(-90),
            Element::combining(1, Element::line(-90)),
            Element::line(-90),
            Element::radius(180),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "db" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(90),
            Element::radius(180),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-45),
        ],
        "rdb" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(90),
            Element::radius(-180),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-45),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_hammerhead(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "h" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::stall(180, 0),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "dh" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(45),
            Element::line(90),
            Element::combining(1, Element::line(90)),
            Element::line(90),
            Element::stall(180, 0),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "hd" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::stall(180, 0),
            Element::line(-90),
            Element::combining(1, Element::line(-90)),
            Element::line(-90),
            Element::radius(45),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "dhd" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(45),
            Element::line(90),
            Element::combining(1, Element::line(90)),
            Element::line(90),
            Element::stall(180, 0),
            Element::line(-90),
            Element::combining(2, Element::line(-90)),
            Element::line(-90),
            Element::radius(45),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "ta" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::stall(0, -180),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "ita" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::stall(0, 180),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_three_roll(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "n" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(135),
            Element::invline(-45),
            Element::combining(1, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-135),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "in" => vec![
            Element::radius(-90),
            Element::line(-90),
            Element::combining(-1, Element::line(-90)),
            Element::line(-90),
            Element::radius(135),
            Element::line(45),
            Element::combining(1, Element::line(45)),
            Element::line(45),
            Element::radius(-135),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "pn" => vec![
            Element::radius(90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(-135),
            Element::line(-45),
            Element::combining(1, Element::line(-45)),
            Element::line(-45),
            Element::radius(135),
            Element::line(90),
            Element::combining(0, Element::line(90)),
            Element::line(90),
            Element::radius(-90),
        ],
        "ipn" => vec![
            Element::radius(-90),
            Element::line(90),
            Element::combining(-1, Element::line(90)),
            Element::line(90),
            Element::radius(-135),
            Element::invline(45),
            Element::combining(1, Element::invline(45)),
            Element::invline(45),
            Element::radius(135),
            Element::line(-90),
            Element::combining(0, Element::line(-90)),
            Element::line(-90),
            Element::radius(90),
        ],
        "w" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(-135),
            Element::line(-90),
            Element::combining(1, Element::line(-90)),
            Element::line(-90),
            Element::radius(135),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        "iw" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::radius(135),
            Element::line(90),
            Element::combining(1, Element::line(90)),
            Element::line(90),
            Element::radius(-135),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "gg" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(-270),
            Element::invline(45),
            Element::combining(1, Element::invline(45)),
            Element::invline(45),
            Element::radius(270),
            Element::line(45),
            Element::combining(0, Element::line(45)),
            Element::line(45),
            Element::radius(-45),
        ],
        "igg" => vec![
            Element::radius(-45),
            Element::line(-45),
            Element::combining(-1, Element::line(-45)),
            Element::line(-45),
            Element::radius(270),
            Element::invline(-45),
            Element::combining(1, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(-270),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_extra(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "mm" => vec![Element::radius(180), Element::radius(-180)],
        "zb" => vec![
            Element::radius(135),
            Element::invline(45),
            Element::combining(-1, Element::invline(45)),
            Element::invline(45),
            Element::radius(180),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "rzb" => vec![
            Element::radius(135),
            Element::invline(45),
            Element::combining(-1, Element::invline(45)),
            Element::invline(45),
            Element::radius(-180),
            Element::line(-45),
            Element::combining(0, Element::line(-45)),
            Element::line(-45),
            Element::radius(45),
        ],
        "bz" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(180),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(135),
        ],
        "rbz" => vec![
            Element::radius(45),
            Element::line(45),
            Element::combining(-1, Element::line(45)),
            Element::line(45),
            Element::radius(-180),
            Element::invline(-45),
            Element::combining(0, Element::invline(-45)),
            Element::invline(-45),
            Element::radius(135),
        ],
        "zy" => vec![
            Element::radius(135),
            Element::invline(45),
            Element::combining(-1, Element::invline(45)),
            Element::invline(45),
            Element::radius(-225),
            Element::invline(-90),
            Element::combining(0, Element::invline(-90)),
            Element::invline(-90),
            Element::radius(90),
        ],
        _ => {
            unreachable!();
        }
    }
}

pub fn get_elements_for_rolling_turn(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    // Default turn angle is 90, check for multipliers
    let mut turn_angle = 90;
    if current_pair.as_rule() == Rule::turn_angle {
        turn_angle = 90 * current_pair.as_str().parse::<i16>().unwrap();
        current_pair = inner_pairs.next().unwrap();
    }
    #[derive(PartialEq)]
    enum TurnType {
        J,
        JO,
        JOI,
        JIO,
    }

    let turn_type = match current_pair.as_str() {
        "j" => TurnType::J,
        "jo" => TurnType::JO,
        "joi" => TurnType::JOI,
        "jio" => TurnType::JIO,
        _ => {
            unreachable!();
        }
    };
    let mut turn_rolls = 0.0;
    if let Some(current_pair) = inner_pairs.next() {
        match current_pair.as_str() {
            "1" | "2" | "3" | "4" => {
                turn_rolls = current_pair.as_str().parse::<f32>().unwrap();
            }
            "15" => {
                turn_rolls = 1.5;
            }
            _ => {
                unreachable!();
            }
        }
    }
    if turn_rolls == 0.0 {
        // We don't care about type as we're not rolling
        return vec![Element {
            main_angle: turn_angle,
            ..Element::new(ElementType::Turn)
        }];
    }

    if turn_type == TurnType::J || turn_type == TurnType::JO {
        return match turn_type {
            TurnType::J => vec![Element {
                main_angle: turn_angle,
                aux_angle: (360.0 * turn_rolls) as i16,
                ..Element::new(ElementType::Turn)
            }],
            TurnType::JO => vec![Element {
                main_angle: turn_angle,
                aux_angle: (-360.0 * turn_rolls) as i16,
                ..Element::new(ElementType::Turn)
            }],
            _ => {
                unreachable!();
            }
        };
    }
    let initial_direction = match turn_type {
        TurnType::JIO => 1,
        TurnType::JOI => -1,
        _ => {
            unreachable!();
        }
    };
    let turn_angle_per_roll = ((turn_angle as f32) / turn_rolls) as i16;
    if turn_rolls == 1.5 {
        // Finicky special case...
        return vec![
            Element {
                main_angle: turn_angle_per_roll,
                aux_angle: initial_direction * 360,
                ..Element::new(ElementType::Turn)
            },
            Element {
                main_angle: turn_angle_per_roll / 2,
                aux_angle: initial_direction * -180,
                ..Element::new(ElementType::Turn)
            },
        ];
    }
    // Integer number of rolls
    let mut direction = initial_direction;
    let mut elements = Vec::<Element>::new();
    for _ in 1..(turn_rolls as i8) {
        elements.push(Element {
            main_angle: turn_angle_per_roll,
            aux_angle: direction * 360,
            ..Element::new(ElementType::Turn)
        });
        direction = -direction;
    }

    return elements;
}
pub fn get_elements_for_non_aresti(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "oj" => vec![
            Element::radius(90),
            Element {
                aux_angle: 90,
                roll_type: RollType::Standard,
                ..Element::line(90)
            },
            Element::radius(80),
            Element::combining(0, Element::invradius(20)),
            Element::invradius(170),
        ],
        "ioj" => vec![
            Element::radius(170),
            Element::combining(0, Element::invradius(20)),
            Element::invradius(80),
            Element {
                aux_angle: 90,
                roll_type: RollType::Standard,
                ..Element::line(-90)
            },
            Element::radius(90),
        ],
        "mj" => vec![
            Element::radius(90),
            Element {
                aux_angle: 90,
                roll_type: RollType::Standard,
                ..Element::line(90)
            },
            Element::radius(90),
        ],
        "aj" => vec![
            Element::radius(-90),
            Element {
                aux_angle: 90,
                roll_type: RollType::Standard,
                ..Element::line(-90)
            },
            Element::radius(-90),
        ],
        _ => unreachable!(),
    }
}
