// We only need to import System now, SystemExt is gone.
use sysinfo::System;

fn main() {
    // 1. Create a System object to gather all information
    let mut sys = System::new_all();

    // 2. Refresh the system information to get the latest data
    sys.refresh_all();

    // 3. Print a basic value (e.g., Total Memory)
    // Note: total_memory() returns bytes
    println!("Total Memory: {} bytes", sys.total_memory());
}
