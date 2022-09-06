use vl53l0x::VL53L0x;

fn main() {
  let _ = VL53L0x::new().unwrap();
  
  // let mut tof = VL53L0x::new().unwrap();

  // tof.set_measurement_timing_budget(20_000).unwrap();

  // tof.start_continuous(20).unwrap();

  // loop {
  //   match tof.read_range_mm() {
  //     Ok(range) => println!("{}mm", range),
  //     Err(e) => println!("{:?}", e),
  //   }
  // }
}
