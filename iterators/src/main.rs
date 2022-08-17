fn main() {
    iter1();
    iter2();
    iter3();
    iter4();
}

fn iter1() {
    let input = [1, 2, 3];
    let iterator = input.iter();
    let mapped = iterator.map(|&x| x * 2);
    let output = mapped.collect::<Vec<usize>>();
    println!("{:?}", output);
}

fn iter2() {
    let vals = [1, 2, 3, 4, 5];
    let mut iter = vals.iter();
    println!("{:?}", iter.next().unwrap());
    println!("{:?}", iter.skip(2).take(2).collect::<Vec<_>>())
}

fn iter3() {
    let input = [1, 2, 3];
    let iterator = input.iter();
    let mapped = iterator
        .inspect(|&x| println!("Predefined map:\t{}", x))
        .map(|&x| x * 10)
        .inspect(|&x| println!("First map:\t{}", x))
        .map(|x| x + 5)
        .inspect(|&x| println!("Second map:\t{}", x));

    println!("{:?}", mapped.collect::<Vec<usize>>());
}

fn iter4() {
    let input = [1, 2, 3];
    let cycled = input.iter().cycle();
    let output = cycled.take(9).collect::<Vec<&usize>>();

    println!("{:?}", output);

}
