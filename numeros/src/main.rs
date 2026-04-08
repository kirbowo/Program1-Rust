//use core::num;
use std::io::{self, Write};  // Para leer entrada y mostrar el menú sin salto de línea
struct Numero {
    valor: u64
}

impl Numero {
    fn new(valor: u64) -> Self {
        Numero { valor }
    }

    fn es_par(&self) -> bool {
        self.valor % 2 == 0
    }

    fn es_primo(&self) -> bool {
        let num: u64 = self.valor;

        if num < 2 {
            return false;
        }
        if num == 2 {
           return true;
        }

        if num % 2 == 0 {
            return false;
        }

        let t: u64 = (num as f64).sqrt() as u64 + 1;
        let mut d: u64 = 3;

        while d <= t {
            if num % d == 0 {
                return false;
            }
            d += 2;
        };

        true
    }

    fn cantidad_digitos(&self) -> u64 {
        let mut count: u64 = 0;
        let mut num: u64 = self.valor;

        while num > 0 {
            num /= 10;
            count += 1;
        }

        count
    }

    fn invertir(&self) -> u64 {
        let mut num: u64 = self.valor;
        let mut invertido: u64 = 0;

        while num > 0 {
            let digito: u64 = num % 10;
            invertido = invertido * 10 + digito;
            num /= 10;
        }

        invertido
    }

    fn es_capicua(&self) -> bool {
        self.valor == self.invertir()
    }

    fn elevado(&self, base: u64, exp: u64) -> u64 {
        let mut conta = 1;
        for _ in 0..exp {
            conta *= base;
        }
        conta
    }

    fn es_armstrong(&self) -> bool {
        let mut num = self.valor;
        let digitos = self.cantidad_digitos();
        let mut suma = 0;

        while num > 0 {
            let digito = num % 10;
            suma += self.elevado(digito, digitos as u64);
            num /= 10;
        }
        suma  == self.valor
    }
    fn cant_dig_par(&self) -> u32 {
        let mut count = 0;
        let mut num = self.valor;

        while num > 0 {
            let digito = num % 10;
            if digito % 2 == 0 {
                count += 1;
            }
            num /= 10;
        }

        count
    }
    fn raiz_digital(&self) -> u64 {
        //La raiz digital, es aquel que sumando sus digitos, se obtiene un nuevo valor.
        //este valor tambien se debe sumar sus digitos, hasta que el valor sea de 1 cifra.
        let mut n = self.valor;
        while n >= 10 {
            let mut suma = 0;
            let mut temp = n;
            while temp > 0 {
                suma = suma + (temp % 10);
                temp = temp / 10;
            }
            n = suma;
        }
        return n;
    }
    //1.- Collatz: Si el número es par, se divide entre 2; si es impar, se multiplica por 3 y se suma 1.
    //    Repetir hasta llegar a 1. 
    //    Mostrar los pasos necesarios y otro que encuentre el valor maximo alcanzado.

    fn conjetura_collats(&self) -> u32 {
        let mut cont: u32 = 0;
        let mut num : u64 = self.valor;

        while num != 1 {
            if num % 2 == 0 {
                num /= 2;
            }
            else {
                num = num * 3 + 1;
            }
            cont += 1;
            }
            cont
        }
    fn valor_max_collatz(&self) -> u64 {
        let mut num : u64 = self.valor;
        let mut max: u64 = 0;

        while num != 1 {
            if num % 2 == 0 {
                num /= 2;
            }
            else {
                num = num * 3 + 1;
            }
            if num > max {
                max = num;
            }
        }
        max
    }
//      13 es impar → 13*3+1=40
//      40 es par → 40/2=20
//      20 es par → 20/2=10
//      10 es par → 10/2=5
//      5 es impar → 5*3+1=16
//      16 es par → 16/2=8
//      8 es par → 8/2=4
//      4 es par → 4/2=2
//      2 es par → 2/2=1
//      Pasos: 9, Valor máximo: 40'
    fn leer_linea(&self) -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

//2.- Insertar un digito en una pos,ej:
//    361, quiero insertar el digito 2 en la 2da posicion
//    3261     

    fn insertar_digito(&self) -> u64 {
        let mut num: u64 = self.valor;
        let cant_dig: u64 = self.cantidad_digitos();
        println!("  Ingresa el dígito a insertar:");
        let digito = self.leer_linea().parse::<u64>().expect("Error al leer el digito");
        println!("  Ingresa la posición (0 para el final):");
        let pos : u64 = self.leer_linea().parse::<u64>().expect("Error al leer la posicion");
        let mut cont: u64 = 0;
        while pos > cont {
            num /= 10;
            cont += 1;
        }
        if cant_dig < pos {
            println!("  Posición inválida. El número tiene solo {} dígitos.", cant_dig);
            return self.valor;
        }
        else{
        let parte1 = num * 10 + digito;
        let parte2 = self.valor % self.elevado(10, pos);
        let resultado = parte1 * self.elevado(10, pos) + parte2;
        resultado
        }
    }
/*     fn fibonacci(&self) -> u64 {
        let mut a: u64 = 0;
        let mut b: u64 = 1;

        for _ in 0..self.valor {
            let temp: u64 = a;
            a = b;
            b = temp + b;
        }

        a
    }*/

