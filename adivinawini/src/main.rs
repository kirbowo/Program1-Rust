use std::io;

struct AdivinaJuego {
    barajas: Vec<Vec<u32>>,
    resultado: u32,
}

impl AdivinaJuego {
    fn nuevo(numero: u32) -> Self {
        AdivinaJuego {
            barajas: Self::crear_barajas(numero),
            resultado: 0,
        }
    }

    fn crear_barajas(_numero: u32) -> Vec<Vec<u32>> {
        let mut barajas = Vec::new();
        for i in 0..7 {
            let mut carta = Vec::new();
            for j in 0..100 {
                if (j >> i) & 1 == 1 {
                    carta.push(j);
                }
            }
            barajas.push(carta);
        }
        barajas
    }

    fn leer_linea() -> String {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string()
    }

    fn jugar(&mut self) {
        for (i, baraja) in self.barajas.iter().enumerate() {
            println!("\n┌─────────────┐");
            println!("│  BARAJA {}   │", i + 1);
            println!("└─────────────┘");
            println!("┌─────────────────┐");
            for chunk in baraja.chunks(4) {
                let linea = chunk.iter().map(|n| format!("{:3}", n)).collect::<Vec<_>>().join(" ");
                println!("│ {:<15} │", linea);
            }
            println!("└─────────────────┘");
            println!("¿Está su número en esta baraja? (s/n)");
            
            loop {
                let respuesta = Self::leer_linea().to_lowercase();
                if respuesta == "s" {
                    self.resultado += 1 << i;
                    break;
                } else if respuesta == "n" {
                    break;
                } else {
                    println!("Por favor, responde solo 's' o 'n'.");
                }
            }
        }
    }

    fn mostrar_resultado(&self) {
        if self.resultado >= 0 && self.resultado <= 99 {
            println!("║   Su número es: {}              ║", self.resultado);
        } else {
            println!("║   ¡Parece que hubo un error!    ║");
            println!("║   El resultado está fuera de     ║");
            println!("║   rango (0-99). Intente de nuevo.║");
        }
    }
}

fn main() {
    println!("\n╔═══════════════════════════════════╗");
    println!("║   JUEGO DE ADIVINANZA BINARIA    ║");
    println!("╚═══════════════════════════════════╝\n");
    
    println!("   • Piensa en un número del 0 al 99");
    println!("   • Responde 's' (sí) o 'n' (no) a cada baraja\n");
    
    let mut juego = AdivinaJuego::nuevo(100);
    juego.jugar();
    
    println!("\n╔═══════════════════════════════════╗");
    juego.mostrar_resultado();
    println!("╚═══════════════════════════════════╝\n");
}
