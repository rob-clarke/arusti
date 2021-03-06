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
    //let sequence_str = "dq v irp 2j o".to_string();
    let sequence = arusti::olan::parse_sequence(sequence_str);
    eprintln!("{:#?}",sequence);
    
    let mut generator = arusti::DataPointGenerator::new(
        arusti::Vector3::x() * 18.0,
        arusti::Vector3::zeros(),
        arusti::PerformanceOptions{
            roll_rate: 180.0,
            snap_rate: 360.0,
            spin_rate: 90.0,
            turn_rate: 22.5,
            hesitation_time: 0.2,
            });
        
    let points = generator.generate_points(&sequence);

    for point in points {
        println!("{},{},{}",point.position[0],point.position[1],point.position[2]);
        }

    }

/*
OLAN doesn't have any RC-specific figures like Harrier, Prop hang etc

Free letters:

e
l
u
x

The grammar currently will not match trailing letters it does not
recognize so letters appended to current figures should work

*/