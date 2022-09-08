use std::fs::File;

use dwarfview::Region;

#[test]
fn check_main_fields() {
    let region = Region::from(File::open("tests/data.xml").unwrap());
    assert_eq!(region.name, "Ngutegnitom");
    assert_eq!(region.altname, "The Cyclopean Planets");
}
