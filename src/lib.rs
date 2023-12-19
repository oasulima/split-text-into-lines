use anyhow::{bail, Result};
use thiserror::Error;

const SPACE: &str = " ";
const SPACE_SIZE: usize = SPACE.len();
const LINE_BREAK: &str = "\n";

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("'{0}' lenght is more than {1}")]
struct WordTooLongError(String, usize);

pub fn transform(input: &str, line_width: usize) -> Result<String> {
    let mut input = input;
    if input.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::from("");
    let max_words_number = line_width / 2 + 1;
    let mut current_line_words: Vec<&str> = Vec::with_capacity(max_words_number);
    let mut current_line_min_length: usize = 0;

    loop {
        let input_len = input.len();
        if input_len == 0 {
            break;
        }

        let GetNextWordResult {
            next_word: word,
            shifted_input,
        } = get_next_word(input, input_len);

        input = shifted_input;

        if word.is_empty() {
            continue;
        }

        if word.len() > line_width {
            bail!(WordTooLongError(word.to_string(), line_width));
        }

        handle_word(
            &mut current_line_words,
            &mut current_line_min_length,
            line_width,
            &mut result,
            word,
        );
    }

    if !current_line_words.is_empty() {
        add_line_to_result(
            line_width,
            &mut current_line_min_length,
            &mut current_line_words,
            &mut result,
        );
    }

    Ok(result)
}

fn handle_word<'a>(
    current_line_words: &mut Vec<&'a str>,
    current_line_min_length: &mut usize,
    line_width: usize,
    result: &mut String,
    word: &'a str,
) {
    let word_len = word.len();

    let mut new_line_min_length = {
        if current_line_words.is_empty() {
            word_len
        } else {
            *current_line_min_length + SPACE_SIZE + word_len
        }
    };

    if new_line_min_length > line_width {
        add_line_to_result(
            line_width,
            current_line_min_length,
            current_line_words,
            result,
        );
        new_line_min_length = word_len;
    }

    current_line_words.push(word);

    *current_line_min_length = new_line_min_length;
}

struct GetNextWordResult<'a> {
    next_word: &'a str,
    shifted_input: &'a str,
}

fn get_next_word(input: &str, input_len: usize) -> GetNextWordResult {
    let after_word_index = input.find(SPACE).unwrap_or(input_len);
    let next_word = &input[0..after_word_index];

    let shifted_input = {
        if after_word_index < input_len {
            &input[after_word_index + SPACE_SIZE..input_len]
        } else {
            ""
        }
    };
    GetNextWordResult {
        next_word,
        shifted_input,
    }
}

fn add_line_to_result(
    line_width: usize,
    current_line_min_length: &mut usize,
    current_line_words: &mut Vec<&str>,
    result: &mut String,
) {
    if !result.is_empty() {
        result.push_str(LINE_BREAK);
    }

    let free_space = line_width - *current_line_min_length;

    let separators_number = current_line_words.len() - 1;
    let separator_min_len = free_space
        .checked_div(separators_number)
        .unwrap_or_default();
    let mut extra_spaces = free_space - separators_number * separator_min_len;

    for (index, word) in current_line_words.iter().enumerate() {
        if index > 0 {
            let curr_separator_extra_len = {
                SPACE_SIZE
                    + if extra_spaces > 0 {
                        extra_spaces -= 1;
                        1
                    } else {
                        0
                    }
            };
            let separator_len = separator_min_len + curr_separator_extra_len;
            add_separator(separator_len, result);
        }
        result.push_str(word);
    }

    if extra_spaces > 0 {
        add_separator(extra_spaces, result);
    }

    current_line_words.clear();
    *current_line_min_length = 0;
}

fn add_separator(separator_len: usize, result: &mut String) {
    for _ in 0..separator_len {
        result.push_str(SPACE);
    }
}

#[cfg(test)]
mod tests {
    use crate::WordTooLongError;

    use super::transform;

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
}
