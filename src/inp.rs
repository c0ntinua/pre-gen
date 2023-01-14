use crate::symbol::*;
use rand::Rng;

pub fn random_input_code(range : u8, max_length : usize, num_symbols : usize) -> Code {
    debug_assert!(num_symbols > 1);
    let mut finished = false;
    let extra_symbols = num_symbols - 2;
    let mut rng = rand::thread_rng();
    let mut code = vec!();
    while extra_symbols > 0 && !finished {
        code = vec!();
        for _ in 0..extra_symbols {
            let length = rng.gen_range(2..=max_length);
            code.push(random_symbol(range, length));
        }
        finished = if !symbol_appears_twice_in(&code){ true} else {false};
    }
    code.push(vec![1u8]);
    code.push(vec![0u8]);
    code
}

pub fn symbol_appears_twice_in(code : &Code) -> bool {
    for i in 0..code.len() {
        for j in 0..code.len() {
            if i != j && code[i].starts_with(&code[j]) {
                return true;
            }
        }
    }
    false
}

// pub fn random_long_code(range : u8, length : usize, num_symbols : usize) -> Code {
//     let mut rng = rand::thread_rng();
//     let mut code = vec!();
//     for _ in 0..num_symbols {
//         let length = rng.gen_range(1..=max_length);
//         code.push(random_symbol(range, length));
//     }
//     code
// }

// func inp(num, max int) []string {
// 	S := make([]string, num)
// 	for i:= 0 ; i < num - len(alphabet); i++ {
// 		w := word(rand.Intn(max-1) + 2)
// 		for ;_word_in(w,S); { w = word(rand.Intn(max-1) + 2 ) }
// 		S[i] = w
// 	}
// 	for i := 0; i < len(alphabet) ; i++ {
// 		S[num - len(alphabet) - i] = string(alphabet[i])
// 	}
// 	return S
// }