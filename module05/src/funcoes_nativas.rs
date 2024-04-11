#[test]
fn broken_test_assert() {
    let condition = true;
    // Corrija o uso de assert! para que o teste não panique.
    assert!(!condition);
}

#[test]
fn broken_test_format() {
    let name = "Rust";
    let formatted = format!("Hello, {}!", name);
    // Corrija a string formatada para que o teste passe.
    assert_eq!(formatted, "Goodbye, Rust!");
}

#[test]
fn broken_test_vec_macro() {
    let numbers = vec![1, 2, 3];
    // Corrija o vetor para que o teste passe.
    assert_eq!(numbers.len(), 4);
}

#[test]
#[should_panic]
fn broken_test_panic() {
    // Remova o comentário e corrija para que o teste passe esperando um panic.
    // panic!("This should panic");
}

#[test]
fn broken_test_dbg_macro() {
    let value = 42;
    let _ = dbg!(value);
    // Corrija o teste para refletir o uso correto de dbg!.
    assert_eq!(value, 42);
}

#[test]
fn broken_test_todo_macro() {
    // Implemente o teste para que ele passe, removendo ou corrigindo o uso de todo!.
    // todo!();
    assert!(true);
}
