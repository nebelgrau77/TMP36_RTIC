#![no_std]
#![no_main]

use arduino_nano33iot as hal;

use hal::clock::GenericClockController;
use hal::prelude::*;
use rtic::app;
use core::fmt::Write;
use hal::adc::Adc;

const FACTOR: f32 = 3300.0/4096.0; //3300 mV / 4096 values for 12-bit ADC

#[app(device = crate::hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        led: hal::gpio::Pa17<hal::gpio::Output<hal::gpio::OpenDrain>>,
        timer: hal::timer::TimerCounter3,        
        uart: hal::sercom::UART5<hal::sercom::Sercom5Pad3<hal::gpio::Pb23<hal::gpio::PfD>>,hal::sercom::Sercom5Pad2<hal::gpio::Pb22<hal::gpio::PfD>>,(),()>,
        analog: hal::gpio::Pa2<hal::gpio::PfB>,        
        converter: hal::adc::Adc<hal::pac::ADC>,
    }


    /// This function is called each time the tc3 interrupt triggers.
    /// We use it to toggle the LED.  The `wait()` call is important
    /// because it checks and resets the counter ready for the next
    /// period.
    
    #[task(binds = TC3, resources = [timer, led, uart, analog, converter])]
    fn tc3(c: tc3::Context) {
        if c.resources.timer.wait().is_ok() {
            c.resources.led.toggle();                
            let sample: u16 = c.resources.converter.read(c.resources.analog).unwrap();
            let voltage: f32 = sample as f32 * FACTOR; 
            let celsius: f32 = (voltage - 500.0) / 10.0;  //as we want to get the tenths of the degree and display them easily
            c.resources.uart.write_fmt(format_args!("Temperature: {:.01}°C\n\r", celsius)).unwrap();
            
        }
    }

    #[init]
    fn init(c: init::Context) -> init::LateResources {
        
        let mut device = c.device;

        let mut clocks = GenericClockController::with_internal_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );
        let gclk0 = clocks.gclk0();
        let mut pins = hal::Pins::new(device.PORT);

        let mut tc3 = hal::timer::TimerCounter::tc3_(
            &clocks.tcc2_tc3(&gclk0).unwrap(),
            device.TC3,
            &mut device.PM,
        );
        
        let mut serial = hal::uart(&mut clocks,
            9600.hz(), 
            device.SERCOM5,
            &mut device.PM,
            pins.rx,
            pins.tx,
            &mut pins.port);

        let mut adc = Adc::adc(
                        device.ADC, 
                        &mut device.PM, 
                        &mut clocks);        

        tc3.start(1.hz());
        tc3.enable_interrupt();

        init::LateResources {
            led: pins.led_sck.into_open_drain_output(&mut pins.port),
            timer: tc3,        
            uart: serial,            
            analog: pins.a0.into_function_b(&mut pins.port),
            converter: adc,
        }
    }
};