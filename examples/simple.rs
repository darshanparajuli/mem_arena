extern crate mem_arena;

use mem_arena::*;

#[derive(Debug)]
struct Test {
    a: u64,
    b: u32,
}

fn main() {
    // Allocate 48 bytes
    let mut arena = MemArena::alloc(48);

    // Push a struct
    let test = arena.push::<Test>();
    test.a = 12;
    test.b = 89;

    println!("{:?}", test);

    // Push an array of size 2 (size greater than 2 will panic as the arena does not have enough
    // space)
    let array = arena.push_array::<Test>(2);
    array[0].a = 12;
    array[1].b = 13;

    println!("{:?}", array);

    // Reset the arena, this simply resets the offset, previously pushed data still exists
    arena.reset();
    let array = arena.push_array::<Test>(3);
    println!("{:?}", array);

    // Clear the arena to zero
    arena.clear();
    let array = arena.push_array::<Test>(3);
    println!("{:?}", array);
}
