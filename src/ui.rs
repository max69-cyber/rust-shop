//входной поток или типа того, Write дает flush
use std::io::{self, Write};
// позволяет запустить что-то другое как отдельный процесс в ОС
use std::process::Command;

/// cli ui для приложения
enum Screen {
    Main,
    Catalog,
    Exit,
}

/// Принимает строковый ввод пользователя в консоли.
pub fn prompt(text: &str) -> String {
    // то, что показать перед полем ввода
    print!("{}", text);
    // чтобы вывелся текст, а только потом ввод
    io::stdout().flush().unwrap();

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

/// Обновление экрана (очистка консоли)
pub fn clear() {
    // вызываем clear (на винду пока забьем), проверяем что получилось
    match Command::new("clear").status() {
        Ok(_) => {}
        // если не получилось то напишем
        Err(err) => {
            println!("Error while updating screen: {}", err);
        }
    }
}

/// запускает cli-интерфейс
pub fn run_cli() {
    // начальный экран, будем мутировать его в цикле
    let mut screen = Screen::Main;

    // бесконечный цикл (до break)
    loop {
        // чистим экран от предыдущего стейта
        clear();
        // переключатель экранов по вводу пользователя
        screen = match screen {
            Screen::Main => {
                // выведем название экрана и опции действий
                println!("=== Main Menu ===\n");
                println!("0. Exit");
                println!("1. Catalog");

                // принимаем инпут с желаемым действием
                match prompt("> ").as_str() {
                    "0" => Screen::Exit,
                    "1" => Screen::Catalog,
                    _ => {
                        println!("No such option in actions list!");
                        prompt("Press Enter to continue...");
                        Screen::Main
                    }
                }
            }
            Screen::Catalog => {
                clear();

                println!("=== Catalog ===");
                println!("No products in catalog now.\n");

                println!("0. To Main Menu");

                match prompt("> ").as_str() {
                    "0" => Screen::Main,
                    _ => {
                        println!("No such option in actions list!");
                        prompt("Press Enter to continue...");
                        Screen::Catalog
                    }
                }
            }
            Screen::Exit => {
                println!("Bye!");
                break;
            }
        }
    }
}
