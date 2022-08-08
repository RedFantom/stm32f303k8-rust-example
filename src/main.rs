#![no_std]
#![no_main]
/** @brief stm32f303k8-rust-example
*      Rust code for blink & serial communication on STM32F303K8 Nucleo
* @copyright Copyright (c) 2022 RedFantom
* @license GNU GPLv3
*/
use core::fmt::Write;
use panic_halt as _;
use stm32f3xx_hal::{prelude::*, self as hal, pac};

type SerialType = hal::serial::Serial<
    pac::USART2, (hal::gpio::PA2<hal::gpio::AF7<hal::gpio::PushPull>>, hal::gpio::PA15<hal::gpio::AF7<hal::gpio::PushPull>>)>;


#[cortex_m_rt::entry]
fn main () -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.sysclk(8.MHz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    // Configure the LED_BUILTIN equivalent pin
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Setup Serial using the Virtual COM Port pins
    // See the schematics found here:
    // <https://www.st.com/resource/en/user_manual/um1956-stm32-nucleo32-boards-mb1180-stmicroelectronics.pdf>
    let mut pins = (
        gpioa.pa2.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl),
        gpioa.pa15.into_af_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrh)
        );
    pins.1.internal_pull_up(&mut gpioa.pupdr, true);
    let mut serial: SerialType = hal::serial::Serial::new(
        dp.USART2, pins, 19200.Bd(), clocks, &mut rcc.apb1);

    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(8_000_000);
        led.set_low().unwrap();
        cortex_m::asm::delay(8_000_000);
        serial.write_str("Hello, world!\n");
    }
}
