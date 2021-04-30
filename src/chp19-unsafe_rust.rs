fn main() {
    // creating raw pointers
    {
        // from known memory location
        let mut num = 5;

        let _r1 = &num as *const i32;
        let _r2 = &mut num as *mut i32;

        // from arbitrary memory location
        let address = 0x012345usize;
        let _r = address as *const i32;
    }

    // dereferencing raw pointers (requires unsafe block)
    {
        let mut num = 5;

        let _r1 = &num as *const i32;
        let _r2 = &mut num as *mut i32;

        unsafe {
            println!("_r1 is: {}", *_r1);
            println!("_r2 is: {}", *_r2);
        }
    }

    // unsafe functions
    {
        unsafe fn danger_zone() {}

        unsafe {
            danger_zone();
        }
    }

    // creating safe abstraction over unsafe code
    {
        use std::slice;

        fn _split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();

            assert!(mid <= len);

            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
    }

    // accessing and modifying static variable
    {
        static mut COUNTER: u32 = 0;

        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }

    // implementing an unsafe trait
    {
        unsafe trait Foo {}

        unsafe impl Foo for i32 {}
    }
}
