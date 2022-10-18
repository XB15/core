use eyre::WrapErr;
use rppal::i2c::I2c;
use vl53l0x::VL53L0x;

const DEFAULT_ADDRESS: u16 = 0x29;

fn main() -> eyre::Result<()> {
  let mut i2c = I2c::with_bus(1).wrap_err("Error while setting bus")?;
  let boop_range = 0..100;
  i2c
    .set_slave_address(DEFAULT_ADDRESS)
    .wrap_err("Error while setting address")?;
  let mut tof = VL53L0x::new(i2c).unwrap();
  println!("chip initialized");

  tof.set_measurement_timing_budget(250_000).unwrap();
  tof.start_continuous(0).unwrap();

  loop {
    match tof.read_range_continuous_millimeters_blocking() {
      Ok(range) => {
        if boop_range.contains(&range) {
          println!("booped {}", range);
        }
      }
      Err(vl53l0x::Error::Timeout) => continue,
      Err(e) => println!("{:?}", e),
    }
  }
}
