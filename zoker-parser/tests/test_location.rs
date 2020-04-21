use zoker_parser::location::Location;

#[test]
fn test_location_new() {
    let loc = Location::new(3, 7);
    assert_eq!(loc.row(), 3);
    assert_eq!(loc.column(), 7);
}

#[test]
fn test_location_reset() {
    let mut loc = Location::new(3, 7);
    loc.reset();
    assert_eq!(loc.row(), 0);
    assert_eq!(loc.column(), 0);
}

#[test]
fn test_location_go_right() {
    let mut loc = Location::new(3, 7);
    loc.go_right();
    assert_eq!(loc.row(), 3);
    assert_eq!(loc.column(), 8);
}

#[test]
fn test_location_new_line() {
    let mut loc = Location::new(3, 7);
    loc.new_line();
    assert_eq!(loc.row(), 4);
    assert_eq!(loc.column(), 1);
}
