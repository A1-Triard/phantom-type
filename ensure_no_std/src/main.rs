#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

mod no_std {
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }

    #[no_mangle]
    extern "C" fn rust_eh_personality() { }
}

use educe::Educe;
use phantom_type::*;

struct NonClonable;

#[derive(Educe)]
#[educe(Clone, Copy)]
struct Test<T>(PhantomType<T>);

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let test: Test<NonClonable> = Test(PhantomType::new());
    let _test_clone = test.clone();
    0
}
