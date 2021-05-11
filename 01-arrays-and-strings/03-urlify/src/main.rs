const URL_WHITESPACE: &str = "%20";

fn urlify(s: &str) -> String {
    s.trim().replace(char::is_whitespace, URL_WHITESPACE)
}

fn main() {
    print!("checking urlify: ");
    let a = "Mr John Smith     ";
    let b = "                  ";
    let c = "I am a tab ->	";
    let d = "tab ->	<- tab";
    assert_eq!(urlify(a), "Mr%20John%20Smith");
    assert_eq!(urlify(b), "");
    assert_eq!(urlify(c), "I%20am%20a%20tab%20->");
    assert_eq!(urlify(d), "tab%20->%20<-%20tab");
    println!("OK");
}
