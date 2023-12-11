# Variáveis em Rust

- Em Rust, uma **variável** é um identificador associado a um valor que pode ser usado em seu programa.
- Por padrão, as variáveis em Rust são imutáveis, o que significa que uma vez que um valor é atribuído a uma variável, ele não pode ser alterado.
- A imutabilidade é uma das muitas garantias de segurança que Rust oferece.

- Para declarar uma variável, utilizamos a palavra-chave `let`.
- Se você quiser que uma variável seja mutável você pode usar `let mut`.
- Rust também permite "shadowing", onde você pode declarar uma nova variável com o mesmo nome, efetivamente criando uma nova variável.

## Extras

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

#[test]
fn test_aprendendo_sobre_tests() {
    // Testes ajudam a verificar se seu código está funcionando como esperado.
    // Este teste intencionalmente falhará, mostrando como o teste pode ser usado para pegar erros.
    let minha_var = 12;
    // Corrija o valor esperado para que o teste passe.
    assert_eq!(minha_var, 11); // O valor esperado deve ser igual ao valor da variável.
}
```

# Como dar nome às coisas?

Em Rust usamos "snake_case" como padrão para variáveis e funções, por exemplo:

**Exemplos Corretos:**

- nome_de_uma_variavel
- total_de_itens
- preco_do_cafe

**Exemplos Incorretos:**

- NomeDeUmaVariavel (isto é "PascalCase", usado para tipos em Rust)
- outroNomeEscritoComCamelCase (isto é "camelCase", comumente usado em outras linguagens como JavaScript)
- variável_com_acento (acentos e caracteres especiais não são recomendados)
- 1_inicial (números não devem ser usados no início dos nomes de variáveis)

**Dicas:**

- Seja descritivo, mas conciso.
- Os nomes devem refletir o propósito da variável ou o valor que ela guarda.
- Evite usar "números mágicos" diretamente no código; dê-lhes um nome significativo.

# Extras

```Rust
#[test]
fn test_variavel_de_letra_unica() {
    let a = "Valor";
    // Este teste falha porque estamos tentando desencorajar o uso de nomes de variáveis de uma letra.
    assert!(false, "Utilize um nome mais descritivo em vez de `a`.");
}

#[test]
fn test_variavel_com_numero() {
    let var123 = "Número da Conta";
    // Este teste falha porque números no nome da variável devem ter um significado claro.
    assert!(
        false,
        "Utilizar um nome descritivo sem números ou com números significativos."
    );
}

#[test]
fn test_variavel_com_acento() {
    let variável = "portugues";
    // Este teste falha porque acentos no nome da variável podem causar "erros".
    assert!(false, "Utilizar um nome descritivo em ingles.");
}

#[test]
fn test_variavel_caso_misto() {
    let Xx = "Algum valor";
    // Este teste falha porque estamos tentando desencorajar o uso de mistura de maiúsculas e minúsculas sem um padrão claro.
    assert!(false, "Use snake_case para nomes de variáveis em Rust.");
}

#[test]
fn test_variavel_caso_de_camelo() {
    let helloWorld = "Olá Mundo";
    // Este teste falha porque não estamos seguindo a convenção snake_case.
    assert!(
        false,
        "Os nomes das variáveis devem estar em snake_case e não em camelCase."
    );
}

#[test]
fn test_variavel_nao_descritiva() {
    let minha_casa = 122000;
    // Este teste falha porque o nome da variável não é descritivo.
    assert!(
        false,
        "Escolha um nome para a variável que descreva o seu objetivo ou conteúdo."
    );
}

#[test]
fn test_sem_variavel_numero_magico() {
    // Este teste falha porque um valor sem variável torna o código confuso
    // Neste caso, o número 4.90 deveria ser uma variável `dolar_price`.

    assert!(
        4.90 * 100.00 == 490.00,
        "Escolha um nome para a variável para o 4.90"
    );
}
```

# Bits & Bytes

- 1 bit é a unidade básica de informação na computação e pode ter um de dois valores, 0 ou 1.
- 1 byte == 8 bits.

Ou seja 1 byte pode representar 256 valores diferentes (2^8), desde 0 até 255 em decimal.
A relação entre bits e bytes pode ser confusa mesmo, mas pense o seguinte:

- 1 duzia == 12 ovos
- 1 hora == 60 mintos
- 1 bytes == 8 bits

Os computadores armazenam todos os dados como bits. e usam bytes para representar informações maiores.
Há duas arquiteturas de computadores principais quanto ao armazenamento de bytes:

- **Big-endian**: os bytes mais significativos (o "big end") são armazenados primeiro.
- **Little-endian**: os bytes menos significativos (o "little end") são armazenados primeiro.

Problemas de segurança

- **Overflow** ocorre quando tentamos armazenar um número que é maior do que o máximo permitido pelo tipo de dados, resultando no ciclo para o menor valor possível.
- **Underflow** ocorre quando tentamos armazenar um número menor do que o mínimo permitido pelo tipo de dados, resultando no ciclo para o maior valor possível.

```Rust
#[test]
fn test_bit_byte_representation() {
    let one_bit: u8 = 0b0000_0001;
    assert_eq!(one_bit.count_ones(), 1);

    let one_byte: u8 = 0b1111_1111;
    assert_eq!(one_byte, 255);
}
#[test]
fn test_arch_representation() {
    let big_endians_bytes_5 = 5u32.to_be_bytes();
    assert_eq!(big_endians_bytes_5, [0, 0, 0, 5]);

    let little_endians_bytes_5 = 5u32.to_le_bytes();
    assert_eq!(little_endians_bytes_5, [5, 0, 0, 0]);
}

