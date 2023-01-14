mod symbol;use symbol::*;
mod pre;use pre::*;
mod inp;use inp::*;
mod print; use print::*;
mod cursor; use cursor::*;
mod state;use state::*;
mod file;use file::*;
fn main() {
    let k = random_key(3,3,3,3,7);
    //print_key(&k);
    print!("{}\n", key_as_string(&k, &"O|@?".to_string()));
    write_key(&k, &"O|@?".to_string());
}
