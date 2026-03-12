use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;

fn main() {
    // 1. Cattura gli argomenti (l'equivalente di sys.argv)
    let args: Vec<String> = env::args().collect();

    // Verifichiamo se l'utente ha passato il file
    if args.len() < 2 {
        eprintln!("Errore: Inserisci il nome del file FASTQ!");
        process::exit(1);
    }

    let file_input = &args[1];
    let file_output = "file.fa";

    // 2. Apriamo il file di input con un Buffer (molto più veloce)
    let f_in = File::open(file_input).expect("Impossibile aprire il file di input");
    let reader = BufReader::new(f_in);

    // 3. Creiamo il file di output
    let mut f_out = File::create(file_output).expect("Impossibile creare il file di output");

    // 4. Ciclo sulle righe (usiamo enumerate come in Python)
    for (i, line) in reader.lines().enumerate() {
        let riga = line.expect("Errore nella lettura della riga");

        match i % 4 {
            0 => {
                // Sostituiamo '@' con '>' (il primo carattere è all'indice 0)
                let header = format!(">{}", &riga[1..]);
                writeln!(f_out, "{}", header).unwrap();
            }
            1 => {
                // Scriviamo la sequenza
                writeln!(f_out, "{}", riga).unwrap();
            }
            _ => continue, // Ignoriamo le altre 2 righe del FASTQ
        }
    }

    println!("Conversione completata in Rust!");
}
