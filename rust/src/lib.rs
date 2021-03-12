mod board;
mod swap;

use board::FilledBoard;
use cpython::{py_fn, py_module_initializer, PyResult, Python};
use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, BufWriter, Write},
    process, usize,
};
// add bindings to the generated python module
// N.B: names: "rust" must be the name of the `.so` or `.pyd` file
py_module_initializer!(rust, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "run", py_fn!(py, run_py()))?;
    Ok(())
});

#[allow(dead_code)]
fn solve_csv(infile: &str, outfile: &str, lineno: usize) -> io::Result<()> {
    let lines = BufReader::new(File::open(infile)?)
        .lines()
        .collect::<io::Result<Vec<String>>>()?[lineno]
        .clone();
    let output = lines
        .split(",")
        .filter(|part| part.chars().all(|c| c.is_numeric()) && part.parse::<usize>().unwrap() > 0)
        .collect::<Vec<&str>>()
        .join(",");

    File::create(outfile)?.write_all(output.as_bytes())?;
    Ok(())
}

fn run_py(_: Python) -> PyResult<usize> {
    // println!("Hello from rust!");
    let mut args: Vec<String> = env::args().collect();
    if !args.is_empty() && args.first().unwrap().starts_with("python") {
        args.remove(0);
    }

    if args.len() != 3 {
        eprintln!("need 4 args");
        process::exit(1);
    }

    // println!("Args good. Proceed");

    let boards = FilledBoard::from_buf(&mut File::open(&args[1]).expect("failed to open file"))
        .expect("Failed to parse board");
    let mut fout = BufWriter::new(File::create(&args[2]).expect("failed to create file"));
    boards.iter().map(|b| b.check_swap()).for_each(|(a, b)| {
        writeln!(fout, "{},{}", a, b).expect("failed to write to file");
    });

    // solve_csv(&args[1], &args[2], args[3].parse().unwrap()).expect("io error");
    // println!("{:?}", args);
    Ok(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
