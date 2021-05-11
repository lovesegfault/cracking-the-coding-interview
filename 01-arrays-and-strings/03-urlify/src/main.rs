const URL_WHITESPACE: &str = "%20";

fn urlify(s: &str) -> String {
    s.trim().replace(char::is_whitespace, URL_WHITESPACE)
}

fn urlify_in_place(v: &mut Vec<char>) {
    // first we must resize the vec to make sure we can fit the replacements
    let original_len = v.len();
    let space_count = v.iter().filter(|c| c.is_whitespace()).count();
    let extra_space_needed = space_count * (URL_WHITESPACE.len() - 1);
    let needed_len = original_len + extra_space_needed;
    if original_len < needed_len {
        v.resize(needed_len, ' ');
    }

    // now we replace " " with %20

    let mut idx = v.len();
    for i in (0..original_len).rev() {
        if v[i].is_whitespace() {
            v[idx - 1] = '0';
            v[idx - 2] = '2';
            v[idx - 3] = '%';
            idx -= 3;
        } else {
            v[idx - 1] = v[i];
            idx -= 1;
        }
    }
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

    print!("checking urlify_in_place: ");
    let mut a = "Mr John Smith".chars().collect::<Vec<_>>();
    let mut b = "".chars().collect::<Vec<_>>();
    let mut d = "tab ->	<- tab".chars().collect::<Vec<_>>();
    urlify_in_place(&mut a);
    urlify_in_place(&mut b);
    urlify_in_place(&mut d);
    assert_eq!(a, "Mr%20John%20Smith".chars().collect::<Vec<_>>());
    assert_eq!(b, "".chars().collect::<Vec<_>>());
    assert_eq!(d, "tab%20->%20<-%20tab".chars().collect::<Vec<_>>());
    println!("OK");
}
