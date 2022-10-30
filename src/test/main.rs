use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[derive(Debug, Default)]
struct Cpuinfo {
    vendor: String,
    model: String,
}

impl Cpuinfo {
    // load(filename) => collect data from memory file /proc/cpuinfo
    // The file /proc/cpuinfo displays what type of processor your system is running including the number of CPUs present
    // model name – Gives you the common name of the processor, including the project name.

    fn load(filename: String) -> Cpuinfo {
        let lines = lines_from_file(filename);
        let a: Vec<&String> = lines.iter().filter(|&e| e.contains("vendor_id")).collect();
        let vendor_id: Vec<&str> = a[0].split(':').collect();
        //println!("Vendor:{}", vendor_id[1].trim_start());
        let b: Vec<&String> = lines.iter().filter(|&e| e.contains("model name")).collect();
        let model_name: Vec<&str> = b[0].split(':').collect();
        //println!("Model:{}", model_name[1].trim_start());

        Cpuinfo {
            vendor: (vendor_id[1].trim_start().to_string()),
            model: (model_name[1].trim_start().to_string()),
        }
    }
}

fn main() {
    let strucpu: Cpuinfo = Cpuinfo::load(String::from("/proc/cpuinfo"));

    println!("Model -> {}", strucpu.model);
    println!("Vendor -> {}", strucpu.vendor);

    // esta parte es la de vectores

    let v = vec![10, 20, 30, 40, 50];

    for x in &v {
        println!("{x}");
    }
    let mut iter = v.iter().enumerate();

    loop {
        match iter.next() {
            // The division was valid
            Some((x, y)) => println!("Result: {x}-{y}"),
            // The division was invalid
            None => break,
        }
    }

    let mut iter2 = v.iter().filter(|x| **x >= 20);

    loop {
        match iter2.next() {
            // The division was valid
            Some(x) => println!("mayores o igual que 20 =>  {x}"),
            // The division was invalid
            None => break,
        }
    }

    let mut iter3 = v.iter().skip(3);

    loop {
        match iter3.next() {
            // The division was valid
            Some(x) => println!("{x}"),
            // The division was invalid
            None => break,
        }
    }
    let mut iter4 = v.iter().map(|x| x + 20);

    loop {
        match iter4.next() {
            // The division was valid
            Some(x) => println!("Le sumamos 20 a >  {x}"),
            // The division was invalid
            None => break,
        }
    }

    // match a.next() {
    //     Some(x) => println!("{x}"),
    //     None => todo!(),
    //  }
}
