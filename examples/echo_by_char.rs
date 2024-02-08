//! Echo console input back to console
//!
//! On blackpill:
//! Connect the Tx pin pa9  to the Rx pin of usb-ttl converter
//! Connect the Rx pin pa10 to the Tx pin of usb-ttl converter
//! Set up the serial console (e.g. minicom) with the same settings used here.
//! (Using 9600bps, could be higher but needs serial console to be the same.)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;

use embedded_io::{Read, Write};

//use core::str::from_utf8;

#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal as hal;

#[cfg(feature = "stm32g4xx")]
use stm32g4xx_hal as hal;

#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal as hal;

use hal::{                    // for common locations, differences below
    // pac::Peripherals, 
    //pac::USART1,
    prelude::*,
    serial::{Rx, Tx},
};



// setup() does  hal/MCU specific setup and returns generic (tx, rx) for use in main code.

#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::{
    pac::Peripherals,
    pac::USART1,
    serial::{config::Config, Serial},
};

#[cfg(feature = "stm32f4xx")]
fn setup() -> (Tx<USART1>, Rx<USART1>) {
    let dp = Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    let gpioa = dp.GPIOA.split();

    Serial::new(
        dp.USART1,
        (gpioa.pa9.into_alternate(), gpioa.pa10.into_alternate()),
        Config::default().baudrate(9600.bps()),
        &clocks,
    ).unwrap().split()
}



#[cfg(feature = "stm32g4xx")]
use stm32g4xx_hal::{
    stm32::Peripherals,            // is there a convention that pac should be used or an alias?
    stm32::USART1,                 // is there a convention that pac should be used or an alias?
    serial::{FullConfig, NoDMA},
    gpio::{Alternate, gpioa::{PA9, PA10}},          // why Tx<USART1, pin, pin> not Tx<USART1> ?
};

#[cfg(feature = "stm32g4xx")]
fn setup() -> (Tx<USART1, PA9<Alternate<7_u8>>, NoDMA>, Rx<USART1, PA10<Alternate<7_u8>>, NoDMA>) {
    let dp = Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let gpioa = dp.GPIOA.split(&mut rcc);

    dp.USART1.usart(
       gpioa.pa9.into_alternate(), 
       gpioa.pa10.into_alternate(), 
       FullConfig::default().baudrate(9600.bps()), &mut rcc).unwrap().split()
}



#[cfg(feature = "stm32h7xx")]
use stm32h7xx_hal::{
    pac::Peripherals,
    pac::USART1,
};

#[cfg(feature = "stm32h7xx")]
fn setup() -> (Tx<USART1>, Rx<USART1>) {
    let dp = Peripherals::take().unwrap();
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(160.MHz()).freeze(vos, &dp.SYSCFG);
    let clocks = ccdr.clocks;
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    dp.USART1.serial(
            (
                gpioa.pa9.into_alternate(),  //tx 
                gpioa.pa10.into_alternate(), //rx
            ),
            9600.bps(),
            ccdr.peripheral.USART1,
            &clocks,
    ).unwrap().split()
}

// End of hal/MCU specific setup. Following should be generic code.



#[entry]
fn main() -> ! {

    let (mut tx1, mut rx1) = setup();

    hprintln!("test write to console ...");

    tx1.write(b"\r\nconsole connect check.\r\n").ok(); // need next disambiguate syntac for stm32f4xx_hal
    //embedded_io::Write::write(&mut tx1, b"\r\nconsole connect check.\r\n").ok();

    hprintln!("test read and write by char. Please type into the console ...");

    tx1.write(b"test read and write by char. Please type into the console ...").ok();
    //embedded_io::Write::write(&mut tx1, b"test read and write by char. Please type into the console ...").ok();

    let mut buffer: [u8; 5] = [0; 5];  // could be length 1 for a byte

    loop {
        // Read a byte and write
        let _len = rx1.read(&mut buffer);
        // let _len = embedded_io::Read::read(&mut rx1, &mut buffer).ok();

        //hprintln!("received");    // for debugging

        tx1.write(&buffer).ok();
        //embedded_io::Write::write(&mut tx1, &buffer).ok();

        hprintln!("{:?}", &buffer); // for debugging
    }
}
