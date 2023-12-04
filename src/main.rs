use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write, stdin, stdout};

fn fact(n : u32) -> u32
{
    if n < 2
    {
        return 1;
    }

    return n * fact(n - 1);
}



fn main() -> io::Result<()>
{    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("ERROR");
    }

    let startup = "indulo.dat";
    let running = "aktualis.dat";

    match fs::metadata(startup)
    {
        Ok(_) => {}
        Err(_) =>
        {
            let _ = File::create(startup);
        }
    }
    
    match fs::metadata(running)
    {
        Ok(_) => {}
        Err(_) =>
        {
            let _ = File::create(running);
        }
    }

    let mut file = File::open(startup)?;

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);

    println!("Content: {}", content);







    let mut in_str = String::new();

    print!("Enter your number: ");
    let _result = stdout().flush();

    let _ = stdin().read_line(&mut in_str).expect("ERROR1");

    let num_int: u32 = match in_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Bad input!");
            1
        }
    };


    let res = fact(num_int);

    println!("Result: {}", res);
    Ok(())
}
