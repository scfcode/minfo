//
//

use mountpoints::mountpaths;

fn main() {
    println!("Diplaying Mountpoints");

    let d: Disks = Disks::new();
    d.print_mount_list();
}


#[derive(Debug)]
struct Disks {
    disk_vector: Vec<String>,  // A vector of Strings
}

impl Disks {
    fn new() -> Disks {
        Disks {
            disk_vector: Disks::build_mount_list(),
        }
    }

    fn build_mount_list() -> Vec<String> {

        let mut v: Vec<String> = vec![];

        for mp in mountpaths().unwrap() {   // Build the Vec of mountpath strings
            v.push(mp);
        }

        v    // Return our Vec
    }

    fn print_mount_list(&self) {

        let mut index: usize = 0;

        while index < self.disk_vector.len() {
            println!(" {}", self.disk_vector[index]);
            index = index + 1;
        }
    }
}
