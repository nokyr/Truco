

 //rodada 2
    
    /*jogador*/
    let mut jjogada2 = String::new();
    let mut ccarta2 = String::new();
    //let numero_str = "123";new();

    if jescolha1 == "1"{
        println!("Sua mão é: {} e {}", jcarta2, jcarta3);
        
        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let let jescolha2 = jjogada2.clone();
        
        if jjogada2 == "1"{
            ccarta2 = jcarta2.to_string(); 
            jjogada2 = ccarta2.clone();
        }

        if jjogada2 == "2"{
            ccarta2 = jcarta3.to_string(); 
            jjogada2 = ccarta2.clone();
        }
    }



    if jescolha1 == "2"{
        println!("Sua mão é: {} e {}", jcarta1, jcarta3);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone(); inativo
        
        if jjogada2 == "1"{
            ccarta2 = jcarta1.to_string(); 
            jjogada2 = ccarta2.clone();
        }
        
        if jjogada2 == "2"{
            ccarta2 = jcarta3.to_string(); 
            jjogada2 = ccarta2.clone();
        }
    }

    if jjogada1 == "3"{
        println!("Sua mão é: {} e {}", jcarta1, jcarta2);

        
        println!("Qual carta quer jogar? 1 ou 2");
        io::stdin().read_line(&mut jjogada2).expect("ERRO ao ler");

        //let jescolha2 = jjogada2.clone();
        
        if jjogada2 == "1"{
            ccarta2 = jcarta1.to_string(); 
            jjogada2 = ccarta2.clone();
        }

        if jjogada2 == "2"{
            ccarta2 = jcarta2.to_string(); 
            jjogada2 = ccarta2.clone();
        }

    }


    /*maquina*/
    //println!("{}",&ccarta2);
    let mut mjogada2 = 0;
    if mescolha1 == 1{
        mjogada2 = rng.gen_range(2..=3);
    }

    if mescolha1 == 2{
       mjogada2 = if rng.gen_bool(0.5) { 1 } else { 3 }; // Gera 1 ou 3 
    }

    if mescolha1 == 3{
        mjogada2 = rng.gen_range(1..=2);
    }

    //Ganhador Segunda Rodada
    let mut ganhador2 = 0;

    if convert_to_int(&jjogada2) > mjogada2{
        ganhador2 += 1;

    }
    else if convert_to_int(&jjogada2) == mjogada2{
        ganhador2 += 3;
    }
    else {
        ganhador2 += 2;   
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

   

    
    println!("você ganhou {} rodadas, a maquina ganhou {} rodadas, mao {}", jrodada, mrodada, jescolha1);
    

}

Entendi o que você está tentando fazer. Parece que você quer atribuir valores a duas variáveis mutáveis, onde uma variável (`variavel2`) recebe o valor de outra (`variavel1`) dentro de um bloco `if`, e depois você quer usar a segunda variável fora do bloco. 

O que pode estar acontecendo é que, se o bloco `if` não for executado, a variável `variavel2` talvez não tenha sido inicializada corretamente antes de ser usada fora do bloco, o que causaria um erro de compilação ou execução.

Aqui está uma maneira simplificada de fazer isso, garantindo que a variável seja inicializada corretamente:

### Exemplo de código:

```rust
fn main() {
    let mut variavel1: Option<i32> = None;
    let mut variavel2: Option<i32> = None;

    if true {  // Aqui entra sua condição, como "se o céu for azul"
        variavel1 = Some(10);  // Atribui valor à variavel1
        variavel2 = variavel1;  // Atribui o valor de variavel1 a variavel2
    }

    // Usando variavel2 fora do if
    match variavel2 {
        Some(valor) => println!("O valor de variavel2 é: {}", valor),
        None => println!("variavel2 não foi inicializada."),
    }
}
```

### Explicação:
1. **`Option<T>`**: Eu usei `Option<i32>` para inicializar as variáveis com `None`, o que evita que você precise definir um valor padrão logo no início. Isso permite que a variável possa ser `None` ou conter um valor.
2. **`match`**: Depois, fora do `if`, usamos um `match` para verificar se `variavel2` foi realmente inicializada. Se foi (`Some(valor)`), imprimimos o valor. Caso contrário, tratamos o caso de erro adequadamente (`None`).

### Possível problema no seu código:
Se você está tentando usar `variavel2` fora do `if` sem garantir que ela tenha sido inicializada, Rust provavelmente está reclamando porque a variável pode não ter um valor válido. A ideia de usar `Option` ajuda a tratar essa situação de maneira segura, sem pânico ou erros inesperados.

Se essa explicação não resolver o problema, você pode compartilhar mais detalhes ou o código específico, e ficarei feliz em ajudar!

Em Rust, o **`match`** é uma construção que permite comparar um valor contra diferentes padrões e executar o código correspondente ao padrão que for compatível. Ele é uma espécie de "switch" em outras linguagens, mas com muito mais poder e flexibilidade.

O **`match`** é especialmente útil ao trabalhar com tipos como `Option`, `Result`, e enums, pois esses tipos podem ter diferentes "estados" (como `Some`/`None` para `Option` ou `Ok`/`Err` para `Result`), e o `match` facilita lidar com esses casos de forma segura.

### Exemplo básico de `match`:

```rust
fn main() {
    let numero = 2;

    match numero {
        1 => println!("O número é 1"),
        2 => println!("O número é 2"),
        3 => println!("O número é 3"),
        _ => println!("O número não é 1, 2 ou 3"),
    }
}
```

### Explicação:
- O `numero` é comparado com vários padrões: `1`, `2`, `3`, e o padrão curinga `_`, que captura qualquer valor que não seja explicitamente comparado.
- Dependendo do valor de `numero`, o código correspondente ao padrão é executado.
  
No exemplo acima, como `numero` é `2`, o output será: `O número é 2`.

### Exemplo com `Option`:

O `Option<T>` é uma enum que pode ser `Some(T)` (quando há um valor) ou `None` (quando não há valor). O `match` é muito útil para lidar com esse tipo:

```rust
fn main() {
    let opcao: Option<i32> = Some(5);

    match opcao {
        Some(valor) => println!("O valor é: {}", valor),
        None => println!("Não há valor."),
    }
}
```

### Explicação:
- O `match` verifica se `opcao` é `Some(valor)` (ou seja, contém um valor) ou `None` (não contém valor).
- Se for `Some(valor)`, o valor é extraído e usado. Caso contrário, o bloco correspondente a `None` é executado.

### Vantagens do `match`:
1. **Segurança em tempo de compilação**: Rust garante que todos os casos sejam tratados. Se você esquecer um caso, o compilador vai reclamar.
2. **Desestruturação poderosa**: Você pode usar `match` para desestruturar valores complexos, como enums com vários campos ou até tuplas.
3. **Flexibilidade**: Você pode combinar padrões e capturar apenas partes do valor, permitindo lógica complexa de maneira declarativa.

O `match` é fundamental em Rust para lidar com tipos que podem representar diferentes "estados" (como `Option` ou `Result`), além de ser uma maneira segura e eficiente de lidar com a lógica condicional.