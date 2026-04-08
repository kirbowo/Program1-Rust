use std::io::{self, Write};
const N: usize = 100; //constante para el tamaño maximo de la cadena
struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}
impl Cadena {
    fn new() -> Cadena {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }
}

fn leer_cadena() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
fn main() {
    
 }
