use regex::Regex;
use std::io::{self, Write};

fn main() {
    // Solicitar ao usuário o código hexadecimal
    println!("Digite o código hexadecimal da cor (por exemplo, #FF00FF):");

    // Ler a entrada do usuário
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    let input = input.trim();

    // Validar o código hexadecimal
    let hex_code = match validate_hex_code(input) {
        Some(code) => code,
        None => {
            eprintln!("Código hexadecimal inválido.");
            return;
        }
    };

    // Converter o código hexadecimal em valores RGB
    let (red, green, blue) = hex_to_rgb(&hex_code);

    println!("Valores RGB: R: {}, G: {}, B: {}", red, green, blue);

    // Mostrar os 24 bits que seriam enviados
    show_24bit_data(red, green, blue);
}

fn validate_hex_code(input: &str) -> Option<String> {
    // Expressão regular para validar o código hexadecimal
    let re = Regex::new(r"^#?([A-Fa-f0-9]{6})$").unwrap();

    if let Some(caps) = re.captures(input) {
        // Retornar apenas os dígitos hexadecimais
        Some(caps.get(1).unwrap().as_str().to_string())
    } else {
        None
    }
}

fn hex_to_rgb(hex_code: &str) -> (u8, u8, u8) {
    let red = u8::from_str_radix(&hex_code[0..2], 16).unwrap();
    let green = u8::from_str_radix(&hex_code[2..4], 16).unwrap();
    let blue = u8::from_str_radix(&hex_code[4..6], 16).unwrap();

    (red, green, blue)
}

fn show_24bit_data(red: u8, green: u8, blue: u8) {
    // Combinar os valores RGB em um único número de 24 bits
    let data_24bit = ((red as u32) << 16) | ((green as u32) << 8) | (blue as u32);

    // Exibir os 24 bits em formato binário
    println!("Dados de 24 bits em binário: {:024b}", data_24bit);

    // Exibir os 24 bits em formato hexadecimal
    println!("Dados de 24 bits em hexadecimal: 0x{:06X}", data_24bit);
}
