use std::io::stdin;

/*
Функции для задачи: .abs(), .sin(), .pow()
*/

//Функция main служит для записи ввода в переменные и коммуникации с пользователем
fn main() {
     //инициализация петли
    println!("Введите значение переменных в порядке a, b, c, d:"); //вывод сообщения с просьбой ввести значение переменных

    let a:f64 = read_var();
    let b:f64 = read_var();
    let c:f64 = read_var();
    let d:f64 = read_var(); //создание переменных

    let a: f64 = calc(a, b, c, d); //переменная a приравнивается к функции calc от переменных
    println!("{a}");
    pause(); //использование фунции паузы
}

fn calc(a: f64, b: f64, c: f64, d: f64) -> f64 {
(a/b * b/d) - (a*b - c)/(c*d) //функция calc выполняет (a/c * b/d) - (a*b - c)/c*d
}
//Функция read_var при вызове считывает ввод и передаёт его как выражение типа f64
fn read_var() -> f64 {
    loop {
        let mut x =String::new();
        stdin()
            .read_line(&mut x)
            .expect("Ошибка чтения ввода"); //считывание ввода и запись его в переменную X (с точки зрения компилятора это одна строка)

        let x: f64 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Переменная должна быть равна числу");
                continue;
            },
        }; //преобразование ввода со string в float-point integer с перезапуском loop в случае ошибки
        break x;
    }
}

fn pause() { //фукция паузы
    println!("нажмите Enter чтобы выйти.");
    let mut q = String::new();
    stdin().read_line(&mut q).expect("ошибка");
}