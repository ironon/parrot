use embedded_serial::MutNonBlockingTx;
use serialport::SerialPort;
struct SomeStruct<T> {
    sent: Option<usize>,
    uart: T
};

impl<T> SomeStruct<T> where T: MutNonBlockingTx {

    fn new(uart: T) -> SomeStruct<T> {
        SomeStruct { uart: uart, sent: Some(0) }
    }

    fn write_data(&mut self) -> Result<bool, <T as MutNonBlockingTx>::Error> {
        let data = b"AT\n";
        if let Some(len) = self.sent {
            match self.uart.puts_try(&data[len..]) {
                // Sent some or more of the data
                Ok(sent) => {
                    let total = len + sent;
                    self.sent = if total == data.len() {
                        None
                    } else {
                        Some(total)
                    };
                    Ok(false)
                }
                // Sent some of the data but errored out
                Err((sent, e)) => {
                    let total = len + sent;
                    self.sent = if total == data.len() {
                        None
                    } else {
                        Some(total)
                    };
                    Err(e)
                }
            }
        } else {
            Ok(true)
        }
    }
}


fn main() {
    let mut uart = serialport::new("/dev/ttyUSB0", 9600).open().unwrap();
    let mut some_struct = SomeStruct::new(&mut uart);

    loop {
        match some_struct.write_data() {
            Ok(true) => {
                // All data has been sent
                break;
            }
            Ok(false) => {
                // Some data has been sent
            }
            Err(e) => {
                // Error occurred
                break;
            }
        }
    }
}