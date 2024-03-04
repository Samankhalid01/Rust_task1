
fn main() {
    let string1 = String::from("Saman, ");
    let string2 = String::from("Khalid!");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("{}", concatenated_string);
}
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::from(s1);
    result.push_str(s2);
    result
}
