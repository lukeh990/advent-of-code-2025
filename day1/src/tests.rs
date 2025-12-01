use super::*;

#[test]
fn test_parse_rotation() {
    assert_eq!(parse_rotation("R50"), Ok(Rotation::Right(50)));
    assert_eq!(parse_rotation("L50"), Ok(Rotation::Left(50)));
    assert_eq!(
        parse_rotation("E50"),
        Err(ParseError::InvalidDirection('E'))
    );
    assert_eq!(parse_rotation(""), Err(ParseError::EmptyLine));
    assert_eq!(
        parse_rotation("R42949672950"),
        Err(ParseError::Overflow(String::from("R42949672950")))
    );
    assert_eq!(parse_rotation("R5F"), Err(ParseError::InvalidDigit('F')));
}

#[test]
fn test_analyze_roatations() {
    let sample = fs::read_to_string("sample.txt").unwrap();

    let res = analyze_roatations(50, &sample, false);

    assert_eq!(res, Ok(3));

    let res = analyze_roatations(50, &sample, true);

    assert_eq!(res, Ok(6));
}
