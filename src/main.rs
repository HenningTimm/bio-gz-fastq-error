use bio::io::fastq;
use bio::io::fastq::FastqRead;
use flate2::bufread::GzDecoder;
use std::error::Error;
use std::fs;
use std::io::BufReader;



fn get_readers(fq1: &str, fq2: &str) -> Result<(Box<dyn FastqRead>, Box<dyn FastqRead>), Box<dyn Error>> {
    match (fq1.ends_with(".gz"), fq2.ends_with(".gz")) {
        (true, true) => Ok((
            Box::new(fastq::Reader::new(
                fs::File::open(fq1)
                    .map(BufReader::new)
                    .map(GzDecoder::new)?)),
            Box::new(fastq::Reader::new(
                fs::File::open(fq2)
                    .map(BufReader::new)
                    .map(GzDecoder::new)?))
        )),
        
        (false, false) => Ok((
            Box::new(fastq::Reader::from_file(fq1)?),
            Box::new(fastq::Reader::from_file(fq2)?)
        )),
        _ => panic!("Invalid input")
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut f_rec = fastq::Record::new();
    let mut r_rec = fastq::Record::new();

    // // This works perfectly
    let fq1 = "10/10_clusters_ACTTGA_1.fastq";
    let fq2 = "10/10_clusters_ACTTGA_2.fastq";

    // // This stops after read 8
    // // Comment this back in to see the problem.
    // let fq1 = "10/10_clusters_ACTTGA_1.fastq.gz";
    // let fq2 = "10/10_clusters_ACTTGA_2.fastq.gz";

    let reader = get_readers(fq1, fq2).unwrap();
    let mut fq1_reader = reader.0;
    let mut fq2_reader = reader.1;

    let mut i = 0;
    loop {
        fq1_reader.read(&mut f_rec)?;
        fq2_reader.read(&mut r_rec)?;
        eprintln!("{}", i);

        match (f_rec.is_empty(), r_rec.is_empty()) {
            (true, true) => {
                eprintln!("Both records are empty");
                break;
            }
            (false, false) => (),
            _ => panic!("Given FASTQ files have unequal lengths"),
        }
        eprintln!(
            "r1 {:?} r2 {:?}\n\n",
            String::from_utf8_lossy(&f_rec.seq()[..10]),
            String::from_utf8_lossy(&r_rec.seq()[..10].to_owned())
        );

        i += 1;
    }
    Ok(())
}
