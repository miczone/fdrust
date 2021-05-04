#![allow(dead_code)]

pub fn _test() {
    let mut x: u32 = 1;
    {
        println!("shadowing 1");
        let x: i32 = 0;
        println!("{}", x);
    }

    x += 5;
    println!("{}", x);

    let y: [i32; 4] = [11,2,3,4];
    println!("{:?} => {:?}", y[0], y[3]);

    const LEN: usize = 500;
    let z: [i32; LEN] = [101; LEN];
    println!("{:?}", z);

    let mut v: Vec<u32> = vec![0; LEN];
    v.clear();
    let vp = &v;
    println!("{:?}", v);
    println!("{:?}", vp);

    let n: (i32, i32) = (1, 2);
    println!("{:?}", n);

    println!("Hello, world!");

    #[derive(Debug)]
    pub enum ETest {
        One = 1,
        Two = 2
    }
    println!("{}", ETest::One as i32);

    let u: Box<u32> = Box::new(3u32);
    println!("{:?}", u);

    let y: Box<u32> = u;
    _swap(y);
}

pub fn _swap(mut x : Box<u32>) {
    *x *= 4;
    println!("{:?}", x);
}

pub fn _out1() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 20,
    };

    let Person { ref name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's age from person struct is {}", person.age);
    println!("The name from person struct is {}", person.name);
}

#[derive(Debug, Clone, Copy)]
pub struct Person {
    name: &'static str,
    email: &'static str,
    year: u32,
    fullname: &'static str,
}

pub fn _ex0(x: &Person) {
    println!("{} -> {} -> {} -> {}", x.name, x.email, x.year, x.fullname);
}

pub fn _ex(x: &mut Person) {
    x.year += 1;
    x.fullname = "HoaPT";
}

pub fn _out2() {
    let p1 = Person {
        name: "Menfusu",
        email: "xxxx@gmail.com",
        year: 1979,
        fullname: "Menfusu PT"
    };

    let mut p2 = p1;
    _ex0(&p1);
    _ex0(&p2);

    _ex(&mut p2);
    _ex0(&p2);
}

pub fn call_util() {
    let x: fdutil::array::heap::HeapList<u32> = fdutil::array::heap::HeapList::new(10);
    println!("{:?}", x);

    let s = fdutil::helper::string::remove_whitespace("Hello word!");
    println!("Len of string ({}) is {}", s, s.len());
}

#[derive(Debug, Clone)]
struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) { self.0 += 1; }
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

pub fn _out3() {
    let mut owner: Owner = Owner(21);
    owner.add_one();
    owner.print();
    println!("{:?}", owner);

    let mut owner1: Owner = owner.clone();
    owner1.add_one();
    owner1.print();
    println!("{:?}", owner1);
}