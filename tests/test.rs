use std::fs::File;

use dwarfview::Region;

#[test]
fn check_name() {
    let region = Region::from_file(File::open("tests/data.xml").unwrap());
    assert_eq!(region.name, "Ngutegnitom");
}
