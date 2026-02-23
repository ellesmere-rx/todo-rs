use std::env;

fn main() {
    // Получаем аргументы в виде итератора и преобразуем его в коллецию Vec
    // Пропускаем первый аргумент, т.к. он является путём к исполняемому файлу
    let args: Vec<String> = env::args().skip(1).collect();

        // Проверка на количество аргументов
    if args.len() < 1 {
        println!("Not enough arguments!");
    }

    // Матчинг команд
    match args[0].as_str() {
        // Добавление задачи
        "add" => {
            println!("You called add");
        }
        // Удаление задачи
        "del" => {
            println!("You called del");
        }
        // Просмотреть задачи
        "show" => {
            println!("You called show");
        }
        // Отметить задачу
        "mark" => {
            println!("You called mark");
        }
        // Сохранить задачи в файл
        "save" => {
            println!("You called save");
        }
        // Неизвестная задача
        _ => {
            println!("Unknown command");
        }
    }
}
