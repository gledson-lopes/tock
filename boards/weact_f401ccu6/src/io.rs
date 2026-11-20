// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::panic::PanicInfo;

use kernel::debug;
use kernel::hil::led;
use kernel::hil::uart;

use stm32f401cc::chip_specs::Stm32f401Specs;
use stm32f401cc::dma::Dma1;
use stm32f401cc::gpio::PinId;
use stm32f401cc::usart::{Usart, UsartPanicWriterConfig};

/// Panic handler.
#[panic_handler]
pub fn panic_fmt(info: &PanicInfo) -> ! {
    // On-board LED C13 is connected to PC13
    // Have to reinitialize several peripherals because otherwise can't access them here.
    let rcc = stm32f401cc::rcc::Rcc::new();
    let clocks: stm32f401cc::clocks::Clocks<Stm32f401Specs> =
        stm32f401cc::clocks::Clocks::new(&rcc);
    let syscfg = stm32f401cc::syscfg::Syscfg::new(&clocks);
    let exti = stm32f401cc::exti::Exti::new(&syscfg);
    let pin = stm32f401cc::gpio::Pin::new(PinId::PC13, &exti);
    let gpio_ports = stm32f401cc::gpio::GpioPorts::new(&clocks, &exti);
    pin.set_ports_ref(&gpio_ports);
    let led = &mut led::LedLow::new(&pin);

    debug::panic::<_, Usart<'static, Dma1<'static>>, _, _>(
        &mut [led],
        UsartPanicWriterConfig {
            registers: crate::PANIC_USART,
            params: uart::Parameters {
                baud_rate: 115200,
                stop_bits: uart::StopBits::One,
                parity: uart::Parity::None,
                hw_flow_control: false,
                width: uart::Width::Eight,
            },
        },
        info,
        &cortexm4::support::nop,
        crate::PANIC_RESOURCES.get(),
    )
}
