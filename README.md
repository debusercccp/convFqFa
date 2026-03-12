convFqFa

Conversione di file genomici dal formato FASTQ al formato FASTA. 
Il progetto include un'implementazione in Python per e una in Rust.

Descrizione del formato

Il processo di conversione segue le specifiche dello standard FASTQ (4 righe per record) per generare un file FASTA (2 righe per record):

    Header: La prima riga del record FASTQ (che inizia con @) viene convertita in header FASTA sostituendo il prefisso con >.

    Sequenza: La seconda riga (sequenza nucleotidica) viene preservata integralmente.

    Qualità: Le righe 3 e 4 (separatore + e stringa di qualità) vengono ignorate.

Implementazione Python

Il file convpy.py è uno script leggero che utilizza esclusivamente la libreria standard di Python.
Requisiti

    Python 3.x

Utilizzo
Bash

python convpy.py percorso/del/file.fastq

L'output verrà generato automaticamente nella stessa directory del file sorgente con estensione .fa.
Implementazione Rust

Il progetto contenuto nella directory conv/ è progettato per gestire file di grandi dimensioni riducendo al minimo l'overhead di memoria grazie all'uso di buffer di lettura.
Requisiti

    Toolchain Rust (rustc e cargo)

Compilazione

Per ottimizzare le prestazioni, compilare il binario in modalità release:
Bash

cd conv
cargo build --release

Installazione nel Sistema (PATH)

Per utilizzare il convertitore come comando globale del terminale:
Bash

cargo install --path .

Una volta installato, è possibile richiamare lo strumento da qualsiasi directory digitando semplicemente:
Bash

conv esempio.fastq

Struttura del Repository

    convpy.py: Implementazione in linguaggio Python.

    conv/: Progetto Rust gestito da Cargo.

    .gitignore: Esclusioni predefinite per file binari (target/), cache di Python e file di sequenziamento pesanti.

Note sulle Prestazioni

Per file di input superiori a 500MB o per processamenti massivi di dataset genomici, è caldamente raccomandato l'utilizzo della versione Rust compilata con il flag --release.  in Rust per l'efficienza computazionale.
Descrizione del formato

Il processo di conversione segue le specifiche dello standard FASTQ (4 righe per record) per generare un file FASTA (2 righe per record):

    Header: La prima riga del record FASTQ (che inizia con @) viene convertita in header FASTA sostituendo il prefisso con >.

    Sequenza: La seconda riga (sequenza nucleotidica) viene preservata integralmente.

    Qualità: Le righe 3 e 4 (separatore + e stringa di qualità) vengono ignorate.

Implementazione Python

Il file convpy.py è uno script leggero che utilizza esclusivamente la libreria standard di Python.
Requisiti

    Python 3.x

Utilizzo
Bash

python convpy.py percorso/del/file.fastq

L'output verrà generato automaticamente nella stessa directory del file sorgente con estensione .fa.
Implementazione Rust

Il progetto contenuto nella directory conv/ è progettato per gestire file di grandi dimensioni riducendo al minimo l'overhead di memoria grazie all'uso di buffer di lettura.
Requisiti

    Toolchain Rust (rustc e cargo)

Compilazione

Per ottimizzare le prestazioni, compilare il binario in modalità release:
Bash

cd conv
cargo build --release

Installazione nel Sistema (PATH)

Per utilizzare il convertitore come comando globale del terminale:
Bash

cargo install --path .

Una volta installato, è possibile richiamare lo strumento da qualsiasi directory digitando semplicemente:
Bash

conv esempio.fastq

Struttura del Repository

    convpy.py: Implementazione in linguaggio Python.

    conv/: Progetto Rust gestito da Cargo.

    .gitignore: Esclusioni predefinite per file binari (target/), cache di Python e file di sequenziamento pesanti.

Note sulle Prestazioni

Per file di input superiori a 500MB o per processamenti massivi di dataset genomici, è caldamente raccomandato l'utilizzo della versione Rust compilata con il flag --release.
