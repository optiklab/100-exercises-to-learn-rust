// https://rust-exercises.com/100-exercises/03_ticket_v1/10_references_in_memory.html

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    // https://doc.rust-lang.org/reference/type-layout.html#pointers-and-references-layout

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8); // Pointers to sized types have the same size and alignment as usize.
        // However, on a 32 bit target, this is 4 bytes, and on a 64 bit target, this is 8 bytes.
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8); // Pointers to unsized types are sized. 
        // The size and alignment is guaranteed to be at least equal to the size and alignment of a pointer.
    }
}
