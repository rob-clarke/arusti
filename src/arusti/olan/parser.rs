use pest::Parser;

#[derive(Parser)]
#[grammar = "arusti/olan/olan.pest"]
struct OLANParser;

use crate::types::{ElementType, Element, Figure, Sequence };

use pest::iterators::Pair;

fn get_element_for_roll_element(roll_element_pair: Pair<Rule>, reverse_roll: bool) -> Element {
    let roll_angle : i8;
    let mut roll_divisions : i8 = 0;
    let mut elem_type : ElementType = ElementType::Roll;
    let mut inverted : bool = false;

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
                roll_divisions = current_pair.as_str().parse().unwrap();
                }
            Rule::flick_spin_type => {
                elem_type = match current_pair.as_str() {
                    "f" | "if" => ElementType::Flick,
                    "s" | "is" => ElementType::Spin,
                    _ => unreachable!()
                    };
                if current_pair.as_str().contains("i") {
                    inverted = true;
                    }
                }
            _ => {
                unreachable!();
                }
            }
        }

    let computed_roll : f32 = match roll_divisions {
        0 => match roll_angle {
            1 => 360.0,
            2 => 180.0,
            3 => 270.0,
            4 => 90.0,
            5 => 450.0,
            6 => 540.0,
            7 => 630.0,
            9 => 720.0,
            _ => { unreachable!(); }
            },
        2 => match roll_angle {
            2 => 360.0,
            3 => 540.0,
            4 => 720.0,
            _ => { unreachable!(); }
            },
        4 => match roll_angle {
            2 => 180.0,
            3 => 270.0,
            4 => 360.0,
            5 => 450.0,
            6 => 540.0,
            7 => 630.0,
            8 => 720.0,
            _ => { unreachable!(); }
            },
        8 => match roll_angle {
            2 => 90.0,
            4 => 180.0,
            6 => 270.0,
            8 => 360.0,
            _ => { unreachable!(); }
            }
        _ => { unreachable!(); }
        };

    Element {
        elem_type: elem_type,
        inverted: inverted,
        angle: if reverse_roll { -computed_roll } else { computed_roll },
        argument: roll_divisions as f32
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
                    elements.push( get_element_for_roll_element(current_pair,reverse_state) );
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
            }
        else {
            break;
            }
        }

    return elements;
    }

use crate::olan::figure_defs;

fn get_elements_for_main_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    let current_pair = figure_pair.into_inner().next().unwrap();

    match current_pair.as_rule() {
        Rule::single_line     => figure_defs::get_elements_for_single_line(current_pair),
        Rule::twin_line       => figure_defs::get_elements_for_twin_line(current_pair),
        Rule::loop_figure     => figure_defs::get_elements_for_loop_figure(current_pair),
        Rule::loop_line_combo => figure_defs::get_elements_for_loop_line_combo(current_pair),
        Rule::double_loop     => figure_defs::get_elements_for_double_loop(current_pair),
        Rule::humpty          => figure_defs::get_elements_for_humpty(current_pair),
        Rule::hammerhead      => figure_defs::get_elements_for_hammerhead(current_pair),
        Rule::three_roll      => figure_defs::get_elements_for_three_roll(current_pair),
        Rule::extra           => figure_defs::get_elements_for_extra(current_pair),
        Rule::non_aresti      => figure_defs::get_elements_for_non_aresti(current_pair),
        _ => { unreachable!(); }
        }
    }

fn find_combining_with_arg(elements: &Vec<Element>, argument: i8) -> Option<usize> {
    let result = elements.iter().enumerate().find(
        |(_,elem)| {
            (elem.elem_type == ElementType::Combining) && ( elem.argument == ( argument as f32 ) )
            }
        );
    match result {
        Some((idx,_)) => Some(idx),
        None => None,
        }
    }

fn insert_combining_rolls(elements: &mut Vec<Element>, position: i8, roll_set_opt: Option<Vec<Element>>) -> Option<Vec<Element>> {
    // Xfif(Y)(Z)W
    // X => -1
    // Y => 1
    // Z => 2
    // W => 0
    let contains_combining = find_combining_with_arg(&elements,position);

    if let Some(idx) = contains_combining {
        if let Some(roll_set) = roll_set_opt {
            // Insert the roll set & append
            elements.splice(idx..idx+1, roll_set);
            return None;
            }
        else {
            // Remove the combining marker & append
            elements.remove(idx);
            return roll_set_opt;
            }
        }
    else {
        return roll_set_opt;
        }
    }

