use crate::calcular_sistema::Calcular;

fn main() {
    comandos::display::display_intro_logo();
    comandos::display::display_apresentacao_mensagem();

    let input_x: u8 = comandos::gerenciar_comandos::input_do_tamanho();
    let input_y: u8 = comandos::gerenciar_comandos::input_do_tamanho();

    println!("Sistema de tamanho {}x{}", input_x, input_y);

    let v_valores_do_sistema = comandos::gerenciar_comandos::valor_no_array(input_x, input_y);

    println!("{:?}", v_valores_do_sistema);

    let temp = calcular_sistema::Calcular::definir_valores(input_x, input_y, v_valores_do_sistema);

    Calcular::xama(&temp);
}

pub mod comandos {
    pub mod display {
        pub fn display_intro_logo() {
            println!(" ______                   _____  _      _         ");
            println!(" | ___ |                 /  ___|| |    (_)        ");
            println!(" | |_/ / ___  ___  ______| `--. | |     _  _ __   ");
            println!(" |    / / _ |/ __||______|`--. || |    | || '_ |  ");
            println!(" | || ||  __/|__ |       /__/  /| |____| || | | | ");
            println!(" |_| |_||___||___/       |____/ |_____/|_||_| |_| ");
            println!("                                                  ");
        }

        pub fn display_apresentacao_mensagem() {
            println!("Bem-vindo ao Res-SLin! (Resolvedor de Sistemas Lineares)");
            println!("Aqui você pode resolver um Sistema de até 10x10!");
            println!("");
            println!("Digite a quantidade de valores do sistema:");
        }
    }
    pub mod gerenciar_comandos {
        use std::{io, u8};

        pub fn input_do_tamanho() -> u8 {
            loop {
                let mut input_do_usuario = String::new();

                io::stdin()
                    .read_line(&mut input_do_usuario)
                    .expect("Valor inválido!");

                let input_do_usuario: u8 = match input_do_usuario.trim().parse() {
                    Ok(u8) => u8,
                    Err(_) => continue,
                };

                if menor_que_dez(input_do_usuario) {
                    return input_do_usuario;
                }
                println!("Valor inválido. Digite novamente: ");
            }
        }

        pub fn menor_que_dez(numero: u8) -> bool {
            numero <= 10
        }

        pub fn valor_no_array(x: u8, y: u8) -> Vec<i32> {
            let mut i = 0;
            let mut j = 0;
            let mut v = Vec::new();

            while j < y {
                println!("Linha {} do Sistema.\n", j);

                while i <= x {
                    if i == x {
                        println!("Digite o S do Sistema da linha {}: ", j);
                    } else {
                        println!("Digite o X{} do Sistema da linha {}: ", i, j);
                    }

                    let mut valor = String::new();

                    io::stdin().read_line(&mut valor).expect("Valor inválido!");
                    let valor: i32 = match valor.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    v.push(valor);
                    i += 1;
                }
                j += 1;
                i = 0;
            }

            v
        }
    }
}

pub mod calcular_sistema {
    pub struct Calcular {
        valor_x: u8,
        valor_y: u8,
        vetor_valores: Vec<i32>,
    }

    impl Calcular {
        pub fn definir_valores(x: u8, y: u8, v: Vec<i32>) -> Calcular {
            Calcular {
                valor_x: x,
                valor_y: y,
                vetor_valores: v,
            }
        }

        fn calcular_tamanho_total(&self) -> u8 {
            (self.valor_x + 1) * self.valor_y
        }

        pub fn xama(&self) {
            let oi = self.calcular_tamanho_total();
            println!("{}", oi);
        }
    }
}
