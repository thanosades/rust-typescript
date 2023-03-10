fn not_complete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }

    todo!("hey, finish this later"); 
}

fn main() {
    let foo = None;

    let bar: i32 = foo.unwrap();
}
