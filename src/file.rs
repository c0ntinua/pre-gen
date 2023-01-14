use std::fs::*;
use std::io::Write;

use crate::symbol::*;
use crate::pre::*;
use crate::inp::*;
use crate::state::*;




pub fn file() -> File {
    File::create(format!("{:?}.txt", chrono::offset::Local::now())).unwrap()
}

pub fn write_key(k : &Key, alphabet : &str) {
    let mut file = file();
    let s = key_as_string(k,alphabet);
    writeln!(file, "{}", &s).unwrap();
}

pub fn symbol_as_string(symbol : &Symbol, alphabet : &str) -> String {
    let mut string = "".to_string();
    for i in symbol {
        let j = *i as usize;
        string.push(alphabet.chars().nth(j).unwrap());
    }
    string
}

pub fn response_as_string(r : &Response, alphabet : &str) ->String {
    format!("({},{},{})", 
        symbol_as_string(&r.read,alphabet),
        symbol_as_string(&r.write,alphabet),
        r.go
    )
}

pub fn state_as_string(s : &State, alphabet : &str) -> String {
    let mut string = "".to_string();
    //string.push('(');
    for i in 0..s.len() {
        string.push_str(&response_as_string(&s[i], alphabet));
    }
    //string.push(')');
    string
}

pub fn key_as_string(k :&Key, alphabet : &str) -> String {
    let mut string = "".to_string();
    for i in 0..k.len() {
        string.push_str(&state_as_string(&k[i], alphabet));
        if i != k.len() - 1 {
            string.push('\n');
        }
    }
    string
}



// pub fn key_as_string(key: &Key) -> String {
//     let mut string = "".to_string();
//     string
// }
// pub fn write_key(key : &Key) {

// }
// pub fn write_symbol(symbol : &Symbol) {
//     //print!(" ");
//     for i in symbol {
//         match i {
//             0 => print!("O"),
//             1 => print!("|"),
//             2 => print!("@"),
//             _ => print!("?"),
//         }
//     }
// }

// pub fn write_state(s : &State, file : &mut File) {
//     for i in 0..s.len() {
//         print!("(");
//         print_symbol_tight(&s[i].read);
//         print!(",");
//         set_color(255,255,0);
//         print_symbol_tight(&s[i].write);
//         set_color(255,255,255);
//         print!(",");
//         set_color(0,255,255);
//         print!("{}",&s[i].go);
//         set_color(255,255,255);
//         print!(")");
//         if i < s.len() - 1 {
//             print!(",");
//         }
//         //print!("    ");
//     }
//     set_color(255,255,255);
//     print!("]");
// }

// pub fn hands_FROM_filename(filename : &str) -> Vec<Hand> {
//     let string = read_to_string(filename).expect("no file!");
//     let mut hands = hands_FROM_string(&string);
//     hands
// }
// pub fn write_WITH_hand_filename(hand : &Hand, file : &mut File)  {
//     let s = string_FROM_hand(hand);
//     writeln!(file, "\n{}", &s).unwrap();
// }
// pub fn filename() -> String {
//     format!("{:?}.txt", chrono::offset::Local::now())
// }

// pub fn write_WITH_hands(hands : &Vec<Hand>) {
//     let mut file = file();
//     for hand in hands.iter() {write_WITH_hand_filename(hand, &mut file);}
// }
   