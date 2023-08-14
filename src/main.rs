fn main() {
    let x: &str = "abc";
    let y: &str = "xs";

    if x.ends_with(y){
        dbg!("true");
    }
    else {
        dbg!("false");
    }
}
