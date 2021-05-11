const URL_WHITESPACE: &str = "%20";

fn urlify(s: &str) -> String {
    s.trim().replace(char::is_whitespace, URL_WHITESPACE)
}

fn urlify_in_place(v: &mut Vec<char>) {
    // first we must resize the vec to make sure we can fit the replacements
    let trailing_whitespace_count = v.iter().rev().take_while(|c| c.is_whitespace()).count();
    let string_text_len = v.len() - trailing_whitespace_count;
    let space_count = v[0..string_text_len]
        .iter()
        .filter(|c| c.is_whitespace())
        .count();
    let extra_space_needed = space_count * (URL_WHITESPACE.len() - 1);
    let total_space_needed = string_text_len + extra_space_needed;
    v.resize(total_space_needed, ' ');

    // now we replace " " with %20
    let mut idx = v.len();
    for i in (0..string_text_len).rev() {
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
    let a = "Mr John Smith      ";
    let a_url = "Mr%20John%20Smith";
    let b = "                  ";
    let b_url = "";
    let c = "I am a tab ->	";
    let c_url = "I%20am%20a%20tab%20->";
    let d = "tab ->	<- tab";
    let d_url = "tab%20->%20<-%20tab";

    print!("checking urlify: ");
    assert_eq!(urlify(a), a_url);
    assert_eq!(urlify(b), b_url);
    assert_eq!(urlify(c), c_url);
    assert_eq!(urlify(d), d_url);
    println!("OK");

    print!("checking urlify_in_place: ");
    let mut a = a.chars().collect::<Vec<_>>();
    let a_url = a_url.chars().collect::<Vec<_>>();
    let mut b = b.chars().collect::<Vec<_>>();
    let b_url = b_url.chars().collect::<Vec<_>>();
    let mut c = c.chars().collect::<Vec<_>>();
    let c_url = c_url.chars().collect::<Vec<_>>();
    let mut d = d.chars().collect::<Vec<_>>();
    let d_url = d_url.chars().collect::<Vec<_>>();
    urlify_in_place(&mut a);
    urlify_in_place(&mut b);
    urlify_in_place(&mut c);
    urlify_in_place(&mut d);
    assert_eq!(a, a_url);
    assert_eq!(b, b_url);
    assert_eq!(c, c_url);
    assert_eq!(d, d_url);
    println!("OK");
}
