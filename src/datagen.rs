use std::fs;

use arusti;

fn main() {
    // Open file
    let unparsed_file = fs::read_to_string("tests/resources/Example.seq").expect("cannot read file");
    let doc = roxmltree::Document::parse(&unparsed_file).unwrap();
    let elem = doc.descendants().find(|n| n.tag_name().name() == "sequence_text").unwrap();

    // Parse OLAN
    let sequence_str = elem.text().unwrap().to_string();
    //let sequence_str = "/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44".to_string();
    //let sequence_str = ".''s.''irp...'-~".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);
    println!("{:#?}",sequence);
    
    // Generate points for sequence
    for figure in sequence {
        for element in figure {
            // Generate points
            }
        }
    }