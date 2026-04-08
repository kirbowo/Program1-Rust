use std::io::{self, Write};

use colored::Colorize;
fn main() {
    let mut fun: String;
    let mut ex: String;
    loop{
        println!("{} {} {} {} {}","Ingrese la función a ejecutar:","(semana, fibonacci, mes)".yellow(),"o","'exit'".red(),"para salir:");    
       (ex,fun) = (String::new(),String::new());
        std::io::stdin().read_line(&mut fun).expect("semana");
    if  fun .trim().to_lowercase() == "semana" {
        println!("{}","Ejecutando la función semana...".yellow().bold());
    loop {
        semana();
        println!("{} {} {}"," Presione".white(), "Enter".color("#808080").bold().blink(), "para continuar o escriba cualquier texto para salir...".white());
        std::io::stdin().read_line(&mut ex).expect("");
        if !ex.trim().is_empty()  {
            break;
            }
        }    
    }

    else if (fun.trim().to_lowercase() == "fibonacci") || (fun.trim().to_lowercase() == "fib") {
        println!("{}","Ejecutando la función fibonacci...".yellow().bold());
    loop {
        fibonacci();
        println!("{} {} {}"," Presione".white(), "Enter".color("#808080").bold().blink(), "para continuar o escriba cualquier texto para salir...".white());        
        std::io::stdin().read_line(&mut ex).expect("");
        if !ex.trim().is_empty()  {
            break;
            }
        }    
    }
    else if fun.trim().to_lowercase() == "mes" {
        println!("{}","Ejecutando la función mes...".yellow().bold());
    loop {
        mesdia();
        println!("{} {} {}"," Presione".white(), "Enter".color("#808080").bold().blink(), "para continuar o escriba cualquier texto para salir...".white());
        std::io::stdin().read_line(&mut ex).expect("");
        if !ex.trim().is_empty()  {
            break;
    }
}
    }
    else {
        println!("Función no reconocida. Por favor, ingrese 'semana', 'fibonacci' o 'mes'.");
}
if fun.trim().to_lowercase() == "exit" {
    println!("Saliendo del programa...");
    break;}
}
}
fn semana(){
 //1.- función que dado un día de la semana, devuelva si es laboral o no (lunes - viernes)
    let mut dia = String::new();{    
    print!("{} {}","Ingrese un día de la semana:".cyan().underline()," ");   
    io::stdout().flush().unwrap(); 
    std::io::stdin().read_line(&mut dia).expect("Error al leer la entrada");
    let dia = dia.trim().to_lowercase();
    match dia.as_str() {
        "lunes" | "martes" | "miércoles" |"miercoles"| "jueves" | "viernes" => println!("{} {}", dia.green().bold(), "sí es un día laboral.".green()),
        "sábado" |"sabado"| "domingo" => println!("{} {}", dia.red().bold(), "no es un día laboral.".red()),
        _ => println!("{}", format!("{} no es un día válido.", dia).red().bold().underline()),
        }
    }
}
fn fibonacci(){
//2.- función para que resuelva Fibonacci.
    let mut n = String::new();
    print!("{} {}","Ingrese el número de términos de Fibonacci a generar:".cyan().underline()," ");
     io::stdout().flush().unwrap(); 
    std::io::stdin().read_line(&mut n).expect("Error al leer la entrada");
    let n: usize = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor, ingrese un número válido.");
            return;
        }
    };
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    println!("Los primeros {} términos de Fibonacci son:", n);
    for _ in 0..n {
        print!("{}-", a.to_string().cyan().bold());
        let temp: i32 = a;
        a = b;
        b = temp + b;
    }
    println!();
}
fn mesdia(){
//3- usando match, realizar una función que dado el mes, devuelva la cantidad de días que tiene ese mes. Ejemplo: febrero (2) - 28
let mut mes = String::new();
print!("{} {}","Ingrese un mes(enero-diciembre) o escriba 'exit' para salir:".cyan().underline()," ");
io::stdout().flush().unwrap();
std::io::stdin().read_line(&mut mes).expect("Error al leer la entrada");
let mes = mes.trim().to_lowercase();
match mes.as_str() {
    "enero" | "marzo" | "mayo" | "julio" | "agosto" | "octubre" | "diciembre" => println!("{} {} {} {}", mes.yellow(),"tiene"," 31".bright_blue()," días."),
    "abril" | "junio" | "septiembre" | "noviembre" => println!("{} {} {} {}", mes.yellow(),"tiene"," 30".bright_blue()," días."),
    "febrero" => println!("{} {} {} {}", mes.yellow(),"tiene"," 28".bright_blue()," días (29 en años bisiestos)."),
    _ => println!("{}", format!("{} no es un mes válido.", mes).red().bold().underline()),
        }
    }
