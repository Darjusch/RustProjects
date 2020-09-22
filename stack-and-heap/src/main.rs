fn main() {
    let a = 2;
    stack_only(a);
}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32{
    let d = 5;
    let e = Box::new(7);
    return d + *e;
}