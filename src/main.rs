use dialoguer::{Input, Select};
use std::process::Command;
use std::process;
use std::io::{self, Write};

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};




fn open_chrome(site: &str){

    println!("Site digitado: {}", site);
}

fn open_project(project: &String){


    Command::new("powershell")
        .args("/c")
        .args(&format!("Start-Process nvim {} -Wait -NoNewWindow", &project))
        .output()
        .expect("Falha ao abrir o Projeto");


//     Command::new("nvim")
//         .arg(" .")
//         .spawn()
//         .expect("Erro ao abrir o Neovim!");
}




fn main() {


            // Inicializa o Terminal com Crossterm
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),  // Define a cor do texto
            Print("----- Alfred -----"), // Imprime Jarvis
            Print("\n"),
            ResetColor                // Restaura a cor padrão do Terminal
        )
        .unwrap();

        stdout.flush().unwrap(); // Limpa o buffer e exibe o texto no terminal
    

        println!("Olá, sou Alfred seu assistente pessoal, o que deseja?");

        let menu = Select::new()
            .item("Abrir Chrome + Projeto")
            .item("Abrir Projeto")
            .item("Abrir Chrome")
            .item("Sair")
            .default(0)
            .interact()
            .unwrap();


        match menu {
        
            0 => {
                // Solicita ao usuário o site para abrir o Chrome
                let site: String = Input::new()
                    .with_prompt("Digite o site para abrir no Chrome")
                    .interact()
                    .unwrap();

                // Solicita ao usuário o projeto para abrir no LunarVim
                let project: String = Input::new()
                    .with_prompt("Digite o caminho do projeto para abrir no LunarVim")
                    .interact()
                    .unwrap();

                open_chrome(&site);
                open_project(&project);
            },

            1 => {
                // Solicita ao usuário o projeto para abrir no LunarVim
                let project: String = Input::new()
                    .with_prompt("Digite o caminho do projeto para abrir no LunarVim")
                    .interact()
                    .unwrap();

                open_project(&project);
            },

            2 => {
                // Solicita ao usuário o site para abrir o Chrome
                let site: String = Input::new()
                    .with_prompt("Digite o site para abrir no Chrome")
                    .interact()
                    .unwrap();

                open_chrome(&site);
            },
            3 => {
                // Saindo
                println!("Encerrando a CLI...");
                process::exit(0);
            },
            _ => println!("Escolha inválida"), 
        }
    }
