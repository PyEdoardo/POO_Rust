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
    dataAdicaoEstoque : DateTime<Local>
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
    fn getNome(self) -> String {
        return self.name;
    }
    fn getCodigo(self) -> i16 {
        return self.codigo;
    }
    fn getPreco(self) -> f64 {
        return self.preco;
    }
    fn getDescricao(self) -> String {
        return self.descricao;
    }
    fn getEstoque(self) -> Estoque {
        return self.estoque;
    }

    //Setters

    fn setNome(&mut self, nome: String) {
        self.name = nome;
    }

    fn setCodigo(&mut self, codigo: i16) {
        self.codigo = codigo;
    }

    fn setPreco(&mut self, preco: f64) {
        self.preco = preco;
    }

    fn setDescr(&mut self, descr: String) {
        self.descricao = descr;
    }

    fn setEstoque(&mut self, estoque: Estoque) {
        self.estoque = estoque;
    }

    fn adicionarAoEstoque(&mut self, qt_produto: i8){
        self.estoque.quantidade += qt_produto as i32;
    }

    fn removerDoEstoque(&mut self, qt_produto: i8){
        if (self.estoque.quantidade <= 0) {
            println!("O Estoque está vazio, não foi possível remover!\nEstoque: {}", self.estoque.quantidade);
        }
        else {
            self.estoque.quantidade -= qt_produto as i32;
        }
    }

    fn toString(&self){
        println!(
            "Nome: {}\nCódigo: {}\nPreço: {}, Descrição: {}\n---------------\nEstoque: {}\nData Estoque: {}\n",
            self.name,
            self.codigo,
            self.preco,
            self.descricao,
            self.estoque.quantidade,
            self.estoque.dataAdicaoEstoque
        )
    }
}

fn main() {
    let mut listaProdutos = vec![Produto {codigo :1, name: String::from("Químico"), preco: 40.22, descricao: String::from("Sla"), estoque: Estoque {quantidade: 50, dataAdicaoEstoque: Local::now()}}];
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

                let mut codigoInt : i16 = codigo.trim().parse().unwrap();
                for x in &mut listaProdutos {
                    if (x.codigo == codigoInt) {
                        x.adicionarAoEstoque(add as i8);
                    }
                }
            }
        "remover estoque" => {

            }
        "printar" => {
            for x in &listaProdutos {
                x.toString();
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
