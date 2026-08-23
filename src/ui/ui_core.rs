//входной поток или типа того, Write дает flush
use std::io::{self, Write};
// позволяет запустить что-то другое как отдельный процесс в ОС
use std::process::Command;

use crate::domain::user::{BuyError, User};
use crate::repository::products_repository::ProductsRepository;

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
    // чтобы вывелся текст, а только потом ввод. flush выводит буфер на экран, не дожидаясь следующего \n.
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
pub fn run_cli(user: &mut User, catalog: &mut ProductsRepository) {
    // начальный экран, будем мутировать его в цикле
    let mut screen = Screen::Main;

    // бесконечный цикл (до break)
    loop {
        // чистим экран от предыдущего стейта
        clear();
        // переключатель экранов по вводу пользователя
        screen = match screen {
            Screen::Main => main_screen(user),
            Screen::Catalog => catalog_screen(user, catalog),
            Screen::Exit => {
                println!("Bye!");
                break;
            }
        }
    }
}

fn main_screen(user: &User) -> Screen {
    println!("=== Main Menu ===\n");
    println!(
        "Пользователь: {}, баланс: {:.2}\n",
        user.get_name(),
        user.get_balance()
    );
    println!("0. Exit");
    println!("1. Catalog");

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

fn catalog_screen(user: &mut User, catalog: &ProductsRepository) -> Screen {
    println!("=== Catalog ===\n");

    let items = catalog.get_list();

    if items.is_empty() {
        println!("No products in catalog now.\n");
    } else {
        // iter позволяет пользоваться вектором после выполнения цикла, без него цикл поглотит его
        // enumerate выдает каждому элементу индекс, зачем цикл использует его (i)
        for (i, product) in items.iter().enumerate() {
            // :.2 == два знака после запятой
            println!(
                "{}. {} — {:.2} ({})",
                i + 1,
                product.get_name(),
                product.get_price(),
                product.get_description()
            );
        }
        println!();
    }

    println!("Введите номер товара, чтобы купить его.");
    println!("0. To Main Menu");

    let choice = prompt("> ");

    if choice == "0" {
        return Screen::Main;
    }

    // преобразует строки в числовые типы, нужный выводится или указывается в турбофише
    let index = match choice.parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            println!("Некорректный ввод!");
            prompt("Press Enter to continue...");
            return Screen::Catalog;
        }
    };

    if index == 0 || index > items.len() {
        println!("Нет товара с таким номером!");
        prompt("Press Enter to continue...");
        return Screen::Catalog;
    }

    let product = items[index - 1];

    match user.buy(product) {
        Ok(()) => println!(
            "Куплено: {}! Остаток на балансе: {:.2}",
            product.get_name(),
            user.get_balance()
        ),
        Err(BuyError::NotEnoughMoneyError) => println!("Недостаточно средств на балансе!"),
    }
    prompt("Press Enter to continue...");

    Screen::Catalog
}
