extern crate bellman;
extern crate blake2;
extern crate byteorder;
extern crate pairing;
extern crate powersoftau;
extern crate rand;

use pairing::PrimeField;
use powersoftau::*;

use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Write};

fn main() {
    // Try to load `./transcript` from disk.
    let reader = OpenOptions::new()
        .read(true)
        .open("transcript")
        .expect("unable open `./transcript` in this directory");

    let mut reader = BufReader::with_capacity(1024 * 1024, reader);

    // Initialize the accumulator
    let mut current_accumulator = Accumulator::new();

    // The "last response file hash" is just a blank BLAKE2b hash
    // at the beginning of the hash chain.
    let mut last_response_file_hash = [0; 64];
    last_response_file_hash.copy_from_slice(blank_hash().as_slice());

    // There were 89 rounds.
    for i in 0..89 {
        // Deserialize the accumulator provided by the player in
        // their response file. It's stored in the transcript in
        // uncompressed form so that we can more efficiently
        // deserialize it.
        let response_file_accumulator =
            Accumulator::deserialize(&mut reader, UseCompression::No, CheckForCorrectness::No)
                .expect("unable to read uncompressed accumulator");

        // Deserialize the public key provided by the player.
        let _ = PublicKey::deserialize(&mut reader)
            .expect("wasn't able to deserialize the response file's public key");

        println!("{}", i);

        current_accumulator = response_file_accumulator;
    }

    {
        let paramname = "g1_coeffs";
        println!("Creating {}", paramname);

        let writer = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(paramname)
            .expect("unable to create parameter file in this directory");

        let mut writer = BufWriter::new(writer);

        for coeff in current_accumulator.tau_powers_g1.iter() {
            writer
                .write_all(format!("{} {}\n", coeff.x.into_repr(), coeff.y.into_repr()).as_ref())
                .unwrap();
        }
    }

    {
        let paramname = "g2_coeffs";
        println!("Creating {}", paramname);

        let writer = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(paramname)
            .expect("unable to create parameter file in this directory");

        let mut writer = BufWriter::new(writer);

        for coeff in current_accumulator.tau_powers_g2.iter() {
            writer
                .write_all(
                    format!(
                        "{} {} {} {}\n",
                        coeff.x.c0.into_repr(),
                        coeff.x.c1.into_repr(),
                        coeff.y.c0.into_repr(),
                        coeff.y.c1.into_repr()
                    )
                    .as_ref(),
                )
                .unwrap();
        }
    }

    {
        let paramname = "g1_alpha_coeffs";
        println!("Creating {}", paramname);

        let writer = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(paramname)
            .expect("unable to create parameter file in this directory");

        let mut writer = BufWriter::new(writer);

        for coeff in current_accumulator.alpha_tau_powers_g1.iter() {
            writer
                .write_all(format!("{} {}\n", coeff.x.into_repr(), coeff.y.into_repr()).as_ref())
                .unwrap();
        }
    }

    {
        let paramname = "g1_beta_coeffs";
        println!("Creating {}", paramname);

        let writer = OpenOptions::new()
            .read(false)
            .write(true)
            .create_new(true)
            .open(paramname)
            .expect("unable to create parameter file in this directory");

        let mut writer = BufWriter::new(writer);

        for coeff in current_accumulator.beta_tau_powers_g1.iter() {
            writer
                .write_all(format!("{} {}\n", coeff.x.into_repr(), coeff.y.into_repr()).as_ref())
                .unwrap();
        }
    }
}
