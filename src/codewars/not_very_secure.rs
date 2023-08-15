// In this example you have to validate if a user input string is alphanumeric. 
// The given string is not nil/null/NULL/None, so you don't have to check that.

// The string has the following conditions to be alphanumeric:

// - At least one character ("" is not valid)
// - Allowed characters are uppercase / lowercase latin letters and digits from 0 to 9
// - No whitespaces / underscore

fn alphanumeric(password: &str) -> bool {
    // At least one character ("" is not valid)
    if (password.len() == 0) {
        return false;
    }
    for c in password.chars(){
        let a = c as u32;
        // If not numbers and if not lowercased characters and if not uppercased characters...
        // https://theasciicode.com.ar/
        if (!(a >= 48 && a <= 57)) && (!(a >= 65 && a <= 90)) && (!(a >= 97 && a <= 122)) {
            return false;
        }
    }
    return true;
}

fn do_test(s: &str, expected: bool) {
    let actual = alphanumeric(s);
    assert_eq!(actual, expected, "\nInput: {s:?}\nYour result (left) did not match the expected output (right)")
}

fn main(){
    do_test("hello world_", false);
    do_test("PassW0rd", true);
    do_test("     ", false);
}
