// https://rust-exercises.com/100-exercises/03_ticket_v1/09_heap.html

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
// STRING:
//       +---------+--------+----------+
// Stack | pointer | length | capacity | 
//       |  |      |   0    |    5     |
//       +--|------+--------+----------+
//          |
//          |
//          v
//        +---+---+---+---+---+
// Heap:  | ? | ? | ? | ? | ? |
//        +---+---+---+---+---+

// s.push_str("Hey");
//       +---------+--------+----------+
// Stack | pointer | length | capacity |
//       |  |      |   3    |    5     |
//       +--|  ----+--------+----------+
//          |
//          |
//          v
//        +---+---+---+---+---+
// Heap:  | H | e | y | ? | ? |
//        +---+---+---+---+---+

// std::mem::size_of returns the amount of space a type would take on the STACK, 
// which is also known as the SIZE OF THE TYPE.
// What about the memory buffer that String is managing on the HEAP? 
// Isn't that part of the SIZE OF String? No!
// That HEAP allocation is a RESOURCE that String is managing. 
// It's not considered to be part of the String type by the compiler.
// std::mem::size_of doesn't know (or care) about additional HEAP-allocated data
// that a type might manage or refer to via pointers, as is the case with String, 
// therefore it doesn't track its size.

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}

// Unfortunately there is no equivalent of std::mem::size_of to measure 
// the amount of HEAP memory that a certain value is allocating at runtime. 
// Some types might provide methods to inspect their heap usage (e.g. String's 
// capacity method), but there is no general-purpose "API" to retrieve runtime 
// heap usage in Rust.
// You can, however, use a memory profiler tool (e.g. 
// [DHAT](https://valgrind.org/docs/manual/dh-manual.html) or a 
// [custom allocator](https://docs.rs/dhat/latest/dhat/)) to inspect the heap 
// usage of your program.