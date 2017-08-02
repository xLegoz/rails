use super::uart;

use mach::mmio::mmio_write;

extern "C" {
  pub fn delay(cycles: u32);
}

fn safe_delay(cycles: u32) {
  unsafe {
    delay(cycles);
  }
}

pub fn uart_init() {

  // Disable UART0.
  mmio_write(uart::UART0_CR, 0x00000000);
  // Setup the GPIO pin 14 && 15.

  // Disable pull up/down for all GPIO pins & delay for 150 cycles.
  mmio_write(uart::GPPUD, 0x00000000);
  safe_delay(150);

  // Disable pull up/down for pin 14,15 & delay for 150 cycles.
  mmio_write(uart::GPPUDCLK0, (1 << 14) | (1 << 15));
  safe_delay(150);

  // Write 0 to GPPUDCLK0 to make it take effect.
  mmio_write(uart::GPPUDCLK0, 0x00000000);

  // Clear pending interrupts.
  mmio_write(uart::UART0_ICR, 0x7FF);

  // Set integer & fractional part of baud rate.
  // Divider = UART_CLOCK/(16 * Baud)
  // Fraction part register = (Fractional part * 64) + 0.5
  // UART_CLOCK = 3000000; Baud = 115200.

  // Divider = 3000000 / (16 * 115200) = 1.627 = ~1.
  mmio_write(uart::UART0_IBRD, 1);
  // Fractional part register = (.627 * 64) + 0.5 = 40.6 = ~40.
  mmio_write(uart::UART0_FBRD, 40);

  // Enable FIFO & 8 bit data transmissio (1 stop bit, no parity).
  mmio_write(uart::UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

  // Mask all interrupts.
  mmio_write(uart::UART0_IMSC,
             (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) |
             (1 << 10));

  // Enable UART0, receive & transfer part of UART.
  mmio_write(uart::UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));
}
