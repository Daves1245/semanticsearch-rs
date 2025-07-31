/* All of these chunking strategies assume we're in markdown */

use regex::Regex;

/*
pub async fn chunk_grammar(content: String) -> Arc<String> {
    // different types of chunks:
    // clause (commas, semicolons, colons)
    // sentences
    // paragraphs
    //
    // these can all be approaches using regexes
}
*/

fn parse_with_delimiters(content: &str, delimiters: &[char]) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_chunk = String::new();

    for cur_char in content.chars() {
        current_chunk.push(cur_char);
        if delimiters.contains(&cur_char) {
            if !current_chunk.is_empty() {
                result.push(current_chunk.clone());
            }
            current_chunk.clear();
        }
    }

    if !current_chunk.is_empty() {
        if !current_chunk.is_empty() {
            result.push(current_chunk.to_string());
        }
    }

    result
}

pub fn parse_semantic(content: &str) -> Vec<String> {
    let delimiters = ['.', '!', '?', '\n', ','];
    parse_with_delimiters(content, &delimiters)
}

// we define sections to be blocks delimited by:
// - a header (1-6 '#' in a row)
// - a horizontal rule (---, ___, or ***)
// - a double newline \n\n.
// will return an array representing a partitioning of the original
// string with every element except possibly the last ending
// in a delimeter.
pub fn parse_sections(content: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_section = String::new();
    let mut chars = content.chars().peekable();
    let mut at_line_start = true;
    let mut consecutive_newlines = 0;

    while let Some(ch) = chars.next() {
        match ch {
            '#' if at_line_start => {
                // num consecutive '#' characters
                let mut hash_count = 1;
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == '#' && hash_count < 6 {
                        hash_count += 1;
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check if it's followed by a space or any non-# character (valid header)
                if let Some(&next_ch) = chars.peek() {
                    if next_ch != '#' {  // valid header if not followed by another #
                        if !current_section.is_empty() {
                            result.push(current_section.to_string());
                            current_section.clear();
                        }
                        // start new section with the header
                        current_section.push_str(&"#".repeat(hash_count));
                    } else {
                        // not a valid header, treat as regular content
                        current_section.push_str(&"#".repeat(hash_count));
                    }
                } else {
                    // end of input, treat as header
                    if !current_section.is_empty() {
                        result.push(current_section.to_string());
                        current_section.clear();
                    }
                    current_section.push_str(&"#".repeat(hash_count));
                }
                at_line_start = false;
            },
            '-' if at_line_start => {

                // check for horizontal rule (--- or more)
                let mut dash_count = 1;
                let mut temp_chars = chars.clone();
                while let Some(next_ch) = temp_chars.next() {
                    if next_ch == '-' {
                        dash_count += 1;
                    } else if next_ch == '\n' || next_ch == ' ' {
                        break;
                    } else {
                        dash_count = 0;
                        break;
                    }
                }

                if dash_count >= 3 {
                    // horizontal rule
                    if !current_section.is_empty() {
                        result.push(current_section.to_string());
                        current_section.clear();
                    }
                    // skip the rest of the rule
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    current_section.push(ch);
                }
                at_line_start = false;
            },
            '*' if at_line_start => {
                // check for horizontal rule (*** or more)
                let mut star_count = 1;
                let mut temp_chars = chars.clone();
                while let Some(next_ch) = temp_chars.next() {
                    if next_ch == '*' {
                        star_count += 1;
                    } else if next_ch == '\n' || next_ch == ' ' {
                        break;
                    } else {
                        star_count = 0;
                        break;
                    }
                }

                if star_count >= 3 {
                    // horizontal rule
                    if !current_section.is_empty() {
                        result.push(current_section.to_string());
                        current_section.clear();
                    }
                    // skip the rest
                    while let Some(&next_ch) = chars.peek() {
                        if next_ch == '\n' {
                            break;
                        }
                        chars.next();
                    }
                } else {
                    current_section.push(ch);
                }
                at_line_start = false;
            },
            // double new lines!
            '\n' => {
                current_section.push(ch);
                consecutive_newlines += 1;
                at_line_start = true;

                if let Some(&'\n') = chars.peek() {
                    if consecutive_newlines >= 1 {  // we have at least one newline, next will be second
                        if !current_section.is_empty() {
                            result.push(current_section.to_string());
                            current_section.clear();
                        }
                        consecutive_newlines = 0;
                    }
                }
            },
            _ => {
                current_section.push(ch);
                at_line_start = false;
                consecutive_newlines = 0;
            }
        }
    }

    // add the final section if it exists
    if !current_section.is_empty() {
        result.push(current_section.to_string());
    }

    if result.is_empty() {
        vec![content.to_string()]
    } else {
        result
    }
}

/*

pub fn chunk_codeblock(content: String) -> Arc<String> {
    // extract metadata from
    return
}

pub fn chunk_summarized(content: String) -> Arc<String> {
    // use LLM to extract useful information from the text
}


*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_semantic() {
        let content = "This is a test! Next element, third case\nFinal";
        assert_eq!(parse_semantic(content), [
            "This is a test!", " Next element,", " third case\n", "Final"
        ]);
    }

    #[test]
    fn test_parse_sections() {
        let content = "### Title\nContent\n##Heading\n";
        assert_eq!(parse_sections(content), ["### Title\nContent\n",
        "##Heading\n"]);
    }
}
