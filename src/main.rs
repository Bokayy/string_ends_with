fn main() {
    let x: &str = "abc";
    let y: &str = "bc";
    let mut x_as_array: Vec<char> = Vec::new();
    let mut y_as_array: Vec<char> = Vec::new();
    let mut flag_true = false;

    for character in x.chars(){
        x_as_array.push(character);
    }
    dbg!(x_as_array);

    for character in y.chars(){
        y_as_array.push(character);
    }
    dbg!(y_as_array);

    for (index, character) in y.char_indices(){
        if (&character == &x_as_array[index]){
            flag_true = true;
        }
        else{
        flag_true = false;}
    }

    dbg!(flag_true);
}
