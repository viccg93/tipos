fn main() {
    //Rust es un lenguaje estaticamente tipado, lo que quiere decir que el compilador
    //debe de conocer los tipos en tiempo de compilacion, y la inferencia se realiza
    //precisamente en la compilacion y no en la ejecicion.
    //En Rust tenemos dos clasificaciones de tipos: Escalares y compuestos.

    //Rust tiene 4 tipos escalares

    //Integers (u = unsugned, i = signed)
    let numero_8bits: u8 = 8;
    let numero_16bits: u16 = 16;
    let numero_32bits: u32 = 32;
    let numero_64bits: u64 = 64;
    let numero_128bits: u128 = 128;
    let numero_archbits: usize = 64;

    //se pueden usar prefijos para los numeros como el separador para mayor claridad como _,x,o,b,b'A'
    //son equivalentes, en ambos casos se asigna el numero 1000
    let x: u32 = 1000;
    let x: u32 = 1_000;

    //hex
    let hex = 0xff;
    //octal
    let hex = 0o77;
    //binario
    let hex = 0b0000_1111;

    //Hay que considerar la posibilidad de un overflow, cuando esto sucede en debug sucede un panic
    //cuando sucede en release se usa un Wrapper que cambia al valor minimo del tipo.
    //Se recomienda no confiar en ese comportamiento ya que nos lleva a obtener valores y comportamientos
    //no deseados.
    //se recomienda siempre usar wrapping_* en conjunto con None, checked_*, overflowing_* y saturating_* para controlar
    //los casos de size overflow.

    //punto flotante
    //Rust admite numeros de punto flotante de 32 y 64 bits, el tipo por defecto es 64
    //la representacion de los numeros de punto flotante es de acuerdo al estandard IEEE-754, con doble precision en el caso de f64.
    let numero_punto_flotante = 2.0; //f64
    let otro_numero_punto_flotante: f32 = 3.0; //f32

    //Operaciones algebraicas
    //Rust admite 5 operaciones basicas
    //En el caso de la division de enteros, se redondea al numero entero mas cercano.

    let suma = 2 + 1;
    let resta = 2 - 1;
    let multiplicacion = 2*1;
    let division = 2/1;
    let modulo = 2%1;

    //bool

    let es_verdadero = true; 
    let es_falso: bool = false;

    //char
    //es el tipo de representacion alphabetica mas primitivo en Rust
    //representa un valor escalar unicode, lo que implica su capacidad
    //para extender su capacidad mas alla de ASCII, permitiendo acentuacion
    //chino, coreano, japones y emojis.

    let a = 'a';
    let b: char = 'b'; //anotacion explicita
    let emoji = '😻';

    //tipos compuestos

    //existen dos tipos compuestos es Rust: tuple y array.
    //los tipos compuestos son lo que almacenan mas de un tipo en ellos

    //tuple
    //este tipo permite almacenar varios valores que pueden ser de distintos tipos.
    //una vez definido su tamaño no se puede ampliar o hacer mas pequeña.

    //definicion de una variable tupla con tres valores.
    let tup = (10,"hola mundo",'😀');

    //definicion de una variable tupla con anotaciones de tipo (opcionales)
    let tup2: (u8, f64, char) = (10,2.0,'😀');

    //Para obtener los valores de una tupla, podemos acceder a ellos mediante su indice (inicia en 0)
    let value: f64 = tup2.1;

    println!("el segundo valor de tup2 es {value}");

    //tambien se puede hacer una deconstruccion de la tupla en variables

    let (x,y,z) = tup;

    println!("los valores de la tupla son {x}, {y}, {z}");

    //Arrays

    //En Rust los arrays son igualmente colecciones de datos (seccion en memoria) del mismo tipo
    //El tamaño del array tambien es fija una vez que se declara el array

    let dummy_array = [1,2,3,4];

    //para acceder al array debemos usar su indice

    let dummy_value = dummy_array[0];

    //para definir el array podemos hacerlo con el tipo explicito y el tamaño en la definicion

    let dummy_array_2: [u8;5] = [1,2,3,4,5];

    //tambien podemos usar un valor inicial que se repita n veces

    let dummy_array_3 = [4;5]; // equivalente a [4,4,4,4,4]

    //Al acceder a los elementos de un array por indice
    //Se puede dar el escenario donde se accede a un indice mas grande de los que existen
    //en el array, en ese caso se lanza un panic indicando un "index out of bounds"
    //este comportamiento es seguro y evita acceder a otras localidades de memoria
    //ademas de que detiene la ejecucion (contrario a otros lenguajes).
}
