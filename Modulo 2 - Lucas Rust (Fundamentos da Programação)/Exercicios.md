# Variáveis em Rust

- Em Rust, uma **variável** é um identificador associado a um valor que pode ser usado em seu programa.
- Por padrão, as variáveis em Rust são imutáveis, o que significa que uma vez que um valor é atribuído a uma variável, ele não pode ser alterado.
- A imutabilidade é uma das muitas garantias de segurança que Rust oferece.

- Para declarar uma variável, utilizamos a palavra-chave `let`.
- Se você quiser que uma variável seja mutável você pode usar `let mut`.
- Rust também permite "shadowing", onde você pode declarar uma nova variável com o mesmo nome, efetivamente criando uma nova variável.

## Exercícios

```Rust
fn main() {

    // Como declarar variaveis
    // Variáveis são declaradas com `let` e seus valores são imutáveis por padrão.
    let x = 10;
    assert_eq!(x, 10);

    // Mudando o valor de uma variavel
    // Variáveis mutáveis podem ter seus valores alterados.
    let mut y = 5;
    y = y + 5;
    assert_eq!(y, 10);

    // Variavel shadowing
    // Rust permite shadowing de variáveis, o que significa redeclarar uma variável com o mesmo nome.
    let x = 5;
    let x = 95; // Shadowing: 'x' é agora 95, não 5.
    assert_eq!(x, 95);

}
fn test_aprendendo_sobre_tests() {
    // Testes ajudam a verificar se seu código está funcionando como esperado.
    // Este teste intencionalmente falhará, mostrando como o teste pode ser usado para pegar erros.
    let minha_var = 12;
    // Corrija o valor esperado para que o teste passe.
    assert_eq!(minha_var, 11); // O valor esperado deve ser igual ao valor da variável.
}
```
