use log;

use itertools::Itertools;

use pest::Parser;

#[derive(Parser)]
#[grammar = "arusti/olan/olan.pest"]
struct OlanParser;

use crate::types::{Attitude, Element, ElementType, Figure, RollType, Sequence};

use pest::iterators::Pair;

fn get_element_for_roll_element(roll_element_pair: Pair<Rule>, reverse_roll: bool) -> Element {
    let roll_angle: i16;
    let mut roll_type = RollType::Standard;

    let mut inner_pairs = roll_element_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    let mut has_type = true;

    // This unwrap will not fail due to parser restrictions
    match current_pair.as_rule() {
        Rule::roll_angle => {
            roll_angle = current_pair.as_str().parse().unwrap();
            match inner_pairs.next() {
                Some(roll_type_pair) => {
                    current_pair = roll_type_pair.into_inner().next().unwrap();
                }
                None => {
                    has_type = false;
                }
            }
        }
        Rule::flick_spin_type => {
            roll_angle = 1;
        }
        _ => {
            unreachable!();
        }
    }
    if has_type {
        match current_pair.as_rule() {
            Rule::hesitation_type => {
                roll_type = match current_pair.as_str().parse().unwrap() {
                    2 => RollType::HesitationHalves,
                    4 => RollType::HesitationQuarters,
                    8 => RollType::HesitationEighths,
                    _ => {
                        unreachable!();
                    }
                };
            }
            Rule::flick_spin_type => {
                roll_type = match current_pair.as_str() {
                    "f" => RollType::Flick,
                    "if" => RollType::InvertedFlick,
                    "s" => RollType::Spin,
                    "is" => RollType::InvertedSpin,
                    _ => {
                        unreachable!();
                    }
                };
            }
            _ => {
                unreachable!();
            }
        }
    }

    let computed_roll = match roll_type {
        RollType::HesitationHalves => match roll_angle {
            2 => 360,
            3 => 540,
            4 => 720,
            _ => {
                unreachable!();
            }
        },
        RollType::HesitationQuarters => match roll_angle {
            2 => 180,
            3 => 270,
            4 => 360,
            5 => 450,
            6 => 540,
            7 => 630,
            8 => 720,
            _ => {
                unreachable!();
            }
        },
        RollType::HesitationEighths => match roll_angle {
            2 => 90,
            4 => 180,
            6 => 270,
            8 => 360,
            _ => {
                unreachable!();
            }
        },
        _ => match roll_angle {
            1 => 360,
            2 => 180,
            3 => 270,
            4 => 90,
            5 => 450,
            6 => 540,
            7 => 630,
            9 => 720,
            _ => {
                unreachable!();
            }
        },
    };

    Element {
        aux_angle: if reverse_roll {
            -computed_roll
        } else {
            computed_roll
        },
        roll_type,
        ..Element::new(ElementType::Line)
    }
}

fn is_direction_swap(roll_separator: Pair<Rule>) -> bool {
    roll_separator.as_str().to_string().contains(",")
}

fn get_elements_for_roll_set(roll_set_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = roll_set_pair.into_inner();

    let mut reverse_state = false;

    let mut elements = Vec::<Element>::new();

    loop {
        if let Some(current_pair) = inner_pairs.next() {
            match current_pair.as_rule() {
                Rule::roll_element => {
                    elements.push(get_element_for_roll_element(current_pair, reverse_state));
                }
                Rule::roll_separator => {
                    if is_direction_swap(current_pair) {
                        reverse_state = !reverse_state;
                    }
                }
                _ => {
                    unreachable!();
                }
            }
        } else {
            break;
        }
    }

    return elements;
}

use crate::olan::figure_defs;

