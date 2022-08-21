use std::borrow::Cow;

fn main() {
    let s = remove_spaces("Herman Radtke");
    
    println!("Length of string is {}", s.len());
}


fn remove_spaces<'a>(input: &'a str) -> Cow<'a, str>{
    if input.contains(' ') {
        // return Cow::Owned(buf);
        input.chars()
            .filter(|&x| x != ' ')
            .collect::<String>()
            .into()
    } else {
        // return Cow::Borrowed(input);
        input.into()
    }
}