use core::panic::PanicInfo;
use crate::println;
use crate::sbi::shutdown;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "[kernel] Panicked at {}:{}",
            location.file(),
            location.line()
        );
    }else if let Some(message) = info.payload().downcast_ref::<&'static str>() {
        println!("[kernel] Panicked: {}", message);
    } else {
        println!("Panicked");
    }
    shutdown()
}