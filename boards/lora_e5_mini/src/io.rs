// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2025.

use core::panic::PanicInfo;

use kernel::debug;
use kernel::hil::led;
use kernel::hil::led::Led;
use stm32wle5jc::chip_specs::Stm32wle5jcSpecs;
use stm32wle5jc::gpio::PinId;
use stm32wle5jc::usart::{Usart, UsartPanicWriterConfig};

/// Panic handler.
#[panic_handler]
pub fn panic_fmt(info: &PanicInfo) -> ! {
    // For now we add a loop to blink the LED to an interesting way.
    // To ensure that all dependencies are set up, we initialize all clocks
    // and GPIOs here in this function.
    //
    // Relying on `main.rs` to initialize clocks/gpios may result in the gpio
    // not being properly configured if the panic occurs early in `main.rs`.
    let rcc = stm32wle5jc::rcc::Rcc::new();
    let clocks: stm32wle5jc::clocks::Clocks<Stm32wle5jcSpecs> =
        stm32wle5jc::clocks::Clocks::new(&rcc);
    let syscfg = stm32wle5jc::syscfg::Syscfg::new();
    let exti = stm32wle5jc::exti::Exti::new(&syscfg);

    let gpio_ports = stm32wle5jc::gpio::GpioPorts::new(&clocks, &exti);
    gpio_ports.setup_circular_deps();
    gpio_ports
        .get_port_from_port_id(stm32wle5jc::gpio::PortId::B)
        .enable_clock();
    let pin = stm32wle5jc::gpio::Pin::new(PinId::PB05, &exti);
    pin.set_ports_ref(&gpio_ports);
    let led = &mut led::LedLow::new(&pin);
    led.init();

    debug::panic_print::<Usart, crate::ChipHw, crate::ProcessPrinterInUse>(
        UsartPanicWriterConfig {
            registers: crate::PANIC_USART,
        },
        info,
        &cortexm4::support::nop,
        crate::PANIC_RESOURCES.get(),
    );

    // Unique LED blink pattern for panic
    loop {
        led.on();
        // Wait for LONG
        delay_long();

        led.off();
        delay_short();

        // SHORT
        led.on();
        delay_short();

        led.off();
        delay_short();

        // SHORT
        led.on();
        delay_short();

        led.off();
        delay_short();

        // LONG
        led.on();
        delay_long();

        led.off();
        delay_long();
    }
}

fn delay_long() {
    for _ in 0..1_000_000 {
        cortexm4::support::nop();
    }
}

fn delay_short() {
    for _ in 0..100_000 {
        cortexm4::support::nop();
    }
}
