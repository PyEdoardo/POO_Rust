use std::io;
use std::process::exit;
use chrono::{DateTime, Local};


struct Produto {
    name: String,
    codigo: i16,
    preco: f64,
    descricao: String,
    estoque: Estoque
}

struct Estoque {
    quantidade : i32,
    data_adicao_estoque: DateTime<Local>
}

impl Produto {
    fn vender(&mut self, quantidade: i32) {
        if self.estoque.quantidade >= quantidade {
            self.estoque.quantidade -= quantidade;
            println!("Venda do produto {} confirmada!", self.name);
        }
        else {
            println!("Venda do produto {} não foi realizada!", self.name);
        }
    }
    //Getters
    fn get_nome(self) -> String {
        return self.name;
    }
    fn get_codigo(self) -> i16 {
        return self.codigo;
    }
    fn get_preco(self) -> f64 {
        return self.preco;
    }
    fn get_descricao(self) -> String {
        return self.descricao;
    }
    fn get_estoque(self) -> Estoque {
        return self.estoque;
    }

    //Setters

    fn set_nome(&mut self, nome: String) {
        self.name = nome;
    }

    fn set_codigo(&mut self, codigo: i16) {
        self.codigo = codigo;
    }

    fn set_preco(&mut self, preco: f64) {
        self.preco = preco;
    }

    fn set_descr(&mut self, descr: String) {
        self.descricao = descr;
    }

    fn set_estoque(&mut self, estoque: Estoque) {
        self.estoque = estoque;
    }

    fn adicionar_ao_estoque(&mut self, qt_produto: i8){
        self.estoque.quantidade += qt_produto as i32;
    }

    fn remover_do_estoque(&mut self, qt_produto: i8){
        if (self.estoque.quantidade <= 0) {
            println!("O Estoque está vazio, não foi possível remover!\nEstoque: {}", self.estoque.quantidade);
        }
        else {
            self.estoque.quantidade -= qt_produto as i32;
        }
    }
    
    fn to_string(&self){
        println!(
            "Nome: {}\nCódigo: {}\nPreço: {}, Descrição: {}\n---------------\nEstoque: {}\nData Estoque: {}\n",
            self.name,
            self.codigo,
            self.preco,
            self.descricao,
            self.estoque.quantidade,
            self.estoque.data_adicao_estoque
        )
    }
}

fn main() {
    let mut lista_produtos = vec![Produto {codigo :1, name: String::from("Químico"), preco: 40.22, descricao: String::from("Sla"), estoque: Estoque {quantidade: 50, data_adicao_estoque: Local::now()}}];
    while true {
        let mut opcao: String = String::new();
        println!("Digite uma opção: (adicionar estoque, remover estoque, printar produtos, realizar venda, adicionar Produto)");
        io::stdin()
            .read_line(&mut opcao)
            .expect("Falha ao ler a linha");

        let mut opcao = opcao.trim();

        //opcao = adicionar estoque, remover estoque, printar produtos, realizar venda, adicionar Produto

        match opcao {
            "adicionar estoque" => {
                let mut qt_add: String = String::new();
                io::stdin()
                    .read_line(&mut qt_add)
                    .expect("Falha ao ler a linha");
                let mut add: i8 = qt_add.trim().parse().unwrap();

                let mut codigo: String = String::new();

                io::stdin()
                    .read_line(&mut codigo)
                    .expect("Falha ao ler a linha");

                let mut codigo_int: i16 = codigo.trim().parse().unwrap();
                for x in &mut lista_produtos {
                    if (x.codigo == codigo_int) {
                        x.adicionar_ao_estoque(add as i8);
                    }
                }
            }
        "remover estoque" => {

            }
        "printar" => {
            for x in &lista_produtos {
                x.to_string();
            }
        }
        "sair" => {
            exit(0);
        }
        _ => {
            println!("Erro na opção!");
            panic!();
            }
        }
    }
}
