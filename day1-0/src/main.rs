use sysinfo::System;

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("Total Memory: {} bytes", sys.total_memory());
}
