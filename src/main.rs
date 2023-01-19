
use std::io;
fn main() {

//Necesitamos que numero sea un string mutable porque va capturar una cadena de teclado    
let mut numero: String=String::new();
let _tope: u16=100;

println!("Digita un numero");
//read_line retorna un Result que es un enum que contiene dos variant
// Ok y Err Ese enum lo obtenemos en la variable result
let result=io::stdin().read_line(&mut numero);

let numero1: u16;

//Ahora le daremos manejo a los datos que retornó result
match result {
    Ok(nro_bytes)=>println!("Numero de bytes capturados {nro_bytes}"),
    Err(error)=>println!("Ocurrio este error {}",error)
}

println!("El número 1 ingresado es {}",numero);

numero1=numero.trim().parse().expect("Debes escribir un numero");

println!("Digita un segundo numero");

    



let _vacio:&str="";

//Asignamos a la variable numero un string vacio, porque sino el numero que digiten
//se concatenara con el anterior porque usaremos la misma variable

numero=String::from(_vacio);

//Esta vez simplificaremos el manejo del Result y simplemente en caso de que ocurra un error
//Lo mostraremos con .expect
io::stdin().read_line(&mut numero).ok().expect("Error al leer teclado");

println!("El número 2 ingresado es {}",numero.trim());

let numero: u16=numero.trim().parse().expect("Debes escribir un numero");

let producto: u16;
producto=numero1*numero;

    if numero1>numero
    {
        println!("El numero {} es mayor que {}",numero1,numero);
    }    
    else
    {
        println!("El numero {} es mayor que {}",numero,numero1);
    }
    println!("El producto es {}",producto);


}
