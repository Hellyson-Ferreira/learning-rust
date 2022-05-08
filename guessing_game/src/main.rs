extern crate rand; // adicionando dependência externa
use std::io::stdin; // Importando função de um módulo
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("O numero secreto é {}", secret_number);    

    loop{

        println!("Digite o seu palpite [0 pra sair.]");

        let mut guess = String::new();// Em rust por padrão todas a variáveis são imutáveis, o `mut` serve pra explicitar que um variável vai mudar de valor 

        stdin()
        .read_line(&mut guess)//referências são imutáveis por padrão. Por isso, precisamos escrever &mut guess, em vez de apenas &guess
        .expect("Falha ao ler entrada");
    
        println!("Você disse: {guess}");
    
        let guess: u32 = match guess.trim().parse(){
                Err(_) => continue,
                Ok(num) =>{ 
                    match num{
                        0 => break,
                        num => num
                    }
                },
                
        };
    
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }

    }
}
