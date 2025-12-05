use std::error::Error;
use std::io::{Write, stdin, stdout};

use midir::{Ignore, MidiInput, MidiOutput};

fn main() {
    env_logger::init();
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_out = MidiOutput::new("midir writing output")?;

    // Get an input port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port = match out_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    println!("\nOpening connection");
    let out_port_name = midi_out.port_name(out_port)?;

    let mut conn_out = midi_out.connect(out_port, "midir-write-output")?;

    println!(
        "Connection open, reading input from '{}' (press enter to exit) ...",
        out_port_name
    );

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    conn_out.send(&[192, 0])?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    conn_out.send(&[192, 6])?;

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
