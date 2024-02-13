# Variáveis em Rust

- Em Rust, uma **variável** é um identificador associado a um valor que pode ser usado em seu programa.
- Por padrão, as variáveis em Rust são imutáveis, o que significa que uma vez que um valor é atribuído a uma variável, ele não pode ser alterado.
- A imutabilidade é uma das muitas garantias de segurança que Rust oferece.

- Para declarar uma variável, utilizamos a palavra-chave `let`.
- Se você quiser que uma variável seja mutável você pode usar `let mut`.
- Rust também permite "shadowing", onde você pode declarar uma nova variável com o mesmo nome, efetivamente criando uma nova variável.

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

# Os tipos são seus amigos mano

Rust é uma linguagem de programação com tipagem estática e forte, o que significa que:

**Tipagem estática**

- O tipo de cada variável é determinado em tempo de compilação e não muda.
- Uma variável declarada com um tipo específico não pode ser reatribuída a um valor de outro tipo sem uma conversão explícita.

**Tipagem forte**

- Rust é rigoroso com as operações entre tipos.
- Não é possível transformar diretamente um número e uma string sem converter explicitamente um deles para o tipo compatível do outro.

O compilador de Rust é inteligente o suficiente para inferir o tipo de muitas variáveis, o que significa que nem sempre é necessário especificar o tipo explicitamente.
