use eyre::{Result, WrapErr};
use rppal::i2c;
use vl53l0x::VL53L0x;

fn main() -> eyre::Result<()> {
  let mut i2c = dbg!(I2c::with_bus(1)?);

  dbg!(i2c.set_slave_address(address)?);
  let mut chip = VL53L0x::new(I2c).unwrap();
  let wai = dbg!(chip.who_am_i()?);
  if wai == 0xEE {
    dbg!(chip.init_hardware()?);
    // FIXME: return an error/optional
    /*
    chip.set_high_i2c_voltage(); // TODO: make configurable
    chip.revision_id = chip.read_revision_id();
    chip.reset();
    chip.set_high_i2c_voltage();
    chip.set_standard_i2c_mode(); // TODO: make configurable
     */
  } else {
    panic!("invalid device: {wai}")
  }

  println!("test");
  chip.set_measurement_timing_budget(200_000).unwrap();
  chip.start_continuous(20).unwrap();

  loop {
    match tof.read_range_continuous_millimeters_blocking() {
      Ok(range) => println!("{}mm", range),
      Err(vl53l0x::Error::Timeout) => continue,
      Err(e) => println!("{:?}", e),
    }
  }
}
