// Usually when you buy something, you're asked whether your credit card number,
// phone number or answer to your most secret question is still correct.
// However, since someone could look over your shoulder, you don't want that shown on your screen.
// Instead, we mask it.

// Your task is to write a function maskify, which changes all but the last four characters into '#'.
// Examples (input --> output):
// "4556364607935616" --> "############5616"
//      "64607935616" -->      "#######5616"
//                "1" -->                "1"
//                 "" -->                 ""
// "Skippy" --> "##ippy"
// "Nananananananananananananananana Batman!" --> "####################################man!"

fn maskify(cc: &str) -> String {
    let input = cc;
    let mut output = String::from("");

    if input.len() <= 4 {
        output = input.to_string();
    } else {
        for (i, c) in input.chars().enumerate() {
            if i < input.len() - 4 {
                output.push('#');
            } else {
                output.push(c);
            }
        }
    }

    return output;
}

fn main(){
    assert_eq!(maskify("4556364607935616"), "############5616");
    assert_eq!(maskify("1"), "1");
    assert_eq!(maskify("11111"), "#1111");
}