fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut smallest = input[0];
    let mut largest = input[0];
    for &num in &input {
        if num < smallest
        smallest = num;

    }
    if num > largest {
        largest = num;
    }

    println!("{} is largest and {} is smallest",largest,smallest);
}
