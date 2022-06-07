use build_time::build_time_utc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("This program was built on {}.", build_time_utc!());
    Ok(())
}
