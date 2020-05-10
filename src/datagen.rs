use std::fs;

use arusti;

fn main() {
    let unparsed_file = fs::read_to_string("test/sequence.olan").expect("cannot read file");

    let sequence = "/dq v .''s.''irp...'-~ ~----2j- [0,20] -'',24'' 2> c,24.... [0,22] ~+v-- 4> -id2 2> ''1''m2.' [0,20] ~~++++++2j2 f,2- -22a44".to_string();
    //let sequence = "1".to_string();
    let result = arusti::olan::parse_sequence(sequence);
    println!("{:#?}",result);
    
    // Open file
    // Parse YAML
    // Generate points for sequence
    }