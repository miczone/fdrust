extern crate fdutil;

fn _test() {
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
}

fn call_util() {
    let x: fdutil::array::heap::HeapList<u32> = fdutil::array::heap::HeapList::new(10);
    println!("{:?}", x);
}

fn main() {
    call_util();
}
