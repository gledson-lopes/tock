// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use core::panic::PanicInfo;

use kernel::debug;
use kernel::hil::led;
use kernel::hil::uart;

use stm32f412g::chip_specs::Stm32f412Specs;
use stm32f412g::dma::Dma1;
use stm32f412g::gpio::PinId;
use stm32f412g::usart::{Usart, UsartPanicWriterConfig};

/// Panic handler.
#[panic_handler]
pub fn panic_fmt(info: &PanicInfo) -> ! {
    // User LD2 is connected to PB07
    // Have to reinitialize several peripherals because otherwise can't access them here.
    let rcc = stm32f412g::rcc::Rcc::new();
    let clocks: stm32f412g::clocks::Clocks<Stm32f412Specs> = stm32f412g::clocks::Clocks::new(&rcc);
    let syscfg = stm32f412g::syscfg::Syscfg::new(&clocks);
    let exti = stm32f412g::exti::Exti::new(&syscfg);
    let pin = stm32f412g::gpio::Pin::new(PinId::PE02, &exti);
    let gpio_ports = stm32f412g::gpio::GpioPorts::new(&clocks, &exti);
    pin.set_ports_ref(&gpio_ports);
    let led = &mut led::LedHigh::new(&pin);

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