fn get_elements_for_main_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    let current_pair = figure_pair.into_inner().next().unwrap();

    match current_pair.as_rule() {
        Rule::single_line => figure_defs::get_elements_for_single_line(current_pair),
        Rule::twin_line => figure_defs::get_elements_for_twin_line(current_pair),
        Rule::loop_figure => figure_defs::get_elements_for_loop_figure(current_pair),
        Rule::loop_line_combo => figure_defs::get_elements_for_loop_line_combo(current_pair),
        Rule::double_loop => figure_defs::get_elements_for_double_loop(current_pair),
        Rule::humpty => figure_defs::get_elements_for_humpty(current_pair),
        Rule::hammerhead => figure_defs::get_elements_for_hammerhead(current_pair),
        Rule::three_roll => figure_defs::get_elements_for_three_roll(current_pair),
        Rule::extra => figure_defs::get_elements_for_extra(current_pair),
        Rule::non_aresti => figure_defs::get_elements_for_non_aresti(current_pair),
        _ => {
            unreachable!();
        }
    }
}

fn find_combining_with_arg(elements: &Vec<Element>, target_roll_selector: i32) -> Option<usize> {
    let result = elements
        .iter()
        .enumerate()
        .find(|(_, elem)| match elem.elem_type {
            ElementType::Combining { roll_selector, .. } => roll_selector == target_roll_selector,
            _ => false,
        });
    match result {
        Some((idx, _)) => Some(idx),
        None => None,
    }
}

fn insert_combining_rolls(
    elements: &mut Vec<Element>,
    position: i32,
    roll_set_opt: Option<Vec<Element>>,
) -> Option<Vec<Element>> {
    // Xfif(Y)(Z)W
    // X => -1
    // Y => 1
    // Z => 2
    // W => 0
    if let Some(idx) = find_combining_with_arg(&elements, position) {
        // Convert the combining marker into its base element
        let base_element = Element {
            elem_type: match elements[idx].elem_type {
                ElementType::Combining { base_type, .. } => base_type.into(),
                _ => unreachable!(),
            },
            ..elements[idx].clone()
        };
        if let Some(roll_set) = roll_set_opt {
            // Need to dulicate the base_element for the number of roll elements
            //  and apply their aux_angle & roll_type
            let applied_rolls = roll_set.iter().map(|roll_elem| Element {
                aux_angle: roll_elem.aux_angle,
                roll_type: roll_elem.roll_type,
                ..base_element.clone()
            });
            // Insert the roll set & append
            elements.splice(idx..idx + 1, applied_rolls);
            return None;
        } else {
            // Just put the base_element in place of the marker
            elements[idx] = base_element;
            return roll_set_opt;
        }
    } else {
        return roll_set_opt;
    }
}

fn get_elements_for_named_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    let mut elements = Vec::<Element>::new();

    let mut entry_roll_set_opt: Option<Vec<Element>> = None;
    let mut inner_roll_set_opt: Option<Vec<Element>> = None;
    let mut inner_roll_set2_opt: Option<Vec<Element>> = None;
    let mut exit_roll_set_opt: Option<Vec<Element>> = None;

    if current_pair.as_rule() == Rule::roll_set {
        entry_roll_set_opt = Some(get_elements_for_roll_set(current_pair));
        current_pair = inner_pairs.next().unwrap();
    }
    let mut main_figure_elements = get_elements_for_main_figure(current_pair);

    // Try to unwrap next pair
    while let Some(current_pair) = inner_pairs.next() {
        match current_pair.as_rule() {
            Rule::inner_roll_set => {
                if inner_roll_set_opt.is_none() {
                    inner_roll_set_opt = Some(get_elements_for_roll_set(
                        current_pair.into_inner().next().unwrap(),
                    ));
                } else if inner_roll_set2_opt.is_none() {
                    inner_roll_set2_opt = Some(get_elements_for_roll_set(
                        current_pair.into_inner().next().unwrap(),
                    ));
                }
            }
            Rule::roll_set => {
                exit_roll_set_opt = Some(get_elements_for_roll_set(current_pair));
            }
            _ => {
                unreachable!();
            }
        }
    }

    // Deal with roll set before figure
    let remaining_entry_rolls_opt =
        insert_combining_rolls(&mut main_figure_elements, -1, entry_roll_set_opt);
    if let Some(mut entry_rolls) = remaining_entry_rolls_opt {
        elements.append(&mut entry_rolls);
    }
    // Deal with inner roll sets
    insert_combining_rolls(&mut main_figure_elements, 1, inner_roll_set_opt);
    insert_combining_rolls(&mut main_figure_elements, 2, inner_roll_set2_opt);

    // Deal with roll set after figure
    let remaining_exit_rolls_opt =
        insert_combining_rolls(&mut main_figure_elements, 0, exit_roll_set_opt);
    elements.append(&mut main_figure_elements);

    if let Some(mut exit_rolls) = remaining_exit_rolls_opt {
        elements.append(&mut exit_rolls);
    }

    return elements;
}

