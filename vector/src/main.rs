use vector::core::domain::vector::MyVector;

fn main() {
    let mut vec = MyVector::with_capacity(2);
    println!("Init: len={}, cap={}", vec.len(), vec.capacity());

    vec.reserve(10);
    println!("Reserve: len={}, cap={}", vec.len(), vec.capacity());

    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("Push: len={}, cap={}", vec.len(), vec.capacity());

    vec.insert(1, 100);

    print!("Insert: ");
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!("");

    let removed = vec.remove(1);
    println!("Remove: {}", removed);

    print!("Elements: ");
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!("");

    println!("Extend [4, 5]");
    vec.extend([4, 5]);

    print!("Elements: ");
    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!("");

    vec.shrink_to_fit();
    println!("Shrink: len={}, cap={}", vec.len(), vec.capacity());

    println!("Pop:");
    while let Some(x) = vec.pop() {
        print!("{} ", x);
    }
    println!("");

    println!("Push 100");
    vec.push(100);

    vec.clear();
    println!("Clear: len={}, is_empty={}", vec.len(), vec.is_empty());
}
