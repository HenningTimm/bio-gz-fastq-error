# bio-gz-fastq-error
Minimal example for problem with fastq reader.

Execute with `cargo run`.

The simulated reads in the folder 10 are present as `.fastq.gz` and as `.fastq` files (extracted with `gunzip -k`).
Run the code once to see 100 read beeing read, then comment lines 42-43 back in and only 8 readcs are read.
With `zless` the reads are there and since the unzipped files were generated from the zipped files, this should not be a problem of the input.
