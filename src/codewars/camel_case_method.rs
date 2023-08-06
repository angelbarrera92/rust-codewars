// Write a method (or function, depending on the language) that converts a string to camelCase, that is,
// all words must have their first letter capitalized and spaces must be removed.

// Examples (input --> output):

// "hello case" --> "HelloCase"
// "camel case word" --> "CamelCaseWord"


fn camel_case(str: &str) -> String {
    let mut r = String::from("");
    let mut upper_next_char = false;

    // Iterate over the trimmed input string
    for (i, c) in str.trim().chars().enumerate() {
        // Uppercase the 1st letter
        if i == 0 {
            r.push(c.to_ascii_uppercase());
        } else {
            // Check for spaces to mark the next character to be upperized
            if c == ' ' {
                upper_next_char = true;
            }else{
                // If it is not a character, check if it needs to be upperized
                if upper_next_char {
                    r.push(c.to_ascii_uppercase());
                    upper_next_char = false;
                }else{
                    r.push(c);
                }
            }
        }
    }

    return r;
}

fn main(){
    assert_eq!(camel_case("test case"), "TestCase");
    assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
    assert_eq!(camel_case("say hello "), "SayHello");
    assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
    assert_eq!(camel_case(""), "");
}