use rppal::gpio::Level;
use rppal::gpio::Mode;

tonic::include_proto!("gpio_types");

impl From<Level> for GpioState {
    fn from(value: Level) -> Self {
        match value {
            Level::High => GpioState::High,
            Level::Low => GpioState::Low,
        }
    }
}

impl From<Mode> for GpioDirection {
    fn from(value: Mode) -> Self {
        match value {
            Mode::Output => GpioDirection::Out,
            Mode::Input => GpioDirection::In,
            _ => GpioDirection::Unspecified,
        }
    }
}
