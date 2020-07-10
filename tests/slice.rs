use slice::fields::FieldParser;

#[test]
fn test_field_neg_all_commas() {
    assert!(FieldParser::from_spec(",,,,,,,", false).is_err());
}

#[test]
fn test_field_neg_all_hyphens() {
    assert!(FieldParser::from_spec("-------", false).is_err());
}

#[test]
fn test_field_neg_empty_field_spec() {
    assert!(FieldParser::from_spec("", false).is_err());
}

#[test]
fn test_field_neg_two_leading_hyphens() {
    assert!(FieldParser::from_spec("--4,5-6,7-", false).is_err());
    assert!(FieldParser::from_spec("--,5-6,7-", false).is_err());
}

#[test]
fn test_field_neg_two_trailing_hyphens() {
    assert!(FieldParser::from_spec("-4,5-6,7--", false).is_err());
    assert!(FieldParser::from_spec("-4,5-6,--", false).is_err());
}

#[test]
fn test_field_neg_two_hyphens_in_range() {
    assert!(FieldParser::from_spec("-4,5--6,7-", false).is_err());
}

#[test]
fn test_field_neg_more_than_two_parts_in_range() {
    assert!(FieldParser::from_spec("-4,5-6-7,8-", false).is_err());
    assert!(FieldParser::from_spec("-4,5-6-7-8,9-", false).is_err());
}

#[test]
fn test_field_neg_open_ranges_in_middle() {
    assert!(FieldParser::from_spec("-4,5-6,7-8,9-,10", false).is_err());
    assert!(FieldParser::from_spec("2,-4,5-6,7-8,10", false).is_err());
}

#[test]
fn test_field_neg_invalid_ranges_in_start() {
    assert!(FieldParser::from_spec("2-,3-4,5-6,7-8,10", false).is_err());
    assert!(FieldParser::from_spec("-2-4,5-6,7-8,10", false).is_err());
    assert!(FieldParser::from_spec("-,5-6,7-8,10", false).is_err());
}

#[test]
fn test_field_neg_invalid_ranges_in_end() {
    assert!(FieldParser::from_spec("-4,5-6,7-8,-10", false).is_err());
    assert!(FieldParser::from_spec("-3,5-6,7-8,10-12-", false).is_err());
    assert!(FieldParser::from_spec("-4,5-6,7-8,-", false).is_err());
}

#[test]
fn test_field_simple_sequence() {
    let field = FieldParser::from_spec("1,2,3,4,5,6", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(1));
    assert!(field.valid_field(3));
    assert!(field.valid_field(6));

    assert!(!field.valid_field(11));
    assert!(!field.valid_field(31));
    assert!(!field.valid_field(16));
}

#[test]
fn test_field_ordered_scatterred_numbers() {
    let field = FieldParser::from_spec("2,6,9", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(2));
    assert!(field.valid_field(6));
    assert!(field.valid_field(9));

    assert!(!field.valid_field(1));
    assert!(!field.valid_field(3));
    assert!(!field.valid_field(7));
    assert!(!field.valid_field(11));
}

#[test]
fn test_field_unordered_scatterred_numbers() {
    let field = FieldParser::from_spec("9,2,6", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(2));
    assert!(field.valid_field(6));
    assert!(field.valid_field(9));

    assert!(!field.valid_field(1));
    assert!(!field.valid_field(3));
    assert!(!field.valid_field(7));
    assert!(!field.valid_field(11));
}

#[test]
fn test_field_single_leading_hyphen() {
    let field = FieldParser::from_spec("-4,5", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(1));
    assert!(field.valid_field(3));
    assert!(field.valid_field(5));

    assert!(!field.valid_field(6));
    assert!(!field.valid_field(21));
}

#[test]
fn test_field_single_trailing_hyphen() {
    let field = FieldParser::from_spec("6,7-", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(!field.valid_field(1));
    assert!(!field.valid_field(3));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(7));
    assert!(field.valid_field(8));
    assert!(field.valid_field(15));
}

#[test]
fn test_field_single_range() {
    let field = FieldParser::from_spec("6-12", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));

    assert!(!field.valid_field(14));
    assert!(!field.valid_field(15));
}

#[test]
fn test_field_ordered_ranges() {
    let field = FieldParser::from_spec("6-12,20-31", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));

    assert!(!field.valid_field(14));
    assert!(!field.valid_field(15));

    assert!(field.valid_field(26));
    assert!(field.valid_field(29));
    assert!(field.valid_field(21));

    assert!(!field.valid_field(32));
    assert!(!field.valid_field(41));
}

#[test]
fn test_field_unordered_ranges() {
    let field = FieldParser::from_spec("20-31,6-12", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));

    assert!(!field.valid_field(14));
    assert!(!field.valid_field(15));

    assert!(field.valid_field(26));
    assert!(field.valid_field(29));
    assert!(field.valid_field(21));

    assert!(!field.valid_field(32));
    assert!(!field.valid_field(41));
}

#[test]
fn test_field_overlapping_ranges() {
    let field = FieldParser::from_spec("10-16,6-12", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));
    assert!(field.valid_field(14));
    assert!(field.valid_field(15));

    assert!(!field.valid_field(17));
    assert!(!field.valid_field(19));
    assert!(!field.valid_field(21));
}

#[test]
fn test_field_range_and_numbers() {
    let field = FieldParser::from_spec("3,6-12,17", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(3));

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));

    assert!(!field.valid_field(14));
    assert!(!field.valid_field(15));

    assert!(field.valid_field(17));
}

#[test]
fn test_field_range_and_numbers_open() {
    let field = FieldParser::from_spec("-3,6-12,17-", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(1));
    assert!(field.valid_field(3));

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(5));

    assert!(field.valid_field(6));
    assert!(field.valid_field(9));
    assert!(field.valid_field(12));

    assert!(!field.valid_field(14));
    assert!(!field.valid_field(15));

    assert!(field.valid_field(17));
    assert!(field.valid_field(18));
}

#[test]
fn test_field_complex_spec() {
    let field = FieldParser::from_spec("-3,12-15,,7-9,,5,14-17,19-", false);
    assert!(field.is_ok());

    let field = field.unwrap();

    assert!(field.valid_field(1));
    assert!(field.valid_field(3));

    assert!(!field.valid_field(4));
    assert!(!field.valid_field(6));

    assert!(field.valid_field(7));
    assert!(field.valid_field(9));

    assert!(!field.valid_field(10));
    assert!(!field.valid_field(11));

    assert!(field.valid_field(12));
    assert!(field.valid_field(14));
    assert!(field.valid_field(15));
    assert!(field.valid_field(17));

    assert!(!field.valid_field(18));

    assert!(field.valid_field(19));
    assert!(field.valid_field(20));
    assert!(field.valid_field(30));
}
