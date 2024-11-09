use std::{iter::Sum, result};

use lazy_static::lazy_static;
use regex::{Regex, Split};
use slint::{include_modules, SharedString};

include_modules!();

lazy_static! {
    static ref VALID_EXPRESSION: Regex = Regex::new(r"(\+|-|\*|\/)[0-9]+").unwrap();
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    // TASK: Adaugă logica pentru `on_add_to_text_area`, pentru a manevra cazurile "C", "=", și alte input-uri
    ui.on_add_to_text_area(move |current_text, new_input| {
        let ui = ui_handle.unwrap();
        println!("{:?}",current_text);

        // TASK: Adaugă logica pentru cazurile:
        // - "C": Golirea zonei de text
        // - "=": Calcularea rezultatului și afișarea acestuia
        // - Altele: Adăugarea input-ului curent la zona de text
        // HINT: Folosește un `match` pentru a verifica valoarea `new_input`.
        match new_input .as_str(){
            "C" => {
                ui.global::<MainText>().set_textarea(SharedString::from(""));
            }
            "=" => {
                let result: String = compute(&current_text).unwrap().to_string();
                ui.global::<MainText>().set_textarea(SharedString::from(result));
            }
            _ => {
                println!("{new_input}");
                ui.global::<MainText>()
                    
                    .set_textarea(current_text + new_input.as_str());
            }
        }
    });

    ui.run()
}

// TASK: Completează funcția `evaluate`, care verifică validitatea expresiei și returnează rezultatul sau un mesaj de eroare
// HINT: Folosește regex-ul `VALID_EXPRESSION` pentru a verifica dacă `input` este o expresie validă.
// Dacă expresia este validă, apelează funcția `compute`. Dacă nu, returnează un mesaj de eroare, cum ar fi "Invalid Expression".
fn evaluate(input: &str) -> String {
    //todo!() // <-- Înlocuiește `todo!()` cu implementarea funcției
    if VALID_EXPRESSION.is_match(input) {
        let fin = compute(input);
        let fin_str = fin.unwrap().to_string();
        return fin_str;
    } else {
        return String::from("Invalid expression");
    }
}

// TASK: Implementează funcția `compute` pentru a realiza operațiile de bază (+, -, *, /) și a returna rezultatul
// HINT: Parcurge simbolurile de operare și folosește `.split()` pentru a împărți `input` în două părți: înainte și după simbol.
// Convertește fiecare parte în `f64` și returnează rezultatul în funcție de simbol.
fn compute(input: &str) -> Option<f64> {
    // TASK: Inițializează simbolurile de operare (+, -, *, /)
    // HINT: Creează o listă `let symbols = ["+", "-", "*", "/"];
    const SYMBOLS: [&str; 4] = ["+", "-", "*", "/"];
    for symbol in SYMBOLS {
        if input.contains(symbol) {
            let operation = symbol;
            let words: Vec<&str> = input.split(symbol).collect();

            let x_str = words[0];
            let y_str = words[1];

            let x :f64 = x_str.parse().unwrap();
            let y :f64 = y_str.parse().unwrap();

            if operation == "+" {
                return Some(x + y);
            } else if operation == "-" {
                return Some(x - y);
            } else if operation == "*" {
                return Some(x * y);
            } else {
                return Some(x / y);
            }

        }

        return None;
    }

    None // <-- Returnează None dacă nu găsește niciun simbol valid
}