fn get_elements_for_rolling_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    if let Some(roll_set_pair) = inner_pairs.next() {
        get_elements_for_roll_set(roll_set_pair)
    } else {
        // Rolling figure is actually just a line
        Vec::<Element>::new()
    }
}

fn get_transition_attitude(line_extension: &Pair<Rule>) -> Attitude {
    if line_extension.as_str().to_string().contains("-") {
        Attitude::Inverted
    } else {
        Attitude::Normal
    }
}

fn invert_figure_elements(elements: &mut Vec<Element>, from_idx: usize) {
    // Push -> pull
    // Line -> inverted line (for non-verticals)
    for (i, mut elem) in elements.iter_mut().enumerate() {
        if i < from_idx {
            continue;
        }
        match elem.elem_type {
            ElementType::Radius => {
                if elem.matching >= 0 {
                    elem.main_angle = -elem.main_angle;
                }
                //elem.attitude = Attitude::get_inverted(elem.attitude);
            }
            ElementType::Turn => {
                elem.attitude = Attitude::get_inverted(elem.attitude);
            }
            ElementType::Line => {
                if !((elem.main_angle == 90) || (elem.main_angle == -90)) {
                    elem.attitude = Attitude::get_inverted(elem.attitude);
                }
            }
            _ => {}
        }
    }
}

