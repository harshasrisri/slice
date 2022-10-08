use slice::fields::FieldSpecParser;

#[test]
fn test_field_neg_all_commas() {
    assert!(FieldSpecParser::default().from_spec(",,,,,,,").is_err());
}

#[test]
fn test_field_neg_all_hyphens() {
    assert!(FieldSpecParser::default().from_spec("-------").is_err());
}

#[test]
fn test_field_neg_empty_field_spec() {
    assert!(FieldSpecParser::default().from_spec("").is_err());
}

#[test]
fn test_field_neg_two_leading_hyphens() {
    assert!(FieldSpecParser::default().from_spec("--4,5-6,7-").is_err());
    assert!(FieldSpecParser::default().from_spec("--,5-6,7-").is_err());
}

#[test]
fn test_field_neg_two_trailing_hyphens() {
    assert!(FieldSpecParser::default().from_spec("-4,5-6,7--").is_err());
    assert!(FieldSpecParser::default().from_spec("-4,5-6,--").is_err());
}

#[test]
fn test_field_neg_two_hyphens_in_range() {
    assert!(FieldSpecParser::default().from_spec("-4,5--6,7-").is_err());
}

#[test]
fn test_field_neg_more_than_two_parts_in_range() {
    assert!(FieldSpecParser::default().from_spec("-4,5-6-7,8-").is_err());
    assert!(FieldSpecParser::default()
        .from_spec("-4,5-6-7-8,9-")
        .is_err());
}

#[test]
fn test_field_neg_open_ranges_in_middle() {
    assert!(FieldSpecParser::default()
        .from_spec("-4,5-6,7-8,9-,10")
        .is_err());
    assert!(FieldSpecParser::default()
        .from_spec("2,-4,5-6,7-8,10")
        .is_err());
}

#[test]
fn test_field_neg_invalid_ranges_in_start() {
    assert!(FieldSpecParser::default()
        .from_spec("2-,3-4,5-6,7-8,10")
        .is_err());
    assert!(FieldSpecParser::default()
        .from_spec("-2-4,5-6,7-8,10")
        .is_err());
    assert!(FieldSpecParser::default()
        .from_spec("-,5-6,7-8,10")
        .is_err());
}

#[test]
fn test_field_neg_invalid_ranges_in_end() {
    assert!(FieldSpecParser::default()
        .from_spec("-4,5-6,7-8,-10")
        .is_err());
    assert!(FieldSpecParser::default()
        .from_spec("-3,5-6,7-8,10-12-")
        .is_err());
    assert!(FieldSpecParser::default()
        .from_spec("-4,5-6,7-8,-")
        .is_err());
}

#[test]
fn test_field_simple_sequence() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("1,2,3,4,5,6").is_ok());

    assert!(field.valid(1));
    assert!(field.valid(3));
    assert!(field.valid(6));

    assert!(!field.valid(11));
    assert!(!field.valid(31));
    assert!(!field.valid(16));
}

#[test]
fn test_field_ordered_scatterred_numbers() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("2,6,9").is_ok());

    assert!(field.valid(2));
    assert!(field.valid(6));
    assert!(field.valid(9));

    assert!(!field.valid(1));
    assert!(!field.valid(3));
    assert!(!field.valid(7));
    assert!(!field.valid(11));
}

#[test]
fn test_field_unordered_scatterred_numbers() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("9,2,6").is_ok());

    assert!(field.valid(2));
    assert!(field.valid(6));
    assert!(field.valid(9));

    assert!(!field.valid(1));
    assert!(!field.valid(3));
    assert!(!field.valid(7));
    assert!(!field.valid(11));
}

#[test]
fn test_field_single_leading_hyphen() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("-4,5").is_ok());

    assert!(field.valid(1));
    assert!(field.valid(3));
    assert!(field.valid(5));

    assert!(!field.valid(6));
    assert!(!field.valid(21));
}

#[test]
fn test_field_single_trailing_hyphen() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("6,7-").is_ok());

    assert!(!field.valid(1));
    assert!(!field.valid(3));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(7));
    assert!(field.valid(8));
    assert!(field.valid(15));
}

#[test]
fn test_field_single_range() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("6-12").is_ok());

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));

    assert!(!field.valid(14));
    assert!(!field.valid(15));
}

#[test]
fn test_field_ordered_ranges() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("6-12,20-31").is_ok());

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));

    assert!(!field.valid(14));
    assert!(!field.valid(15));

    assert!(field.valid(26));
    assert!(field.valid(29));
    assert!(field.valid(21));

    assert!(!field.valid(32));
    assert!(!field.valid(41));
}

#[test]
fn test_field_unordered_ranges() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("20-31,6-12").is_ok());

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));

    assert!(!field.valid(14));
    assert!(!field.valid(15));

    assert!(field.valid(26));
    assert!(field.valid(29));
    assert!(field.valid(21));

    assert!(!field.valid(32));
    assert!(!field.valid(41));
}

#[test]
fn test_field_overlapping_ranges() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("10-16,6-12").is_ok());

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));
    assert!(field.valid(14));
    assert!(field.valid(15));

    assert!(!field.valid(17));
    assert!(!field.valid(19));
    assert!(!field.valid(21));
}

#[test]
fn test_field_range_and_numbers() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("3,6-12,17").is_ok());

    assert!(field.valid(3));

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));

    assert!(!field.valid(14));
    assert!(!field.valid(15));

    assert!(field.valid(17));
}

#[test]
fn test_field_range_and_numbers_open() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("-3,6-12,17-").is_ok());

    assert!(field.valid(1));
    assert!(field.valid(3));

    assert!(!field.valid(4));
    assert!(!field.valid(5));

    assert!(field.valid(6));
    assert!(field.valid(9));
    assert!(field.valid(12));

    assert!(!field.valid(14));
    assert!(!field.valid(15));

    assert!(field.valid(17));
    assert!(field.valid(18));
}

#[test]
fn test_field_complex_spec() {
    let mut field = FieldSpecParser::default();
    assert!(field.from_spec("-3,12-15,,7-9,,5,14-17,19-").is_ok());

    assert!(field.valid(1));
    assert!(field.valid(3));

    assert!(!field.valid(4));
    assert!(!field.valid(6));

    assert!(field.valid(7));
    assert!(field.valid(9));

    assert!(!field.valid(10));
    assert!(!field.valid(11));

    assert!(field.valid(12));
    assert!(field.valid(14));
    assert!(field.valid(15));
    assert!(field.valid(17));

    assert!(!field.valid(18));

    assert!(field.valid(19));
    assert!(field.valid(20));
    assert!(field.valid(30));
}
