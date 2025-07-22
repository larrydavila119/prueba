use barcoders::generators::image::*;
use barcoders::sym::code128::Code128;
use std::fs::File;
use std::io::Write;

fn generar_codigo_barra(archivo: &str, codigo: &str) {
    println!("Código proporcionado: {}", codigo);

    // Crear el código de barras en formato Code 128
    let code128 = Code128::new(codigo).expect("Error al crear el código de barras.");
    
    // Convertir el código de barras a bits
    let encoded = code128.encode();

    // Configuración del generador de imágenes
    let generator = Image::png(80); // Ajustar resolución
    let bytes = generator.generate(&encoded).expect("Error al generar el código de barras.");

    // Guardar como archivo PNG
    let mut file = File::create(archivo).expect("No se pudo crear el archivo.");
    file.write_all(&bytes).expect("No se pudo escribir en el archivo.");

    println!("Código de barras guardado en: {}", archivo);
}

fn main() {
    let archivo = "codigo_barra.png";
    let codigo = "1234567890123456"; // Código estático de 16 dígitos
    generar_codigo_barra(archivo, codigo);
}

