enum Color {
    Red,
    Yellow,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("It's Red"),
        Color::Yellow => println!("It's Red"),
        Color::Blue => println!("It's Red"),
        _ => println!("unknow color"),
    }
}

enum BuildingLocaltion {
    Number(i32),
    Name(String),
    Unkonw,
}

impl BuildingLocaltion {
    fn print_localtion(&self) {
        match self {
            BuildingLocaltion::Number(num) => println!("number: {num}"),
            BuildingLocaltion::Name(name) => println!("name: {name}"),
            _ => (),
        }
    }
}

enum Flavor {
    Spicy,
    Sweet,
    Solt,
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    fn print(&self) {
        match self.flavor {
            Flavor::Spicy => println!("splicy"),
            Flavor::Sweet => println!("sweet"),
            Flavor::Solt => println!("solt"),
        }
        println!("{}", self.price);
    }

    fn buy(&self) {
        if self.price > 10.0 {
            println!("I am poor");
        } else {
            println!("buy it");
        }
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn print_point(point: Point) {
    println!("print_point {:?}", point)
}

fn print_point_borrow(point: &Point) {
    println!("print_point_borrow {:?}", point)
}

fn func_copy_back() -> i32 {
    let n = 100;
    n
}

fn func_no_copy_back() -> String {
    let s = String::from("value");
    s
}

fn get_mess<'a>() -> &'a str {
    return "hello world";
}

fn mul_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}
fn mul(x: i32) -> i32 {
    x * x
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Result::Err(String::from("value"));
    }
    Result::Ok(a / b)
}

fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (i, v) in array.iter().enumerate() {
        if *v == target {
            return Some(i);
        }
    }
    None
}

fn find_first_even(numbers: Vec<i32>) -> Option<i32> {
    let first_even = numbers.iter().find(|&x| x % 2 == 0)?;
    println!("even:{}", first_even);
    Some(*first_even)
}

#[derive(Debug)]
struct MyError {
    detail: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.detail)
    }
}

impl std::error::Error for MyError {
    fn description(&self) -> &str {
        &self.detail
    }
}

fn my_error() -> Result<(), MyError> {
    Err(MyError {
        detail: "Custom Error".to_string(),
    })
}

fn main() {
    print_color(Color::Red);

    let loc = BuildingLocaltion::Number(100);
    loc.print_localtion();

    //
    println!("---------------------------------");
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 11.0,
    };
    sweet.print();
    sweet.buy();

    println!("box ------------------------------------");
    let boxed_point = Box::new(Point { x: 10, y: 20 });
    println!("point x:{} y:{}", boxed_point.x, boxed_point.y);

    let mut boxed_i = Box::new(100);
    *boxed_i += 10;
    println!("i:{}", boxed_i);

    println!("clone ------------------------------------");
    let point1 = Point { x: 10, y: 20 };
    let point2 = point1.clone();
    println!("{:?}, {:?}", point1, point2);

    println!("fn ------------------------------------");
    let point = Point { x: 10, y: 20 };
    print_point_borrow(&point);
    print_point(point);

    println!("back ------------------------------------");
    let i = func_copy_back();
    let s = func_no_copy_back();
    println!("{i}, {s}");

    let s = get_mess();
    println!("{s}");

    println!("major fn ------------------------------------");
    let v = mul_twice(mul, 5);
    println!("{v}");

    let v = mul_twice(|x| x + x, 10);
    println!("{v}");

    let number = vec![1, 2, 3, 4, 5, 6, 7];
    let res: Vec<_> = number.iter().map(|x| x + x).collect();
    println!("{:?}", res);

    let res: Vec<_> = number.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", res);

    let sum = number.iter().fold(0, |acc, &x| acc + x);
    println!("{:?}", sum);

    println!("result/option ------------------------------------");
    let ret = divide(10, 5);
    println!("{:?}", ret);
    let ret = divide(10, 0);
    println!("{:?}", ret);

    let array = [1, 2, 3, 4, 5, 6, 7, 8];
    match find_element(&array, 5) {
        Some(index) => println!("find index:{}", index),
        None => println!("Not Found"),
    }

    println!("unwrap ? ------------------------------------");
    let result_ok: Result<i32, &str> = Ok(32);
    let value = result_ok.unwrap();
    println!("{}", value);

    // let result_err: Result<i32, &str> = Err("error");
    // let value = result_err.unwrap();
    // println!("{}", value);

    let numbers = vec![1, 3, 5];
    match find_first_even(numbers) {
        Some(number) => println!("first even:{}", number),
        None => println!("no even"),
    }

    println!("error ------------------------------------");
    match my_error() {
        Ok(_) => println!("ok"),
        Err(err) => println!("Error: {}", err),
    }

    println!("borrow ------------------------------------");
    let mut s = String::from("helo");
    let s1 = &s;
    let s2 = &s;
    println!("{}, {}, {}", s, s1, s2);
    {
        let s3 = &mut s;
        //println!("{}, {}", s1, s3);
        println!("{}", s3);
    }

    println!("life ------------------------------------");
    fn no_need(s: &str) -> &str {
        s
    }
    fn need<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        s1
    }
    println!("{}", no_need("hello"));
    println!("{}", need("hello", "world"));

    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            return s1;
        }
        return s2;
    }
    println!("longest {}", longest("hello", "world"));

    fn longest_str<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'out str
    where
        'a: 'out,
        'b: 'out,
    {
        if s1.len() > s2.len() {
            return s1;
        }
        return s2;
    }
    println!("longest_str {}", longest_str("hello", "world"));

    struct MyString<'a> {
        text: &'a str,
    }
    impl<'a> MyString<'a> {
        fn get_length(&self) -> usize {
            self.text.len()
        }

        fn modiy_data(&mut self) {
            self.text = "world";
        }
    }
    let str1 = String::from("value");
    let mut x = MyString {
        text: str1.as_str(),
    };
    x.modiy_data();
    println!("{}", x.text);
}
