use split_text_into_lines::{transform, WordTooLongError};

#[test]
fn simple() {
    // Arrange
    let test_cases = [
        ("", 5, ""),
        ("test", 5, "test "),
        ("Lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua", 12,
         "Lorem  ipsum\ndolor    sit\namet        \nconsectetur \nadipiscing  \nelit  sed do\neiusmod     \ntempor      \nincididunt  \nut labore et\ndolore magna\naliqua      "),
        ("Lorem     ipsum    dolor", 17, "Lorem ipsum dolor"),
    ];

    for &(input, line_width, expected) in &test_cases {
        println!("input: '{}'", input);
        // Act
        let result = transform(input, line_width);

        //Assert
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result, expected);
    }
}

#[test]
fn word_is_too_long() {
    // Arrange
    let input = "Loremipsumdolor";
    let line_width = 5;

    // Act
    let result = transform(input, line_width);

    //Assert
    assert!(result.is_err_and(|x| x.is::<WordTooLongError>()));
}

#[test]
fn unicode() {
    // Arrange
    let input = "ΒΓΑ  ΒΓΔ  γβα";
    let line_width = 11;
    let expected = "ΒΓΑ ΒΓΔ γβα";

    // Act
    let result = transform(input, line_width);

    //Assert
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result, expected);
}
