use zoker_parser;
use zoker_parser::location::Location;

#[test]
fn test_location_new() {
    let loc = Location::new(3, 7);
    assert_eq!(loc.row(), 3);
    assert_eq!(loc.column(), 7);
}
