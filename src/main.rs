
use rand::Rng;
use std::io;

fn convert_to_int(data_input: &String) -> i32 {
    match data_input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Erro: Entrada inválida. Usando valor 0 por padrão.");
            0 // valor padrão em caso de erro
        }
    }
}

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

    println!("Sua mão é: {}, {}, e {}", jcarta1, jcarta2, jcarta3);
    println!("A mão da máquina é: {}, {}, {}", mcarta1, mcarta2, mcarta3);

    let mut jjogada1 = String::new();
    println!("Qual carta quer jogar? 1, 2 ou 3");
    io::stdin().read_line(&mut jjogada1).expect("ERRO ao ler");

    //Quando você le a escolha do jogador o valor é do tipo string e você precisa converter para i32
    let jescolha1 = jjogada1.clone();
    if convert_to_int(&jjogada1) == 1{
        let ccarta = jcarta1.to_string(); //convertar a variavel jcarta1 para string para que a jjogada1 possa receber o valor
        jjogada1 = ccarta;
    }

    if convert_to_int(&jjogada1) == 2{
        let ccarta = jcarta2.to_string(); 
        jjogada1 = ccarta;
    }

    if convert_to_int(&jjogada1) == 3{
        let ccarta = jcarta3.to_string();
        jjogada1 = ccarta;
    }

    //Jogada da máquina

    let mut mjogada1 = rng.gen_range(1..=3);

    let mescolha1 = mjogada1.clone();

    if mjogada1 == 1{
        mjogada1 = mcarta1;
    }

    if mjogada1 == 2{
        mjogada1 = mcarta2;
    }

    if mjogada1 == 3{
        mjogada1 = mcarta3;
    }




    //let numero: i32 = string_numero.parse().expect("Não foi possível converter para número");
    println!("Você jogou a carta: {}", jjogada1);
    println!("a máquina jogou a carta: {}", mjogada1);


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

    
    /*jogador*/
    let mut jjogada2 = String::new();
    let mut ccarta2 = String::new();
    //let numero_str = "123";new();
    //|| jescolha1 == 1

    if convert_to_int(&jescolha1) == 1{
        println!("Sua mão é: {} e {}", jcarta2, jcarta3);
        
        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let let jescolha2 = jjogada2.clone();
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta2.to_string();
            ucj += jcarta3.clone(); 
            jjogada2 = ccarta2.clone();
        }

        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta3.to_string(); 
            ucj += jcarta2.clone();
            jjogada2 = ccarta2.clone();
        }
    }



    if convert_to_int(&jescolha1) == 2{
        println!("Sua mão é: {} e {}", jcarta1, jcarta3);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone(); inativo
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta1.to_string();
            ucj += jcarta3.clone(); 
            jjogada2 = ccarta2.clone();
        }
        
        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta3.to_string();
            ucj += jcarta1.clone(); 
            jjogada2 = ccarta2.clone();
        }
    }

    if convert_to_int(&jescolha1) == 3{
        println!("Sua mão é: {} e {}", jcarta1, jcarta2);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone();
        
        if convert_to_int(&jjogada2) == 1{
            ccarta2 = jcarta1.to_string();
            ucj += jcarta2.clone(); 
            jjogada2 = ccarta2.clone();
        }

        if convert_to_int(&jjogada2) == 2{
            ccarta2 = jcarta2.to_string();
            ucj += jcarta1.clone(); 
            jjogada2 = ccarta2.clone();
        }

    }


    /*maquina*/
    //println!("{}",&ccarta2);
    let mut ucm = 0; //ucm = Úĺtima Carta Máquina
    let mut mjogada2 = 0;
    if mescolha1 == 1{
        mjogada2 = rng.gen_range(2..=3);

        if mjogada2 == 2{
            mjogada2 = mcarta2;
            ucm = mcarta3;
        }

        if mjogada2 == 3{
            mjogada2 = mcarta3;
            ucm = mcarta2;
        }
    }

    if mescolha1 == 2{
       mjogada2 = if rng.gen_bool(0.5) { 1 } else { 3 }; // Gera 1 ou 3 

        if mjogada2 == 1{
            mjogada2 = mcarta1;
            ucm = mcarta3;
        }

        if mjogada2 == 3{
            mjogada2 = mcarta3;
            ucm = mcarta1;
        }

    }

    if mescolha1 == 3{
        mjogada2 = rng.gen_range(1..=2);

        if mjogada2 == 1{
            mjogada2 = mcarta1;
            ucm = mcarta2;


        }

        if mjogada2 == 2{
            mjogada2 = mcarta2;
            ucm = mcarta1;
        }
    }

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
    println!("Sua última carta é: {}", ucj);
    
    println!("Digite 1 para jogar!");
    io::stdin().read_line(&mut jjogada3).expect("ERRO ao ler");

    if convert_to_int(&jjogada3) == 1{
        jjogada3 == ucj.to_string();
    }
   
    /*maquina*/
    let mut mjogada3 = ucm;

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

    println!("você ganhou {} rodadas, a maquina ganhou {} rodadas, mao {}", jrodada, mrodada, jescolha1);

    //fim da rodada
    
}