#[test]
fn test_overflow_underflow() {
    let underflow_number = 0u32.wrapping_sub(1);
    assert_eq!(underflow_number.to_be_bytes(), [255, 255, 255, 255]);

    let overflow_number = 255u8.wrapping_add(1);
    assert_eq!(overflow_number.to_be_bytes(), [0]);
}

#[test]
fn broken_test_byte_value() {
    // let byte: u8 = 0b1002_0001;
    // Este código contém um byte inválido (números binários são apenas 0 e 1).
    // assert_eq!(byte, 33);
}

#[test]
fn broken_test_endianness() {
    let value = 258u16;
    let big_endian_bytes = value.to_be_bytes();
    // O assert está incorreto porque os bytes estão na ordem errada. Corrija-o para que o teste passe.
    assert_eq!(big_endian_bytes, [1, 2]);
}

#[test]
fn broken_test_overflow() {
    // let big_number: u8 = 250u8;
    // let result = big_number + 10;
    // Este teste causará um overflow sem o método wrapping_add, altere-o para corrigir o problema.
    // assert_eq!(result, 4);
}

#[test]
fn broken_test_underflow() {
    // let small_number: i8 = -128i8;
    // let result = small_number - 1;
    // Este teste causará um underflow sem o método wrapping_sub, altere-o para corrigir o problema.
    // assert_eq!(result, 127);
}

#[test]
fn broken_test_bit_count() {
    let bits: u8 = 0b0101_0101;
    // A função de contagem está errada, encontre a função correta que conta o número de bits 1.
    assert_eq!(bits.count_zeros(), 4);
}
```

# Os tipos são seus amigos mano

Rust é uma linguagem de programação com tipagem estática e forte, o que significa que:

Tipagem estática:
- O tipo de cada variável é determinado em tempo de compilação e não muda.
- Uma variável declarada com um tipo específico não pode ser reatribuída a um valor de outro tipo sem uma conversão explícita.

Tipagem forte:
- Rust é rigoroso com as operações entre tipos.
- Não é possível transformar diretamente um número e uma string sem converter explicitamente um deles para o tipo compatível do outro.

O compilador de Rust é inteligente o suficiente para inferir o tipo de muitas variáveis, o que significa que nem sempre é necessário especificar o tipo explicitamente.


```Rust
#[test]
fn test_integer_types() {
    let small_int: i8 = -128;
    let big_uint: u128 = 340282366920938463463374607431768211455;
    assert_eq!(small_int, -128);
    assert_eq!(big_uint, 340282366920938463463374607431768211455);
}

#[test]
fn test_float_types() {
    let float_num: f32 = 3.14;
    let double_num: f64 = 2.71828;
    assert!(float_num - 3.14 < f32::EPSILON);
    assert!(double_num - 2.71828 < f64::EPSILON);
}

#[test]
fn test_boolean_type() {
    let is_rust_awesome: bool = true;
    assert_eq!(is_rust_awesome, true);
}

#[test]
fn test_char_type() {
    let letter: char = 'R';
    assert_eq!(letter, 'R');
}

#[test]
fn test_tuple_type() {
    let tuple: (i32, f64, char) = (500, 6.4, '🚀');
    assert_eq!(tuple, (500, 6.4, '🚀'));
}

#[test]
fn test_array_type() {
    let array: [i32; 3] = [1, 2, 3];
    assert_eq!(array, [1, 2, 3]);
}

#[test]
fn test_size_of_integer_types() {
    assert_eq!(std::mem::size_of::<i8>(), 1);
    assert_eq!(std::mem::size_of::<u128>(), 16);
}

#[test]
fn test_size_of_float_types() {
    assert_eq!(std::mem::size_of::<f32>(), 4);
    assert_eq!(std::mem::size_of::<f64>(), 8);
}

#[test]
fn test_size_of_bool_type() {
    assert_eq!(std::mem::size_of::<bool>(), 1);
}

#[test]
fn test_size_of_char_type() {
    assert_eq!(std::mem::size_of::<char>(), 4);
}

#[test]
fn test_size_of_composite_types_using_padding() {
    assert_eq!(std::mem::size_of::<(i32, f64)>(), 16);
    assert_eq!(std::mem::size_of::<(char, i32, i128)>(), 32);
    assert_eq!(std::mem::size_of::<[i32; 3]>(), 12);
}

#[test]
fn broken_test_integer_overflow() {
    // let num: u8 = 255;
    // Corrija o seguinte código para evitar o overflow e faça o teste passar.
    // let result = num + 1;
    // assert_eq!(result, 0);
}

#[test]
#[ignore]
fn broken_test_type_mismatch() {
    let boolean: bool = true;
    // Corrija o tipo esperado no assert para que o teste passe.
    assert_eq!(boolean, false);
}

#[test]
fn broken_test_tuple_access() {
    let my_tuple: (i32, f64, char) = (500, 6.4, '🚀');
    // Corrija o acesso ao elemento da tupla para que o teste passe.
    assert_eq!(1.1, 6.4);
}

#[test]
fn broken_test_array_size() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    // Corrija o tamanho esperado do array para que o teste passe.
    assert_eq!(std::mem::size_of_val(&array), 0);
}
```
