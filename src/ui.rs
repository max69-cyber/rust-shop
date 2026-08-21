use std::io;

// cli ui для приложения
enum Screen {
    Main,
    Catalog,
    Exit,
}

// Принимает строковый ввод пользователя в консоли.
pub fn prompt(text: &str) -> String {
    // то, что показать перед полем ввода
    println!("{}", text);

    // заготовка под вводимую строку
    let mut buf = String::new();

    // читаем через системную либу, read_line зачитает строку до enter и вернет число байт или ошибку
    let read_result = io::stdin().read_line(&mut buf);
    // выводим ошибку, если произошла
    match read_result {
        Ok(_) => {}
        Err(err) => {
            println!("Error while accepting input from user: {}", err);
        }
    };

    // возвращаем обрезаем и преобразуем обратно в String + return
    buf.trim().to_string()
}
