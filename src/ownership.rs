pub fn ownership () {
    //Cuando se va a asignar una variable a otra variable en otra se clona el contenido. Pero ello 
    //depende del tipo de contenido, ya que ello solo pasa con variables de tipo int, bool, float, char y tuplas
    //que contienen variables de los tipos antes dichos. Porque son variables que no ocupan mucho espacio
    // en memoria, asi que tener copia de estas variables no afectaria tanto al rendimiento.Este tipo de 
    //variables implementan el trait Copy, por ello se puede copiar.

    let x = String::from("hola");
    // print_variable(x.clone());
    // println!("{:p}", &x);
    let mut y: f32 = 5.0;
    println!("{:p}", &y);
    print_number(&mut y);
    println!("{:p}", &y);
}

fn print_number(var: &mut f32) {
    // var = var + 1.5;
    println!("{:p}", &var);
    *var = 555.22;
    println!("{}", var);
    println!("{:p}", &var);
}

fn print_variable(var: String) {
    // var.
    println!("{:p}", &var);
}
