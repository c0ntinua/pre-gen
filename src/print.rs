use crate::symbol::*;
use crate::cursor::*;
use crate::state::*;

pub fn print_symbol(symbol : &Symbol) {
    print!("  ");
    for i in symbol {
        match i {
            0 => print!("O"),
            1 => print!("|"),
            2 => print!("@"),
            _ => print!("?"),
        }
    }
    print!("  ");
}
pub fn print_symbol_tight(symbol : &Symbol) {
    //print!(" ");
    for i in symbol {
        match i {
            0 => print!("O"),
            1 => print!("|"),
            2 => print!("@"),
            _ => print!("?"),
        }
    }
}

pub fn print_code(code : &Code) {
    set_color(255,0,0);
    print!("(");
    set_color(255,255,0);
    for s in code {
        print_symbol(s);
    }
    set_color(255,0,0);
    print!(")");
    print!("\n");
    set_color(255,255,255);
}

pub fn print_state(state : &State) {
    set_color(255,255,255);
    print!("(");
    for r in state {
        set_color(255,0,0);
        print_symbol_tight(&r.read);
        set_color(255,255,0);
        
        print_symbol_tight(&r.write);
        
        set_color(0,255,255);
        print!("{:3}",r.go);
        print!("    ");
    }
    set_color(255,255,255);
    print!(")");
}
pub fn print_state_tight(s : &State) {
    set_color(255,255,255);
    print!("[");
    for i in 0..s.len() {
        set_color(255,255,255);
        print!("(");
        set_color(255,0,0);
        print_symbol_tight(&s[i].read);
        set_color(255,255,255);
        print!(",");
        set_color(255,255,0);
        print_symbol_tight(&s[i].write);
        set_color(255,255,255);
        print!(",");
        set_color(0,255,255);
        print!("{}",&s[i].go);
        set_color(255,255,255);
        print!(")");
        if i < s.len() - 1 {
            print!(",");
        }
        //print!("    ");
    }
    set_color(255,255,255);
    print!("]");
}

pub fn print_key(key : &Key) {
    for s in key {
        print_state_tight(&s);
        print!("\n");
    }
}