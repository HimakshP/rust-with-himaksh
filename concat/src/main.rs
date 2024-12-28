fn main() {
    let string1=String::from ("Hi");
    let string2=String::from ("world");
    let concatenated_string= concatenate_strings(&string1 , &string2);
    println!("the concatenated string is {}",concatenated_string);

}
fn concatenate_strings(s: &String,s2: &String)-> String {
    let mut scloned=s.clone();
    scloned.push_str(s2);
    let result=scloned;
    result

}