fn get_elements_for_named_figure(figure_pair: Pair<Rule>) -> Vec<Element> {
    let mut inner_pairs = figure_pair.into_inner();
    let mut current_pair = inner_pairs.next().unwrap();

    let mut elements = Vec::<Element>::new();

    let mut entry_roll_set_opt  : Option<Vec<Element>> = None;
    let mut inner_roll_set_opt  : Option<Vec<Element>> = None;
    let mut inner_roll_set2_opt : Option<Vec<Element>> = None;
    let mut exit_roll_set_opt   : Option<Vec<Element>> = None;

    if current_pair.as_rule() == Rule::roll_set {
        entry_roll_set_opt = Some( get_elements_for_roll_set(current_pair) );
        current_pair = inner_pairs.next().unwrap();
        }
    
    let mut main_figure_elements = get_elements_for_main_figure(current_pair);

    // Try to unwrap next pair
    while let Some(current_pair) = inner_pairs.next() {
        match current_pair.as_rule() {
            Rule::inner_roll_set => {
                if inner_roll_set_opt.is_none() {
                    inner_roll_set_opt = Some( get_elements_for_roll_set(current_pair.into_inner().next().unwrap()) );
                    }
                else if inner_roll_set2_opt.is_none() {
                    inner_roll_set2_opt = Some( get_elements_for_roll_set(current_pair.into_inner().next().unwrap()) );
                    }
                }
            Rule::roll_set => {
                exit_roll_set_opt = Some( get_elements_for_roll_set(current_pair) );
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
        }
    else {
        // Rolling figure is actually just a line
        Vec::<Element>::new()
        }

    }

fn is_transition_inverted(line_extension: &Pair<Rule>) -> bool {
    line_extension.as_str().to_string().contains("-")
    }

fn parse_figure(olan_figure: Pair<Rule>) -> Figure {
    let mut figure = Figure::new();

    let mut inner_pairs = olan_figure.into_inner();

    let mut current_pair = inner_pairs.next().unwrap();

    // If has initial inter_line_extension, check for inverted entry
    let mut entry_is_inverted = false;
    if current_pair.as_rule() == Rule::inter_line_extension {
        entry_is_inverted = is_transition_inverted(&current_pair);
        // Advance current_pair
        current_pair = inner_pairs.next().unwrap();
        }

    // Add entry line
    if entry_is_inverted {
        figure.push( Element::invline(0.0) );
        }
    else {
        figure.push( Element::line(0.0) );
        }
    
    let figure_elements = match current_pair.as_rule() {
        Rule::named_figure   => get_elements_for_named_figure(current_pair),
        Rule::rolling_figure => get_elements_for_rolling_figure(current_pair),
        Rule::rolling_turn   => figure_defs::get_elements_for_rolling_turn(current_pair),
        _ => unreachable!()
        };
    figure.append(figure_elements);

    // If has trailing inter_line_extension, check for inverted exit
    let mut exit_is_inverted = false;
    if let Some(current_pair) = inner_pairs.next() {
        // Will be inter_line_extension by parser rules
        exit_is_inverted = is_transition_inverted(&current_pair);
        }

    // Add exit line
    if exit_is_inverted {
        figure.push( Element::invline(0.0) );
        }
    else {
        figure.push( Element::line(0.0) );
        }

    return figure;
    }

pub fn parse_sequence(olan_string: String) -> Sequence {
    let mut sequence = Sequence::new();

    let olan_sequence = OLANParser::parse(Rule::sequence, &olan_string)
        .expect("Failed to parse sequence")
        .next().unwrap();
    
    let inner_rules = olan_sequence.into_inner();

    for olan_sequence_part in inner_rules {
        match olan_sequence_part.into_inner().next() {
            Some(x) => {
                sequence.push( parse_figure(x) );
                }
            _ => {}
            }
        }
    
    return sequence;

    /*
    match sequence.into_inner().as_rule() {
        Rule::figure => ;
        _ => ;
        }
    */
    
    }
