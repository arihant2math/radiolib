use crate::utils::cryptography::Aes128;
use core::time::Duration;

// #if !defined(_RADIOLIB_HAL_H)
// #define _RADIOLIB_HAL_H
//
// #include <stdint.h>
// #include <stddef.h>
//
// #include "BuildOpt.h"
//
// #include "utils/Cryptography.h"
//
// /*! \brief Global-scope function that returns timestamp since start (in microseconds). */
// RadioLibTime_t rlb_time_us();

pub trait GpioMode {
    fn input() -> Self;
    fn output() -> Self;
}

pub trait GpioLevel {
    fn low() -> Self;
    fn high() -> Self;
}

pub trait GpioInterrupt {
    fn rising() -> Self;
    fn falling() -> Self;
}

pub trait RadioLibHal {
    type Pin;
    type Interrupt;
    type GpioMode: GpioMode;
    type GpioLevel: GpioLevel;
    type GpioInterrupt: GpioInterrupt;
    type AES128: Aes128;

    /// GPIO pin mode (input/output/...) configuration method.
    /// # Parameters
    /// - `pin`: Pin to be changed
    /// - `mode`: Mode to be set
    fn pin_mode(pin: Self::Pin, mode: Self::GpioMode);

    /// Digital write method.
    /// # Parameters
    /// - `pin`: Pin to be changed
    /// - `value`: Value to be set
    fn digital_write(pin: Self::Pin, value: Self::GpioLevel);

    /// Digital read method
    /// # Parameters
    /// - `pin`: Pin to be changed
    /// # Returns
    /// Value read on the pin
    fn digital_read(pin: Self::Pin) -> Self::GpioLevel;

    /// Method to attach function to an external interrupt.
    /// # Parameters
    /// - `interrupt_num`: Interrupt number to attach to
    /// - `interrupt_cb`: Interrupt service routine to execute.
    /// - `mode`: Rising/falling mode.
    fn attach_interrupt(
        interrupt_num: Self::Interrupt,
        interrupt_cb: fn(),
        mode: Self::GpioInterrupt,
    );

    /// Method to detach function from an external interrupt.
    /// # Parameters
    /// - `interrupt_num`: Interrupt number to detach from
    fn detach_interrupt(interrupt_num: Self::Interrupt);

    //     /*!
    //       \brief Blocking wait function.
    //       Must be implemented by the platform-specific hardware abstraction!
    //       \param ms Number of milliseconds to wait.
    //     */
    /// Blocking wait function
    /// # Parameters
    /// - Time to wait.
    fn delay(time: Duration);

    /// Get number of milliseconds since start.
    /// # Parameters
    /// - Number of milliseconds since start.
    fn uptime() -> Duration;
    //     /*!
    //       \brief Measure the length of incoming digital pulse in microseconds.
    //       Must be implemented by the platform-specific hardware abstraction!
    //       \param pin Pin to measure on (platform-specific).
    //       \param state Pin level to monitor (platform-specific).
    //       \param timeout Timeout in microseconds.
    //       \returns Pulse length in microseconds, or 0 if the pulse did not start before timeout.
    //     */
    fn pulse_in(pin: Self::Pin, state: Self::GpioLevel, timeout: Duration) -> Duration;
    //     /*!
    //       \brief SPI initialization method.
    //     */
    fn spi_begin();
    //
    //     /*!
    //       \brief Method to start SPI transaction.
    //     */
    fn spi_begin_transaction();
    //
    //     /*!
    //       \brief Method to transfer buffer over SPI.
    //       \param out Buffer to send.
    //       \param len Number of data to send or receive.
    //       \param in Buffer to save received data into.
    //     */
    fn spi_transfer(out: &[u8], in_buf: &mut [u8]);
    //
    //     /*!
    //       \brief Method to end SPI transaction.
    //     */
    fn spi_end_transaction();
    //
    //     /*!
    //       \brief SPI termination method.
    //     */
    fn spi_end();
    //
    //     // virtual methods - these may or may not exists on a given platform
    //     // they exist in this implementation, but do nothing
    //
    //     /*!
    //       \brief Module initialization method.
    //       This will be called by all radio modules at the beginning of startup.
    //       Can be used to e.g., initialize SPI interface.
    //     */
    //     virtual void init();
    fn init() {}
    //
    //     /*!
    //       \brief Module termination method.
    //       This will be called by all radio modules when the destructor is called.
    //       Can be used to e.g., stop SPI interface.
    //     */
    fn term() {}
    //
    //     /*!
    //       \brief Method to produce a square-wave with 50% duty cycle ("tone") of a given frequency at some pin.
    //       \param pin Pin to be used as the output.
    //       \param frequency Frequency of the square wave.
    //       \param duration Duration of the tone in ms. When set to 0, the tone will be infinite.
    //     */
    fn tone(_pin: Self::Pin, _frequency: u32, _duration: Duration) {}

    //     /*!
    //       \brief Method to stop producing a tone.
    //       \param pin Pin which is currently producing the tone.
    //     */
    //     virtual void noTone(uint32_t pin);
    fn no_tone(_pin: Self::Pin) {}
    //     /*!
    //       \brief Yield method, called from long loops in multi-threaded environment (to prevent blocking other threads).
    //     */
    //     virtual void yield();
    fn r#yield() {}
    //     /*!
    //       \brief Function to convert from pin number to interrupt number.
    //       \param pin Pin to convert from.
    //       \returns The interrupt number of a given pin.
    //     */
    // TODO: Should be optional
    fn pin_to_interrupt(pin: Self::Pin) -> Self::Interrupt;
    //     /*!
    //       \brief Enable or disable pull up or pull down for a specific pin.
    //       \param pin Pin to change.
    //       \param enable True to enable, false to disable.
    //       \param up Pull direction, true for pull up, false for pull down.
    //     */
    // TODO: Should be optional
    fn pull_up_down(pin: Self::Pin, enable: bool, up: bool);
}
