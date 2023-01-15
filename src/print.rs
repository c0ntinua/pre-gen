use crate::symbol::*;
use crate::cursor::*;
use crate::state::*;
use crate::file::*;



pub fn print_response(r : &Response, alphabet : &str) {
    let b = 100;
    set_color(b,b,b);
    print!(" <");
    set_color(255,0,0);
    print!(" {}", &symbol_as_string(&r.read,alphabet));
    set_color(255,255,0);
    print!(" {} ", &symbol_as_string(&r.write,alphabet));
    set_color(0,255,0);
    print!("{} ",&r.go);
    set_color(b,b,b);
    print!("> ");
    set_color(255,255,255);
}
pub fn print_state(s : &State, alphabet : &str) {
    for r in s {
        print_response(r, alphabet);
    }
}
pub fn print_key(k : &Key, alphabet : &str) {
    for s in k {
        print_state(s, alphabet);
        print!("\n");
    }
}
pub fn symbol_as_string(symbol : &Symbol, alphabet : &str) -> String {
    let mut string = "".to_string();
    for i in symbol {
        let j = *i as usize;
        string.push(alphabet.chars().nth(j).unwrap());
    }
    string
}


