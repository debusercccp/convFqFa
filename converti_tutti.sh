#!/bin/bash

# Controlla se è stata fornita una cartella, altrimenti usa quella corrente
DIRECTORY=${1:-.}

echo "Inizio conversione dei file .fastq e .fq in $DIRECTORY..."

for file in "$DIRECTORY"/*.{fastq,fq}; do
    [ -e "$file" ] || continue
    
    echo "Processando: $file"
    
    # Opzione 1: Versione Rust
    conv "$file"
    
    # python3 convpy.py "$file"
done

echo "Conversione completata!"
