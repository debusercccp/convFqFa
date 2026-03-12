import sys
import os

# sys.argv[0] è il nome dello script stesso
# sys.argv[1] sarà il primo argomento che scrivi nel terminale

if len(sys.argv) < 2:
    print("Errore: Devi inserire il nome del file FASTQ!")
    print("Utilizzo: python script.py file.fq")
    sys.exit(1) # Esce dal programma se manca l'argomento

file_input = sys.argv[1]
file_output = file_input.replace(".fq", ".fa").replace(".fastq", ".fa")

try:
    with open(file_input, "r") as f_in, open(file_output, "w") as f_out:
        for i, riga in enumerate(f_in):
            # Logica di conversione FASTQ -> FASTA
            if i % 4 == 0:
                f_out.write(">" + riga[1:]) # Header
            elif i % 4 == 1:
                f_out.write(riga)           # Sequenza
    
    print(f"Conversione completata! Creato il file: {file_output}")

except FileNotFoundError:
    print(f"Errore: Il file '{file_input}' non esiste.")
