use svg_hush::Filter;

use penumbra_registry::sanitize::sanitize_svg;

#[test]
fn test_sanitize_valid_svg() {
    // let svg = "<svg xmlns='http://www.w3.org/2000/svg' />".as_bytes();
    // let result = sanitize_svg(svg).unwrap();
    // let back = String::from_utf8(result).unwrap();
    //
    // let test = 1;

    let svg = r##"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:svg="http://www.w3.org/2000/svg" xmlns:vector="http://www.w3.org/2000/svg">
        <rect height="300" width="300"/>
        <svg:rect height="200" width="200">
            <title>Test</title>
        </svg:rect>
        <vector:rect height="100" width="100"/>
        <svg:text xml:space="preserve">  Hallo World  </svg:text>
    </svg>
    "##;

    let f = Filter::new();
    let mut out = Vec::new();
    let mut out2 = Vec::new();
    f.filter(&mut svg.as_bytes(), &mut out).unwrap();
    f.filter(&mut out.as_slice(), &mut out2).unwrap();
    assert_eq!(&out, &out2);
}

#[test]
fn test_sanitize_svg_with_scripts() {
    let svg_with_script =
        b"<svg><script>alert('XSS')</script><rect width=\"100\" height=\"100\"/></svg>";
    let expected_sanitized_svg = b"<svg><rect width=\"100\" height=\"100\"/></svg>";
    let result = sanitize_svg(svg_with_script);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected_sanitized_svg);
}

#[test]
fn test_sanitize_invalid_svg() {
    let invalid_svg = b"not a valid svg";
    let result = sanitize_svg(invalid_svg);

    assert!(result.is_err());
    // Here you might want to assert that the error is the correct type,
    // e.g., `assert!(matches!(result.unwrap_err(), AppError::FilterError(_)));`
}

#[test]
fn test_sanitize_empty_input() {
    let empty_svg = b"";
    let result = sanitize_svg(empty_svg);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), empty_svg);
}
