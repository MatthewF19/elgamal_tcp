use std::{env, io::{self, ErrorKind, Read, Write}, net::{TcpListener, TcpStream}, str::FromStr};

use lib::sqmul;

#[derive(Debug)]
enum Status {
    BOB,
    ALICE,
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "bob" => Ok(Self::BOB),
            "alice" => Ok(Self::ALICE),
            _ => Err(())
        }
    }
}

fn host() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let (mut socket, _addr) = listener.accept()?;

    // read the message to be sent and parse
    // it as an array of bytes
    print!("enter message: ");
    io::stdout().flush()?;
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed_msg = buffer.trim();
    let msg = trimmed_msg.parse::<u32>().unwrap();

    // ask for a prime from the user or use the default (7)
    print!("enter a prime (or 0 for default -- 7): ");
    io::stdout().flush()?;
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed_prime = buffer.trim();
    let mut prime = trimmed_prime.parse::<u32>().unwrap();
    if prime == 0 { prime = 7; }

    // ask for alpha value from the user or use the default (3)  
    print!("enter an alpha value in 2..p-2 (or 0 for default -- 3): ");
    io::stdout().flush()?;
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed_alpha = buffer.trim();
    let mut alpha = trimmed_alpha.parse::<u32>().unwrap();
    if alpha == 0 { alpha = 3; }

    // ask for d value from the user or use the default (5)  
    print!("enter a d value in 2..p-2 (or 0 for default -- 5): ");
    io::stdout().flush()?;
    buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    let trimmed_d = buffer.trim();
    let mut d = trimmed_d.parse::<u32>().unwrap();
    if d == 0 { d = 5; }
    let bin_d = String::from(format!("{d:b}"));

    let beta = sqmul::square_mult(alpha, bin_d.clone(), prime);

    let mut prime_bytes = u32_to_u8_vec(prime);
    let mut alpha_bytes = u32_to_u8_vec(alpha);
    let mut beta_bytes = u32_to_u8_vec(beta);

    println!("writing prime");
    socket.write(&mut prime_bytes)?;
    socket.flush()?;
    println!("writing alpha");
    socket.write(&mut alpha_bytes)?;
    socket.flush()?;
    println!("writing beta");
    socket.write(&mut beta_bytes)?;
    socket.flush()?;
    
    let mut buff = [0; 4];
    socket.read_exact(&mut buff)?;
    let Ke = u8_vec_to_u32(buff.try_into().expect("ISSUE WITH KE"));
    println!("received Ke");

    let Km = sqmul::square_mult(Ke, bin_d, prime);

    // encryption
    let y = (msg * Km) % prime;
    let mut bin_y = u32_to_u8_vec(y);

    println!("writing y");
    socket.write(&mut bin_y)?;

    Ok(())
}


fn u32_to_u8_vec(x: u32) -> [u8; 4] {
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return [b1, b2, b3, b4]
}

fn u8_vec_to_u32(bytes: [u8; 4]) -> u32 {
    let mut res: u32 = 0;

    for byte in bytes.iter() {
        res <<= 8;
        let byte32 = *byte as u32;
        res |= byte32;
    }

    return res;
}

fn client() -> std::io::Result<()> {
    let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8080")?; 
    
    let mut buff = [0; 4];
    stream.read_exact(&mut buff)?;
    let prime = u8_vec_to_u32(buff.try_into().expect("ISSUE WITH PRIME"));
    println!("received prime");

    buff = [0; 4];
    stream.read_exact(&mut buff)?;
    let alpha = u8_vec_to_u32(buff.try_into().expect("ISSUE WITH ALPHA"));
    println!("received alpha");

    buff = [0; 4];
    stream.read_exact(&mut buff)?;
    let beta = u8_vec_to_u32(buff.try_into().expect("ISSUE WITH BETA"));
    println!("received beta");

    // ask for alpha value from the user or use the default (3)  
    print!("enter an i value in 2..{} (or 0 for default -- 3): ", prime-2);
    io::stdout().flush()?;
    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer)?;

    let trimmed_i = input_buffer.trim();
    let mut i = trimmed_i.parse::<u32>().unwrap();
    if i == 0 { i = 3; }
    let bin_i = String::from(format!("{i:b}"));

    let Ke = sqmul::square_mult(alpha, bin_i.clone(), prime);
    let mut Ke_bytes = u32_to_u8_vec(Ke);

    println!("writing Ke");
    stream.write(&mut Ke_bytes)?;

    let Km = sqmul::square_mult(beta, bin_i, prime);

    // read encrypted message
    buff = [0; 4];
    stream.read_exact(&mut buff)?;
    let y = u8_vec_to_u32(buff.try_into().expect("ISSUE WITH Y"));
    println!("received y");


    let (gcd, s, t) = sqmul::eea(Km as i32, prime as i32);
    let Km_inv: u32 = ((s + prime as i32) % (prime as i32)).try_into().expect("HUH");

    println!("MESSAGE: {}", y * Km_inv % prime);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { return Err(std::io::Error::new(ErrorKind::Other, "wrong args")); }

    if let Ok(status) = Status::from_str(&args[1]) {
        match status {
            Status::BOB => host()?,
            Status::ALICE => client()?,
        };
    }

    Ok(())
}