    fn resetear(&mut self, nuevo: u64) {
        self.valor = nuevo;
    }
    //funcion que elimine un digito dada una posicion. empezando desde la izquierda.
    //4675 queremos eliminar el digito numero 3
    /**/
    //fn buscar_digito(&self)-> u64{

    //}
    //obtener digito: dada una posicion, devolver el digito
    //465, quiero obtener el digito en la posicion 2, devuelve 6
    /*1ro el usuario introduce una pos
    2do debemos saber cuantos digitos tiene el numero
    3do debemos llegar a ese digito
    4to devolvemos el digito encontrado*/
    fn obtener_digito(&self, posicion: u64) -> u64 {
        let digitos = self.cantidad_digitos();
        let medio = self.elevado( 10,  digitos - posicion);
        let resultado = (self.valor / medio) % 10;
        resultado
    }
}
// Función para leer una línea de entrada del usuario
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
// Función para leer un número u64 del usuario, devuelve None si no es válido
fn leer_numero() -> Option<u64> {
    leer_linea().parse::<u64>().ok()
}

fn mostrar_menu(n: &Numero) {
    println!("\n╔══════════════════════════════════╗");
    println!("║   NÚMERO ACTUAL: {:>14}  ║", n.valor);
    println!("╠══════════════════════════════════╣");
    println!("║  Consulta                        ║");
    println!("║  1. ¿Es par?                     ║");
    println!("║  2. ¿Es primo?                   ║");
    println!("║  3. Cantidad de dígitos          ║");
    println!("║  4. Invertir                     ║");    
    println!("║  5. ¿Es capicúa?                 ║");
    println!("║  6. ¿Es Armstrong?               ║");
    println!("║  7. Cantidad dígitos pares       ║");
    println!("║  8. Raíz digital                 ║");
    println!("║  9. Pasos Collatz                ║");
    println!("║ 10. Valor máximo Collatz         ║");
    println!("║ 11. Insertar dígito              ║");
    println!("║ 12. Obtener dígito               ║");
    println!("╠══════════════════════════════════╣");
    println!("║  0. Ingresar nuevo número        ║");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Números - POO — Programación I");
    println!("════════════════════════════════════");
    println!("Ingresa un número para comenzar:");
    // Validar que el usuario ingrese un número válido antes de crear la instancia de Numero
    let valor_inicial: u64 = loop { //loop se repite hasta que encuentre un break
        match leer_numero() {
            Some(num) => break num,
            None    => println!("Número inválido. Intenta de nuevo:"),
        }
    };
    //creando una instancia de Numero con el valor inicial ingresado por el usuario
    //ESTA ES LA INSTANCIA DEL OBJETO NUMERO, A PARTIR DE AQUÍ SE UTILIZARÁ PARA REALIZAR TODAS LAS CONSULTAS
    let mut n =  Numero::new(valor_inicial);

    loop { //el menu se mostrará en un bucle infinito hasta que el usuario decida salir usando la opción 'Q' (break)
        mostrar_menu(&n);
        let opcion = leer_linea();

        match opcion.as_str() {  //usando match, se puede llamar a la función correspondiente.
            // Consultas
            "1" => println!("  ¿Es par?          → {}", n.es_par()),
            "2" => println!("  ¿Es primo?        → {}", n.es_primo()),
            "3" => println!("  Cantidad Digitos: → {}", n.cantidad_digitos()),
            "4" => println!("  Invertir:         → {}", n.invertir()),
            "5" => println!("  ¿Es capicúa?      → {}", n.es_capicua()),
            "6" => println!("  ¿Es Armstrong?    → {}", n.es_armstrong()),
            "7" => println!("  Cant. Digitos Par → {}", n.cant_dig_par()),
            "8" => println!("  Raíz Digital:     → {}", n.raiz_digital()),
            "9" => println!("  Pasos Collatz:    → {}", n.conjetura_collats()),
            "10" => println!("  Valor Máx Collatz: → {}", n.valor_max_collatz()),
            "11" => println!("  Insertar dígito:  → {}", n.insertar_digito()),
            "12" => println!("  Obtener dígito:   → {}", n.obtener_digito(/*u64*/)),
            "0" => {
                println!("  Ingresa el nuevo número:");
                match leer_numero() {
                    Some(num) => { n.resetear(num); println!("  ✓ Nuevo número: {}", n.valor); }
                    None    => println!("  Número inválido, se mantiene {}", n.valor),
                }
            }
            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; } //aquí se rompe el ciclo con "q" o "Q"
            _ => println!("  Opción no válida."),
        }
    }
}