fn parse_figure(olan_figure: Pair<Rule>) -> Figure {
    let mut figure = Figure::new();

    let mut inner_pairs = olan_figure.into_inner();

    let mut current_pair = inner_pairs.next().unwrap();

    // If has initial inter_line_extension, check for inverted entry
    let mut entry_attitude = Attitude::Normal;
    if current_pair.as_rule() == Rule::inter_line_extension {
        entry_attitude = get_transition_attitude(&current_pair);
        // Advance current_pair
        current_pair = inner_pairs.next().unwrap();
    }

    // Add entry line
    if entry_attitude == Attitude::Inverted {
        figure.push(Element::invline(0));
    } else {
        figure.push(Element::line(0));
    }
    let mut figure_elements = match current_pair.as_rule() {
        Rule::named_figure => get_elements_for_named_figure(current_pair),
        Rule::rolling_figure => get_elements_for_rolling_figure(current_pair),
        Rule::rolling_turn => figure_defs::get_elements_for_rolling_turn(current_pair),
        _ => unreachable!(),
    };

    // If has trailing inter_line_extension, check for inverted exit
    let mut exit_attitude = Attitude::Normal;
    if let Some(current_pair) = inner_pairs.next() {
        // Will be inter_line_extension by parser rules
        exit_attitude = get_transition_attitude(&current_pair);
    }

    log::debug!("PIV: {:#?}", figure_elements);
    // Apply inversions due to in-figure rolls
    let mut current_attitude = Attitude::Normal;
    let mut inverted_by_roll = false;
    let mut current_pitch = 0;
    for mut element in figure_elements.iter_mut() {
        match element.elem_type {
            ElementType::Radius => {
                if inverted_by_roll == true && element.matching >= 0 {
                    element.main_angle = -element.main_angle
                }
                current_pitch = (current_pitch + element.main_angle + 360) % 360;
                if 90 < current_pitch && current_pitch < 270 {
                    current_attitude = Attitude::Inverted;
                } else {
                    current_attitude = Attitude::Normal;
                }
            }
            ElementType::Line => {
                element.attitude = current_attitude;
                match current_pitch {
                    90 | 270 => {}
                    0 | 180 => {
                        if (element.aux_angle % 360) != 0 {
                            inverted_by_roll = !inverted_by_roll;
                            current_attitude = Attitude::get_inverted(current_attitude);
                            current_pitch = (current_pitch + 180 + 360) % 360;
                        }
                    }
                    45 | 225 => {
                        if (element.aux_angle % 360) != 0 {
                            inverted_by_roll = !inverted_by_roll;
                            current_attitude = Attitude::get_inverted(current_attitude);
                            current_pitch = (current_pitch + 90 + 360) % 360;
                        }
                    }
                    135 | 315 => {
                        if (element.aux_angle % 360) != 0 {
                            inverted_by_roll = !inverted_by_roll;
                            current_attitude = Attitude::get_inverted(current_attitude);
                            current_pitch = (current_pitch - 90 + 360) % 360;
                        }
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
            ElementType::Stall => {
                // Indicate pitch reversal through stalling figure
                current_pitch = (current_pitch + 180 + 360) % 360;
            }
            _ => {}
        }
        log::debug!("Pitch: {}", current_pitch);
    }

    log::debug!("PFR: {:#?}", figure_elements);
    log::debug!("Current_inversion {:?}", current_attitude);
    if entry_attitude == Attitude::Inverted {
        //log::debug!("Entry is inverted");
        // Figure library assumes no inversion
        // Invert all figure elements
        invert_figure_elements(&mut figure_elements, 0);
        current_attitude = Attitude::get_inverted(current_attitude);
    }

    log::debug!("IFR: {:#?}", figure_elements);
    log::debug!("Current_inversion {:?}", current_attitude);
    if current_attitude != exit_attitude {
        log::debug!("Mismatched exit!");
        // Mismatched exit
        // Step back to find invertible radius
        let mut rev_idx_invert_from = figure_elements.len();
        for (i, element) in figure_elements.iter_mut().rev().enumerate() {
            match element.elem_type {
                ElementType::Radius => {
                    if element.matching >= 0 {
                        rev_idx_invert_from = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        log::debug!("Inverting from {}", rev_idx_invert_from);
        let invert_from = figure_elements.len() - 1 - rev_idx_invert_from;
        invert_figure_elements(&mut figure_elements, invert_from);
    }

    figure.append(figure_elements);

    // Add exit line
    match exit_attitude {
        Attitude::Normal => {
            figure.push(Element::line(0));
        }
        Attitude::Inverted => {
            figure.push(Element::invline(0));
        }
        _ => {
            panic!("Invalid figure exit attitude.")
        }
    }

    return figure;
}

fn is_plain_radius(elem: &Element) -> bool {
    elem.elem_type == ElementType::Radius && elem.roll_type == RollType::None
}

fn combine_contiguous_radii(figure: &mut Figure) {
    let batched_elements = figure
        .elements
        .iter()
        .peekable()
        .batching(|iter| {
            match iter.next() {
                None => None,
                Some(elem) => {
                    if !is_plain_radius(elem) {
                        Some(elem.clone())
                    } else {
                        let mut total_angle = elem.main_angle;
                        while let Some(next_elem_peek) = iter.clone().peek() {
                            if !is_plain_radius(next_elem_peek) {
                                break;
                            }
                            if elem.matching == next_elem_peek.matching
                                && elem.main_angle.signum() == next_elem_peek.main_angle.signum()
                            {
                                // Consume the element
                                let next_elem = iter.next().unwrap();
                                total_angle += next_elem.main_angle
                            }
                        }
                        Some(Element {
                            main_angle: total_angle,
                            ..elem.clone()
                        })
                    }
                }
            }
        })
        .collect();
    figure.elements = batched_elements
}

pub fn parse_sequence(olan_string: String) -> Sequence {
    let mut sequence = Sequence::new();

    let olan_sequence = OlanParser::parse(Rule::sequence, &olan_string)
        .expect("Failed to parse sequence")
        .next()
        .unwrap();
    let inner_rules = olan_sequence.into_inner();

    for olan_sequence_part in inner_rules {
        match olan_sequence_part.into_inner().next() {
            Some(x) => {
                sequence.push(parse_figure(x));
            }
            _ => {}
        }
    }
    for figure in &mut sequence.figures {
        // Remove doubled lines
        figure
            .elements
            .dedup_by(|a, b| a.elem_type == ElementType::Line && a == b);

        combine_contiguous_radii(figure);
    }
    // With sequence parsed, need to check for:
    // - Inverted flight continuity
    // - Spin entry trimming

    return sequence;
}
