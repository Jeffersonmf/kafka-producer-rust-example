# kafka-producer-rust-example

### Documentação ainda é precária:

**mas vamos lá...**  

### Como Compilar

```
$ cargo update

$ cargo build --release
```

### Como Executar

```
via Cargo Runtime
$ cargo run "BROKER:PORTA" "TÓPICO" "MENSAGEM DO PAYLOAD" "A CHAVE DA MENSAGEM"

nativo
$ ./target/release/ultra_faster_producer "BROKER:PORTA" "TÓPICO" "MENSAGEM DO PAYLOAD" "A CHAVE DA MENSAGEM"
```
