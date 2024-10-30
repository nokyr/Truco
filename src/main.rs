use rand::Rng;
use std::io;
use rand::seq::SliceRandom; // Importa o trait para usar o método choose
use rand::thread_rng; // Para obter um gerador de números aleatórios

fn convert_to_int(data_input: &String) -> i32 {
    match data_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Erro: Entrada inválida. Usando valor 0 por padrão.");
            0 // valor padrão em caso de erro
        }
    }
}

static mut pontos_jogador: i32 = 0;
static mut pontos_maquina: i32 = 0;

fn main() {

    let mut jrodada = 0;
    let mut mrodada = 0;
    let mut rpj = 0; //rpj = Rodada Pontos Jogador
    let mut rpm = 0; //rpm = Rodada Pontos Máquina


    // Cria um gerador de números aleatórios
    let mut rng = rand::thread_rng();

    // Gera um número aleatório entre 1 e 14 (inclusive)
    //
    //Jogador

    let jcarta1 = rng.gen_range(1..=14);
    let jcarta2 = rng.gen_range(1..=14);
    let jcarta3 = rng.gen_range(1..=14);

    //Maquina
    let mcarta1 = rng.gen_range(1..=14);
    let mcarta2 = rng.gen_range(1..=14);
    let mcarta3 = rng.gen_range(1..=14);

    
    /*<Sistema para mostrar as cartas / para converter número para carta, tipo 1 para 4 de ouro>*/
    //naipes ♣♠♥♦

    //carta1 jogador
    let mut carta_jogador1 = String::new();
    let mut jcarta1_off = String::new();

    if jcarta1 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        jcarta1_off = format!("4{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta1_off = format!("5{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        jcarta1_off = format!("6{}", naipe);
        carta_jogador1 = jcarta1_off;

    }
    if jcarta1 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta1_off = format!("7{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("Q{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("J{}", naipe);
        carta_jogador1 = jcarta1_off; 
    }
    if jcarta1 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("K{}", naipe);
        carta_jogador1 = jcarta1_off;
     }
    if jcarta1 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("A{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("2{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta1_off = format!("3{}", naipe);
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 11{
        jcarta1_off = String::from("7♦");
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 12{
        jcarta1_off = String::from("A♠");
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 13{
        jcarta1_off = String::from("7♥");
        carta_jogador1 = jcarta1_off;
    }
    if jcarta1 == 14{
        jcarta1_off = String::from("4♣");
        carta_jogador1 = jcarta1_off;
    }

    //carta2 jogador
    let mut carta_jogador2 = String::new();
    let mut jcarta2_off = String::new();

    if jcarta2 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        jcarta2_off = format!("4{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta2_off = format!("5{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        jcarta2_off = format!("6{}", naipe);
        carta_jogador2 = jcarta2_off;

    }
    if jcarta2 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta2_off = format!("7{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("Q{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("J{}", naipe);
        carta_jogador2 = jcarta2_off; 
    }
    if jcarta2 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("K{}", naipe);
        carta_jogador2 = jcarta2_off;
     }
    if jcarta2 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("A{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("2{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta2_off = format!("3{}", naipe);
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 11{
        jcarta2_off = String::from("7♦");
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 12{
        jcarta2_off = String::from("A♠");
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 13{
        jcarta2_off = String::from("7♥");
        carta_jogador2 = jcarta2_off;
    }
    if jcarta2 == 14{
        jcarta2_off = String::from("4♣");
        carta_jogador2 = jcarta2_off;
    }

    //carta3 jogador
    let mut carta_jogador3 = String::new();
    let mut jcarta3_off = String::new();

    if jcarta3 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        jcarta3_off = format!("4{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta3_off = format!("5{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        jcarta3_off = format!("6{}", naipe);
        carta_jogador3 = jcarta3_off;

    }
    if jcarta3 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        jcarta3_off = format!("7{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("Q{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("J{}", naipe);
        carta_jogador3 = jcarta3_off; 
    }
    if jcarta3 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("K{}", naipe);
        carta_jogador3 = jcarta3_off;
     }
    if jcarta3 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("A{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("2{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        jcarta3_off = format!("3{}", naipe);
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 11{
        jcarta3_off = String::from("7♦");
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 12{
        jcarta3_off = String::from("A♠");
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 13{
        jcarta3_off = String::from("7♥");
        carta_jogador3 = jcarta3_off;
    }
    if jcarta3 == 14{
        jcarta3_off = String::from("4♣");
        carta_jogador3 = jcarta3_off;
    }
    /*</>*/

    //carta1 máquina
    let mut carta_maquina1 = String::new();
    let mut mcarta1_off = String::new();

    if mcarta1 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        mcarta1_off = format!("4{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta1_off = format!("5{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        mcarta1_off = format!("6{}", naipe);
        carta_maquina1 = mcarta1_off;

    }
    if mcarta1 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta1_off = format!("7{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("Q{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("J{}", naipe);
        carta_maquina1 = mcarta1_off; 
    }
    if mcarta1 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("K{}", naipe);
        carta_maquina1 = mcarta1_off;
     }
    if mcarta1 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("A{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("2{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta1_off = format!("3{}", naipe);
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 11{
        mcarta1_off = String::from("7♦");
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 12{
        mcarta1_off = String::from("A♠");
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 13{
        mcarta1_off = String::from("7♥");
        carta_maquina1 = mcarta1_off;
    }
    if mcarta1 == 14{
        mcarta1_off = String::from("4♣");
        carta_maquina1 = mcarta1_off;
    }

    //carta2 maquina
    let mut carta_maquina2 = String::new();
    let mut mcarta2_off = String::new();

    if mcarta2 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        mcarta2_off = format!("4{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta2_off = format!("5{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        mcarta2_off = format!("6{}", naipe);
        carta_maquina2 = mcarta2_off;

    }
    if mcarta2 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta2_off = format!("7{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("Q{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("J{}", naipe);
        carta_maquina2 = mcarta2_off; 
    }
    if mcarta2 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("K{}", naipe);
        carta_maquina2 = mcarta2_off;
     }
    if mcarta2 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("A{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("2{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta2_off = format!("3{}", naipe);
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 11{
        mcarta2_off = String::from("7♦");
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 12{
        mcarta2_off = String::from("A♠");
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 13{
        mcarta2_off = String::from("7♥");
        carta_maquina2 = mcarta2_off;
    }
    if mcarta2 == 14{
        mcarta2_off = String::from("4♣");
        carta_maquina2 = mcarta2_off;
    }

    //carta3 jogador
    let mut carta_maquina3 = String::new();
    let mut mcarta3_off = String::new();

    if mcarta3 == 1{
        let naipes = vec!["♦", "♥", "♠"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); // .unwrap() é usado aqui para lidar com a possibilidade de None
        //escolhe 1 dos 3 naipes da carta

        mcarta3_off = format!("4{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 2{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta3_off = format!("5{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 3{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap(); 
        mcarta3_off = format!("6{}", naipe);
        carta_maquina3 = mcarta3_off;

    }
    if mcarta3 == 4{
        let naipes = vec!["♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();
        

        mcarta3_off = format!("7{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 5{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("Q{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 6{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("J{}", naipe);
        carta_maquina3 = mcarta3_off; 
    }
    if mcarta3 == 7{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("K{}", naipe);
        carta_maquina3 = mcarta3_off;
     }
    if mcarta3 == 8{
        let naipes = vec!["♦", "♥", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("A{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 9{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("2{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 10{
        let naipes = vec!["♦", "♥", "♠", "♣"];
        let mut rng = thread_rng();
        let naipe = naipes.choose(&mut rng).unwrap();

        mcarta3_off = format!("3{}", naipe);
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 11{
        mcarta3_off = String::from("7♦");
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 12{
        mcarta3_off = String::from("A♠");
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 13{
        mcarta3_off = String::from("7♥");
        carta_maquina3 = mcarta3_off;
    }
    if mcarta3 == 14{
        mcarta3_off = String::from("4♣");
        carta_maquina3 = mcarta3_off;
    }
    /*</>*/


    println!("Sua mão é: {}, {}, e {}", carta_jogador1, carta_jogador2, carta_jogador3);
    println!("A mão da máquina é: {}, {}, {}", mcarta1, mcarta2, mcarta3);

    let mut jjogada1 = String::new();
    let mut cjjogada1 = String::new();

    println!("Qual carta quer jogar? 1, 2 ou 3");
    io::stdin().read_line(&mut jjogada1).expect("ERRO ao ler");

    //Quando você le a escolha do jogador o valor é do tipo string e você precisa converter para i32
    let jescolha1 = jjogada1.clone();
    if convert_to_int(&jjogada1) == 1{
        let ccarta = jcarta1.to_string(); //convertar a variavel jcarta1 para string para que a jjogada1 possa receber o valor
        cjjogada1 = carta_jogador1.clone();
        jjogada1 = ccarta;
    }

    if convert_to_int(&jjogada1) == 2{
        let ccarta = jcarta2.to_string(); 
        cjjogada1 = carta_jogador2.clone();
        jjogada1 = ccarta;
    }

    if convert_to_int(&jjogada1) == 3{
        let ccarta = jcarta3.to_string();
        cjjogada1 = carta_jogador3.clone();
        jjogada1 = ccarta;
    }

    //Jogada da máquina

    let mut mjogada1 = rng.gen_range(1..=3);
    let mut cmjogada1 = String::new();

    let mescolha1 = mjogada1.clone();

    if mjogada1 == 1{
        mjogada1 = mcarta1;
        cmjogada1 = carta_maquina1.clone();
    }

    if mjogada1 == 2{
        mjogada1 = mcarta2;
        cmjogada1 = carta_maquina2.clone();
    }

    if mjogada1 == 3{
        mjogada1 = mcarta3;
        cmjogada1 = carta_maquina3.clone();
    }




    //let numero: i32 = string_numero.parse().expect("Não foi possível converter para número");
    println!("Você jogou a carta: {}", cjjogada1);
    println!("a máquina jogou a carta: {}", cmjogada1);


    //Ganhador

    let mut ganhador1 = 0;


    if convert_to_int(&jjogada1) > mjogada1{
        ganhador1 += 1;
        rpj += 1;

    }
    else if convert_to_int(&jjogada1) == mjogada1{
        ganhador1 += 3;
        rpj += 1;
        rpm += 1;
    }
    else {
        ganhador1 += 2;
        rpm += 1;   
    }

    if ganhador1 == 1{
        println!("Você ganhou!");
        jrodada = jrodada+1;
    }

    else if ganhador1 == 3{
        println!("Empate!");
        jrodada = jrodada+1;
        mrodada = mrodada+1;
    }
    else {
        println!("A máquina ganhou!");
        mrodada = mrodada+1;
    }    
    
    //rodada 2

    let mut ucj = 0; //ucj = Última Carta Jogador
    let mut ucj_off = String::new(); //a ucm é int, essa var ja é str

    
    /*jogador*/
    let mut jjogada2 = String::new();
    let mut ccarta2 = String::new();
    let mut cjjogada2 = String::new();
    //let numero_str = "123";new();
    //|| jescolha1 == 1

    
    if convert_to_int(&jescolha1) == 1{
        println!("Sua mão é: {} e {}", carta_jogador2, carta_jogador3);


        
        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let let jescolha2 = jjogada2.clone();
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta2.to_string();
            ucj += jcarta3.clone();
            ucj_off = carta_jogador3.clone();
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador2.clone();
        }

        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta3.to_string(); 
            ucj += jcarta2.clone();
            ucj_off = carta_jogador2.clone();
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador3.clone();
        }
    }



    if convert_to_int(&jescolha1) == 2{
        println!("Sua mão é: {} e {}", carta_jogador1, carta_jogador3);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone(); inativo
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta1.to_string();
            ucj += jcarta3.clone();
            ucj_off = carta_jogador3.clone(); 
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador1.clone();
        }
        
        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta3.to_string();
            ucj += jcarta1.clone();
            ucj_off = carta_jogador1.clone(); 
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador3.clone();
        }
    }

    if convert_to_int(&jescolha1) == 3{
        println!("Sua mão é: {} e {}", carta_jogador1, carta_jogador2);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone();
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta1.to_string();
            ucj += jcarta2.clone();
            ucj_off = carta_jogador2.clone(); 
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador1.clone();
        }

        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta2.to_string();
            ucj += jcarta1.clone();
            ucj_off = carta_jogador1.clone(); 
            jjogada2 = ccarta2.clone();
            cjjogada2 = carta_jogador2.clone();
        }

    }


    /*maquina*/
    //println!("{}",&ccarta2);
    let mut ucm = 0; //ucm = Úĺtima Carta Máquina
    let mut ucm_off = String::new(); //a ucm é int, essa var ja é str
    let mut mjogada2 = 0;
    let mut cmjogada2 = String::new();

    if mescolha1 == 1{
        mjogada2 = rng.gen_range(2..=3);

        if mjogada2 == 2{
            mjogada2 = mcarta2;
            cmjogada2 = carta_maquina2.clone();
            ucm = mcarta3;
            ucm_off = carta_maquina3.clone();
        }

        if mjogada2 == 3{
            mjogada2 = mcarta3;
            cmjogada2 = carta_maquina3.clone();
            ucm = mcarta2;
            ucm_off = carta_maquina2.clone();
        }
    }

    if mescolha1 == 2{
       mjogada2 = if rng.gen_bool(0.5) { 1 } else { 3 }; // Gera 1 ou 3 

        if mjogada2 == 1{
            mjogada2 = mcarta1;
            cmjogada2 = carta_maquina1.clone();
            ucm_off = carta_maquina3.clone();
            ucm = mcarta3;
        }

        if mjogada2 == 3{
            mjogada2 = mcarta3;
            cmjogada2 = carta_maquina3.clone();
            ucm_off = carta_maquina1.clone();
            ucm = mcarta1;
        }

    }

    if mescolha1 == 3{
        mjogada2 = rng.gen_range(1..=2);

        if mjogada2 == 1{
            mjogada2 = mcarta1;
            cmjogada2 = carta_maquina1.clone();
            ucm_off = carta_maquina2.clone();
            ucm = mcarta2;


        }

        if mjogada2 == 2{
            mjogada2 = mcarta2;
            cmjogada2 = carta_maquina2.clone();
            ucm_off = carta_maquina1.clone();
            ucm = mcarta1;
        }
    }

    println!("Você jogou a carta: {}", cjjogada2);
    println!("a máquina jogou a carta: {}", cmjogada2);

    //Ganhador Segunda Rodada
    let mut ganhador2 = 0;

    if convert_to_int(&jjogada2) > mjogada2{
        ganhador2 += 1;
        rpj += 1;

    }
    else if convert_to_int(&jjogada2) == mjogada2{
        ganhador2 += 3;
        rpj += 1;
        rpm += 1;
    }
    else {
        ganhador2 += 2;
        rpm += 1;   
    }

    if ganhador2 == 1{
        println!("Você ganhou!");
        jrodada += 1;
    }

    else if ganhador2 == 3{
        println!("Empate!");
        jrodada += 1;
        mrodada += 1;
    }
    else {
        println!("A máquina ganhou!");
        mrodada += 1;
    }

    //rodada 3

    /*jogador*/
    let mut ganhador3 = 0;

    let mut jjogada3 = String::new();
    println!("Sua última carta é: {}", ucj_off);
    
    println!("Digite 1 para jogar!");
    io::stdin().read_line(&mut jjogada3).expect("ERRO ao ler");

    if convert_to_int(&jjogada3) == 1{
        jjogada3 == ucj.to_string();
    }
   
    
    let mut mjogada3 = ucm;

    println!("Você jogou a carta: {}", ucj_off);
    println!("a máquina jogou a carta: {}", ucm_off);
    
    /*Ganhador rodada 3*/

    if convert_to_int(&jjogada3) > mjogada3{
        ganhador3 += 1;
        rpj += 1;
    }
    else if convert_to_int(&jjogada3) == mjogada3{
        ganhador3 += 3;
        rpj += 1;
        rpm += 1;
    }
    else{
        ganhador3 += 2;
        rpm += 1;
    }

    if ganhador3 == 1{
        println!("Você ganhou!");
        jrodada += 1;
    }

    else if ganhador3 == 3{
        println!("Empate!");
        jrodada += 1;
        mrodada += 1;
    }
    else {
        println!("A máquina ganhou!");
        mrodada += 1;
    }

    println!("você ganhou rodadas {}, a maquina ganhou {}", jrodada, mrodada);

    //fim da rodada

    //coloca os pontos do jogador ou da máquina
    unsafe {
        if rpj > rpm{
            pontos_jogador += 1;
        }
        else if rpj == rpm{
            pontos_jogador += 1;
            pontos_maquina += 1;
        }
        else{
            pontos_maquina += 1;
        }
    }

    unsafe {
        println!("{} a {}", pontos_jogador, pontos_maquina);
    }

}



