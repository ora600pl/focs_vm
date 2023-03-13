use clap::Parser;
use proc_maps::{get_process_maps, Pid};
use patternscan::scan;
use std::io::Cursor;
use std::io::Read;
use std::io::{self, Write};
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::{thread, time};
use pretty_hex::*;


/// Tool for finding patterns in memory
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    ///Size of memory segment to scan
    #[clap(default_value_t=0, short, long)]
    memory_size: u64,

    ///PID of the process to scan
    #[clap(short, long)]
    pid: u64,

    ///Pattern in hex to search for
    #[clap(default_value="FF", short, long)]
    pattern: String,

    ///Parallel degree
    #[clap(default_value_t=4,short='P', long)]
    parallel: u64,

    ///Size of a buffer to print
    #[clap(default_value_t=256,short, long)]
    buffer: usize,

    ///Watch memory offset for incomming data
    #[clap(default_value_t=0,short, long)]
    watch: u64,
}


fn scan_memory(pid: u64, scan_from: u64, scan_to: u64, pattern: String, buffer_to_print: usize) {
    print!("Scanning memory from {} to {} in a separate thread\n", scan_from, scan_to);

    let fname = format!("/proc/{}/mem", pid); //memory file 
    let mut f = File::open(fname).unwrap();  

    let mut position = scan_from;
    while position < scan_to {
        f.seek(SeekFrom::Start(position)).unwrap(); 
        let mut buffer = [0; 1_048_576]; //1M buffer
        f.read(&mut buffer).unwrap();
	    let positions = scan(Cursor::new(buffer), &pattern).unwrap();
        if positions.len() > 0 {
            println!("\nFound {} positions in a chunk", positions.len());
            for p in positions {
		    println!("Offset: {} \n", p+position as usize);
                    println!("{:?}\n\t", buffer[(p as usize)..(p as usize+buffer_to_print)].hex_dump());
            }
        }
        position += 1_048_576;
        print!("\rScanned: {} %", ((position-scan_from) as f64 / (scan_to-scan_from) as f64 * 100 as f64) as u8);
    }
}

fn watch_memory_offset(pid: u64, offset: u64) {
    print!("Watch PID {} at offset {} - press CTRL-C for end\n\n", pid, offset);
    let fname = format!("/proc/{}/mem", pid); //memory file 
    let mut f = File::open(fname).unwrap();  

    let mut buf = Vec::new();
    let mut scan_from = offset;
    let sleep_time = time::Duration::from_millis(500);

    loop {
        f.seek(SeekFrom::Start(scan_from)).unwrap();
        loop {
            let mut byte_val = [0;1];
            f.read(&mut byte_val).unwrap();
            if byte_val[0] > 0 {
                buf.push(byte_val[0]);
                
            } else {
                break;
            }
        }
        if buf.len() > 0 {
            unsafe {
                let message: String = String::from_utf8_unchecked(buf.clone());
                print!("{}", message);
                io::stdout().flush().unwrap();
                scan_from += buf.len() as u64;
            }
        }
        
        buf.clear();
        thread::sleep(sleep_time);
    }

}


fn main() {
    let args = Args::parse();
    let pid = args.pid;
    if args.watch == 0 {
            let mem_size = args.memory_size;
            
            let maps = get_process_maps(pid as Pid).unwrap();
            let pattern = args.pattern;

            let mut scan_from: u64 = 0;
            let mut scan_to: u64 = 0;

            for map in maps {
                if (map.size() as u64) == mem_size {
                    scan_from = map.start() as u64;
                    scan_to   = scan_from + (map.size() as u64);
                    println!("Found map at the start offset = {} \t end offset = {}\n", scan_from, scan_to);
                    break;
                }
            }
        
            let chunk = mem_size / args.parallel;
            let mut scan_from_chunk = scan_from;
            let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();

            while scan_from_chunk < scan_to {
            let mut scan_to_chunk = scan_from_chunk+chunk;
                if scan_to_chunk > scan_to {
                    scan_to_chunk = scan_to;
                 }
            let t = thread::Builder::new().stack_size(32 * 1024 * 1024);
            let p = pattern.clone();
            threads.push(t.spawn(move || {scan_memory(pid, scan_from_chunk, scan_to_chunk, p, args.buffer);}).unwrap());
            scan_from_chunk+=chunk;
            }

            for t in threads {
                t.join().unwrap();
            }
    } else {
        watch_memory_offset(pid, args.watch);
    }
}
