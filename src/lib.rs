use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn osname() -> Vec<String> {
    let file = File::open("/etc/os-release")?;
    let reader = BufReader::new(file);
    let mut vec = Vec::new();

    for line in reader.lines() {
        vec.push(line);
    }

    vec[0]
    
}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn it_works() {
        let result: usize = add(8, 2);
        assert_eq!(result, 10);
        let result2: Vec<String> = osname();
        assert_eq!(result2,["hola"]);
    }
}
