use crate::types::{Element,ElementType};
use crate::olan::parser::Rule;
use pest::iterators::Pair;

pub fn get_elements_for_single_line(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "d"  => vec![ Element::radius(  45.0), Element::line( 45.0), Element::combining(0), Element::line( 45.0), Element::radius( -45.0) ],
        "id" => vec![ Element::radius( -45.0), Element::line(-45.0), Element::combining(0), Element::line(-45.0), Element::radius(  45.0) ],
        "v"  => vec![ Element::radius(  90.0), Element::line( 90.0), Element::combining(0), Element::line( 90.0), Element::radius( -90.0) ],
        "iv" => vec![ Element::radius( -90.0), Element::line(-90.0), Element::combining(0), Element::line(-90.0), Element::radius(  90.0) ],
        "z"  => vec![ Element::radius( 135.0), Element::line( 45.0), Element::combining(0), Element::line( 45.0), Element::radius(-135.0) ],
        "iz" => vec![ Element::radius(-135.0), Element::line(-45.0), Element::combining(0), Element::line(-45.0), Element::radius( 135.0) ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_twin_line(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "t"   => vec![ Element::radius(  45.0), Element::line( 45.0), Element::radius(-135.0), Element::line(-90.0), Element::radius(  90.0) ],
        "it"  => vec![ Element::radius( -45.0), Element::line(-45.0), Element::radius( 135.0), Element::line( 90.0), Element::radius( -90.0) ],
        "k"   => vec![ Element::radius(  90.0), Element::line( 90.0), Element::radius(-135.0), Element::line(-45.0), Element::radius(  45.0) ],
        "ik"  => vec![ Element::radius( -90.0), Element::line(-90.0), Element::radius( 135.0), Element::line( 45.0), Element::radius( -45.0) ],
        "zt"  => vec![ Element::radius( 135.0), Element::line( 45.0), Element::radius( 135.0), Element::line(-90.0), Element::radius(  90.0) ],
        "izt" => vec![ Element::radius(-135.0), Element::line(-45.0), Element::radius(-135.0), Element::line( 90.0), Element::radius( -90.0) ],
        "kz"  => vec![ Element::radius(  90.0), Element::line( 90.0), Element::radius(-135.0), Element::line(-45.0), Element::radius(-135.0) ],
        "ikz" => vec![ Element::radius( -90.0), Element::line(-90.0), Element::radius( 135.0), Element::line( 45.0), Element::radius( 135.0) ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_loop_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "a"   => vec![ Element::radius(-180.0) ],
        "m"   => vec![ Element::radius( 180.0) ],
        "o"   => vec![ Element::radius( 180.0), Element::combining(0), Element::radius( 180.0) ],
        "io"  => vec![ Element::radius(-180.0), Element::combining(0), Element::radius(-180.0) ],
        "qo"  => vec![
            Element::radius(90.0), Element::line(90.0),
            Element::radius(90.0), Element::line(0.0),
            Element::radius(90.0), Element::line(-90.0),
            Element::radius(90.0), Element::line(0.0)
            ],
        "iqo" => vec![
            Element::radius(-90.0), Element::line(-90.0),
            Element::radius(-90.0), Element::line(0.0),
            Element::radius(-90.0), Element::line(90.0),
            Element::radius(-90.0), Element::line(0.0)
            ],
        "dq"  => vec![
            Element::radius(45.0), Element::line( 45.0),
            Element::radius(90.0), Element::line( 45.0),
            Element::radius(90.0), Element::line(-45.0),
            Element::combining(1), Element::line(-45.0),
            Element::radius(90.0), Element::line(-45.0),
            Element::radius(45.0)
            ],
        "idq" => vec![
            Element::radius(-45.0), Element::line(-45.0),
            Element::radius(-90.0), Element::line(-45.0),
            Element::radius(-90.0), Element::line( 45.0),
            Element::combining(1), Element::line( 45.0),
            Element::radius(-90.0), Element::line( 45.0),
            Element::radius(-45.0)
            ],
        "qq"  => vec![
            Element::radius(45.0), Element::line( 45.0),
            Element::radius(45.0), Element::line( 90.0),
            Element::radius(45.0), Element::line( 45.0),
            Element::radius(45.0), Element::line(  0.0),
            Element::radius(45.0), Element::line(-45.0),
            Element::radius(45.0), Element::line(-90.0),
            Element::radius(45.0), Element::line(-45.0),
            Element::radius(45.0) 
            ],
        "iqq" => vec![
            Element::radius(-45.0), Element::line(-45.0),
            Element::radius(-45.0), Element::line(-90.0),
            Element::radius(-45.0), Element::line(-45.0),
            Element::radius(-45.0), Element::line(  0.0),
            Element::radius(-45.0), Element::line( 45.0),
            Element::radius(-45.0), Element::line( 90.0),
            Element::radius(-45.0), Element::line( 45.0),
            Element::radius(-45.0) 
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_loop_line_combo(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "c"   => vec![
            Element::radius(225.0),
            Element::invline(-45.0), Element::combining(0), Element::invline(-45.0),
            Element::radius(-45.0)
            ],
        "ic"  => vec![
            Element::radius(-225.0),
            Element::invline(45.0), Element::combining(0), Element::invline(45.0),
            Element::radius(45.0)
            ],
        "rc"  => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-225.0)
            ],
        "irc" => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(225.0)
            ],
        "g"   => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(-270.0),
            Element::invline(45.0), Element::combining(0), Element::invline(45.0),
            Element::radius(45.0)
            ],
        "ig"  => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::radius(270.0),
            Element::invline(-45.0), Element::combining(0), Element::invline(-45.0),
            Element::radius(-45.0)
            ],
        "p"   => vec![
            Element::radius(270.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0)
            ],
        "ip"  => vec![
            Element::radius(-270.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0)
            ],
        "rp"  => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(270.0)
            ],
        "irp" => vec![
            Element::radius(-90.0),
            Element::line(-90.0), Element::combining(-1), Element::line(-90.0),
            Element::radius(-270.0)
            ],
        "q"   => vec![
            Element::radius(315.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0)
            ],
        "iq"  => vec![
            Element::radius(-315.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-45.0)
            ],
        "rq"  => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(315.0)
            ],
        "irq" => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::radius(-315.0)
            ],
        "y"   => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(225.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0)
            ],
        "iy"  => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::radius(-225.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0)
            ],
        "ry"  => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(225.0),
            Element::line(-45.0), Element::combining(0), Element::line(45.0),
            Element::radius(45.0)
            ],
        "iry" => vec![
            Element::radius(-90.0),
            Element::line(-90.0), Element::combining(-1), Element::line(-90.0),
            Element::radius(-225.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-45.0)
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_double_loop(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "cc" => vec![
            Element::line(0.0), Element::combining(-1),
            Element::radius(225.0),
            Element::invline(-45.0), Element::combining(1), Element::invline(-45.0),
            Element::radius(-270.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0)
            ],
        "rcc" => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0), 
            Element::radius(-270.0),
            Element::invline(45.0), Element::combining(1), Element::invline(45.0),
            Element::radius(225.0),
            Element::combining(0),
            Element::line(0.0)
            ],
        "oo" => vec![
            Element::radius(180.0),
            Element::radius(-360.0),
            Element::radius(180.0)
            ],
        "icc" => vec![
            Element::line(0.0), Element::combining(-1),
            Element::radius(-225.0),
            Element::invline(45.0), Element::combining(1), Element::invline(45.0),
            Element::radius(270.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-45.0)
            ],
        "ircc" => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::line(270.0),
            Element::invline(-45.0), Element::combining(1), Element::invline(-45.0),
            Element::line(-225.0),
            Element::combining(0),
            Element::line(0.0)
            ],
        "ioo" => vec![
            Element::radius(-180.0),
            Element::radius(360.0),
            Element::radius(-180.0)
            ],
        "ooo" => vec![
            Element::radius(360.0),
            Element::radius(-360.0),
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_humpty(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "b" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(180.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "pb" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(-180.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "bb" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(180.0),
            Element::line(-90.0), Element::combining(1), Element::line(-90.0),
            Element::radius(180.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0),
            ],
        "pbb" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(-180.0),
            Element::line(-90.0), Element::combining(1), Element::line(-90.0),
            Element::radius(180.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0),
            ],
        "db" => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(90.0),
            Element::radius(180.0),
            Element::invline(-45.0), Element::combining(0), Element::invline(-45.0),
            Element::radius(-45.0),
            ],
        "rdb" => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(90.0),
            Element::radius(-180.0),
            Element::invline(-45.0), Element::combining(0), Element::invline(-45.0),
            Element::radius(-45.0),
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_hammerhead(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "h" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::stall(180.0,0.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "dh"   => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(45.0),
            Element::line(90.0), Element::combining(1), Element::line(90.0),
            Element::stall(180.0,0.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "hd"  => vec![ 
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::stall(180.0,0.0),
            Element::line(-90.0), Element::combining(1), Element::line(-90.0),
            Element::radius(45.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0),
            ],
        "dhd"  => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(45.0),
            Element::line(90.0), Element::combining(1), Element::line(90.0),
            Element::stall(180.0,0.0),
            Element::line(-90.0), Element::combining(2), Element::line(-90.0),
            Element::radius(45.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0),
            ],
        "ta" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::stall(0.0,-180.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "ita" => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::stall(0.0,180.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_three_roll(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "n"   => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(135.0),
            Element::invline(-45.0), Element::combining(1), Element::invline(-45.0),
            Element::radius(-135.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0),
            ],
        "in"  => vec![
            Element::radius(-90.0),
            Element::line(-90.0), Element::combining(-1), Element::line(-90.0),
            Element::radius(135.0),
            Element::line(45.0), Element::combining(1), Element::line(45.0),
            Element::radius(-135.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "pn"  => vec![
            Element::radius(90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(-135.0),
            Element::line(-45.0), Element::combining(1), Element::line(-45.0),
            Element::radius(135.0),
            Element::line(90.0), Element::combining(0), Element::line(90.0),
            Element::radius(-90.0),
            ],
        "ipn" => vec![
            Element::radius(-90.0),
            Element::line(90.0), Element::combining(-1), Element::line(90.0),
            Element::radius(-135.0),
            Element::invline(45.0), Element::combining(1), Element::invline(45.0),
            Element::radius(135.0),
            Element::line(-90.0), Element::combining(0), Element::line(-90.0),
            Element::radius(90.0),
            ],
        "w"   => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(-135.0),
            Element::line(-90.0), Element::combining(1), Element::line(-90.0),
            Element::radius(135.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-45.0),
            ],
        "iw"  => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::radius(135.0),
            Element::line(90.0), Element::combining(1), Element::line(90.0),
            Element::radius(-135.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0),
            ],
        "gg"  => vec![
            Element::radius(45.0),
            Element::line(45.0), Element::combining(-1), Element::line(45.0),
            Element::radius(-270.0),
            Element::invline(45.0), Element::combining(1), Element::invline(45.0),
            Element::radius(270.0),
            Element::line(45.0), Element::combining(0), Element::line(45.0),
            Element::radius(-45.0),
            ],
        "igg" => vec![
            Element::radius(-45.0),
            Element::line(-45.0), Element::combining(-1), Element::line(-45.0),
            Element::radius(270.0),
            Element::invline(-45.0), Element::combining(1), Element::invline(-45.0),
            Element::radius(-270.0),
            Element::line(-45.0), Element::combining(0), Element::line(-45.0),
            Element::radius(45.0),
            ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_extra(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_rolling_turn(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    // Default turn angle is 90, check for multipliers
    let mut turn_angle = 90.0;
    if current_pair.as_rule() == Rule::turn_angle {
        turn_angle = 90.0 * current_pair.as_str().parse::<f32>().unwrap();
        current_pair = inner_pairs.next().unwrap();
        }
    
    #[derive(PartialEq)]
    enum TurnType {
        J, JO, JOI, JIO
        }

    let turn_type = match current_pair.as_str() {
        "j"   => TurnType::J,
        "jo"  => TurnType::JO,
        "joi" => TurnType::JOI,
        "jio" => TurnType::JIO,
        _ => { unreachable!(); }
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
            _ => { unreachable!(); }
            }
        }
    
    if turn_rolls == 0.0 {
        // We don't care about type as we're not rolling
        return vec![
            Element{
                angle: turn_angle,
                .. Element::new(ElementType::Turn)
                }
            ]
        }

    if turn_type == TurnType::J || turn_type == TurnType::JO {
        return match turn_type {
            TurnType::J  => vec![
                Element{
                    angle: turn_angle,
                    argument: 360.0 * turn_rolls,
                    .. Element::new(ElementType::Turn)
                    }
                ],
            TurnType::JO => vec![
                Element{
                    angle: turn_angle,
                    argument: -360.0 * turn_rolls,
                    .. Element::new(ElementType::Turn)
                    }
                ],
            _ => { unreachable!(); }
            }
        }
    
    let initial_direction = match turn_type {
        TurnType::JIO =>  1.0,
        TurnType::JOI => -1.0,
        _ => { unreachable!(); }
        };
    let turn_angle_per_roll = turn_angle / turn_rolls;
    if turn_rolls == 1.5 {
        // Finicky special case...
        return vec![
            Element{
                angle: turn_angle_per_roll,
                argument: initial_direction * 360.0,
                .. Element::new(ElementType::Turn)
                },
            Element{
                angle: turn_angle_per_roll * 0.5,
                argument: initial_direction * -180.0,
                .. Element::new(ElementType::Turn)
                }
            ]
        }
    
    // Integer number of rolls
    let mut direction = initial_direction;
    let mut elements = Vec::<Element>::new();
    for _ in 1..(turn_rolls as i8) {
        elements.push(
            Element {
                angle: turn_angle_per_roll,
                argument: direction * 360.0,
                .. Element::new(ElementType::Turn)
                }
            );
        direction = -direction; 
        }
    
    return elements;
    }

pub fn get_elements_for_non_aresti(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }