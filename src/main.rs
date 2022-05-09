use crate::calcular_sistema::Calcular;
use comandos::display;

fn main() {
    display::display_intro_logo();
    display::display_apresentacao_mensagem();

    let input_x: usize = comandos::gerenciar_comandos::input_do_tamanho();
    let input_y: usize = comandos::gerenciar_comandos::input_do_tamanho();

    println!("Sistema de tamanho {}x{}", input_x, input_y);

    let v_valores_do_sistema = comandos::gerenciar_comandos::valor_no_array(input_x, input_y);

    println!("Sistema Original:\n{:?}\n", &v_valores_do_sistema);

    let calc_sis_lin = Calcular::definir_valores(input_x, input_y, v_valores_do_sistema);

    Calcular::imprimir_resultado(calc_sis_lin);
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
        use std::io;

        pub fn input_do_tamanho() -> usize {
            loop {
                let mut input_do_usuario = String::new();

                io::stdin()
                    .read_line(&mut input_do_usuario)
                    .expect("Valor inválido!");

                let input_do_usuario: usize = match input_do_usuario.trim().parse() {
                    Ok(usize) => usize,
                    Err(_) => continue,
                };

                if input_do_usuario <= 10 {
                    return input_do_usuario;
                }

                println!("Valor inválido. Digite novamente: ");
            }
        }

        pub fn valor_no_array(qnt_linhas: usize, qnt_valores: usize) -> Vec<Vec<f64>> {
            let mut vetores_na_linha: Vec<Vec<f64>> = Vec::new();
            let mut valor_dos_vetores;

            for linhas in 0..qnt_linhas {
                println!("\nLinha {} do Sistema.\n", linhas);
                valor_dos_vetores = vec![0.0];

                for valores in 0..qnt_valores + 1 {
                    if valores == qnt_valores {
                        println!("Digite a Soma (S) do Sistema da linha {}: ", linhas);
                    } else {
                        println!(
                            "Digite o Valor {} do Sistema da linha {}: ",
                            valores, linhas
                        );
                    }

                    let mut valor = String::new();

                    io::stdin().read_line(&mut valor).expect("Valor inválido!");

                    let valor: f64 = match valor.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    valor_dos_vetores.push(valor);
                }
                vetores_na_linha.push(valor_dos_vetores);
                vetores_na_linha[linhas].remove(0);
            }
            vetores_na_linha
        }
    }
}

pub mod calcular_sistema {
    pub struct Calcular {
        valor_x: usize,
        valor_y: usize,
        vetor_valores: Vec<Vec<f64>>,
    }

    impl Calcular {
        pub fn definir_valores(x: usize, y: usize, v: Vec<Vec<f64>>) -> Calcular {
            Calcular {
                valor_x: x + 1,
                valor_y: y,
                vetor_valores: v,
            }
        }

        fn resolver_valores(
            Self {
                valor_x,
                valor_y,
                mut vetor_valores,
            }: Self,
        ) -> Vec<Vec<f64>> {
            let (qnt_val, qnt_lin) = (valor_x, valor_y);

            for i in 0..qnt_val {
                for j in (i + 1)..qnt_lin {
                    let pivo = &vetor_valores[j][i] / &vetor_valores[i][i];

                    for k in 0..qnt_val {
                        let resultado = &vetor_valores[j][k] + (-pivo) * &vetor_valores[i][k];

                        vetor_valores[j].insert(k, resultado);
                        vetor_valores[j].remove(k + 1);
                    }
                }
            }

            vetor_valores
        }

        fn valores_de_x(self) -> Vec<f64> {
            let res = Calcular::resolver_valores(self);
            let (qnt_lin, qnt_val) = (res.len() - 1, res[0].len() - 1);

            let mut valor_final_x: Vec<f64> = Vec::new();
            let mut multiplicar_x = 0.0;
            let mut pivo_res = 0;

            println!("Sistema resolvido!\n{:?}\n", &res);

            for _l in 0..qnt_lin {
                valor_final_x.push(0.0);
            }

            valor_final_x.push(res[qnt_lin][qnt_val] / res[qnt_lin][qnt_val - 1]);

            for linhas in (0..qnt_lin).rev() {
                for valores in (0..qnt_val).rev() {
                    if linhas == valores {
                        continue;
                    }

                    multiplicar_x -= &res[linhas][valores] * valor_final_x[valores];
                }
                multiplicar_x += &res[linhas][qnt_val];
                multiplicar_x /= &res[linhas][linhas];

                valor_final_x.insert(qnt_lin - pivo_res, multiplicar_x);
                valor_final_x.remove(0);

                pivo_res += 1;
                multiplicar_x = 0.0;
            }

            valor_final_x
        }

        pub fn imprimir_resultado(self) {
            let res = Calcular::valores_de_x(self);
            let mut index_val = 1;

            for i in res {
                println!("X{} = {}", index_val, i);

                index_val += 1;
            }
        }
    }
}
