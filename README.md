convFqFa

Strumenti per la conversione di file genomici dal formato FASTQ al formato FASTA. Il repository offre un'implementazione in Python per la massima portabilità e una in Rust per prestazioni elevate.
Logica di Conversione

Il passaggio da FASTQ (4 righe) a FASTA (2 righe) avviene come segue:

    Header: La riga iniziante con @ diventa l'header FASTA iniziante con >.

    Sequenza: La riga nucleotidica viene mantenuta integralmente.

    Qualità: Il separatore + e i punteggi di qualità vengono scartati.

Implementazione Python (convpy.py)

Script essenziale che utilizza esclusivamente la libreria standard di sistema.

    Requisiti: Python 3.x.

    Utilizzo:
    Bash

    python convpy.py percorso/del/file.fastq

    Note: L'output .fa viene generato automaticamente nella stessa cartella dell'input.

Implementazione Rust (conv/)

Versione ottimizzata con gestione della memoria tramite buffer, ideale per file di grandi dimensioni.
Compilazione e Installazione

    Build (Release):
    Bash

cd conv
cargo build --release

Installazione Globale:
Bash

    cargo install --path .

Utilizzo

Una volta installato, il comando è richiamabile da qualsiasi directory:
Bash

conv esempio.fastq

Automazione Bash (converti_tutti.sh)

Script per il processamento batch di tutti i file in una directory.

    Permessi: chmod +x converti_tutti.sh

    Esecuzione: ./converti_tutti.sh [cartella]

Struttura del Progetto

    convpy.py: Script Python indipendente.

    conv/: Directory sorgente del progetto Rust.

    .gitignore: Filtri per binari (target/), cache e file genomici pesanti.

Performance

Si raccomanda l'uso della versione Rust (compilata con --release) per file superiori a 500MB o per elaborazioni massive di dataset, al fine di ridurre drasticamente i tempi di calcolo.

