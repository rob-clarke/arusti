use crate::types::{Element,ElementType};
use crate::olan::parser::Rule;
use pest::iterators::Pair;

pub fn get_elements_for_single_line(figure_pair: Pair<Rule>) -> Vec<Element> {
    match figure_pair.as_str() {
        "d"  => vec![ Element::radius(  45.0), Element::line( 45.0), Element::radius( -45.0) ],
        "id" => vec![ Element::radius( -45.0), Element::line(-45.0), Element::radius(  45.0) ],
        "v"  => vec![ Element::radius(  90.0), Element::line( 90.0), Element::radius( -90.0) ],
        "iv" => vec![ Element::radius( -90.0), Element::line(-90.0), Element::radius(  90.0) ],
        "z"  => vec![ Element::radius( 135.0), Element::line( 45.0), Element::radius(-135.0) ],
        "iz" => vec![ Element::radius(-135.0), Element::line(-45.0), Element::radius( 135.0) ],
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
            Element::combining(1), Element::line(-135.0),
            Element::radius(90.0), Element::line(-45.0),
            Element::radius(45.0)
            ],
        "idq" => vec![
            Element::radius(-45.0), Element::line(-45.0),
            Element::radius(-90.0), Element::line(-45.0),
            Element::radius(-90.0), Element::line( 45.0),
            Element::combining(1), Element::line( 135.0),
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
        "y"   => vec![ Element::radius(0.0) ],
        "iy"  => vec![ Element::radius(0.0) ],
        "ry"  => vec![ Element::radius(0.0) ],
        "iry" => vec![ Element::radius(0.0) ],
        _ => { unreachable!(); }
        }
    }

pub fn get_elements_for_double_loop(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_humpty(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_hammerhead(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_three_roll(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_extra(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }

pub fn get_elements_for_rolling_turn(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    let mut turn_angle = 90.0;

    if current_pair.as_rule() == Rule::turn_angle {
        turn_angle = 90.0 * current_pair.as_str().parse::<f32>().unwrap();
        current_pair = inner_pairs.next().unwrap();
        }
    
    let argument = match current_pair.as_str() {
        "j"   => 1,
        "jo"  => 2,
        "joi" => 3,
        "jio" => 4,
        _ => { unreachable!(); }
        };
    
    vec![
        Element {
            elem_type: ElementType::TurnStart,
            inverted: false,
            angle: turn_angle,
            argument: argument as f32
            },
        Element::combining(0),
        Element::new(ElementType::TurnEnd)
        ]
    }

pub fn get_elements_for_non_aresti(figure_pair: Pair<Rule>) -> Vec<Element> {
    Vec::<Element>::new()
    }