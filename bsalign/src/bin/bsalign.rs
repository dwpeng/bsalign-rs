use bsalign::pairwise::{BsPairwirseAligner, BsPairwiseParam};
use bsalign::poa::{BsPoaAligner, BsPoaParam};
use bsalign::{AlignMode, AlignScore};

use clap::ValueEnum;
use clap::*;
use noodles::fasta::io::reader::Builder as FastaReaderBuilder;
use noodles::fasta::record::Record;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Copy, Clone, ValueEnum)]
enum AlignCliMode {
    Overlap,
    Global,
    Extend,
    Kmer,
}

impl From<AlignCliMode> for AlignMode {
    fn from(value: AlignCliMode) -> Self {
        match value {
            AlignCliMode::Overlap => AlignMode::Overlap,
            AlignCliMode::Global => AlignMode::Global,
            AlignCliMode::Extend => AlignMode::Extend,
            AlignCliMode::Kmer => AlignMode::Kmer,
        }
    }
}

#[derive(Subcommand)]
enum Commands {
    /// Pairwise alignment (align)
    Align {
        #[arg(
            short,
            long,
            default_value = "overlap",
            hide_long_help = true,
            hide_possible_values = true,
            help = "Align mode: global/extend/overlap"
        )]
        mode: Option<AlignCliMode>,
        /// Bandwidth for overlap alignment
        #[arg(short = 'W', default_value_t = 0)]
        bandwidth: i32,
        /// Match score
        #[arg(short = 'M', default_value_t = 2)]
        match_score: i32,
        /// Mismatch score
        #[arg(short = 'X', default_value_t = -1)]
        mismatch_score: i32,
        /// Gap open score
        #[arg(short = 'O', default_value_t = -3)]
        gap_open_score: i32,
        /// Gap extend score
        #[arg(short = 'E', default_value_t = -2)]
        gap_extend_score: i32,
        /// Penalty for gap2 open
        #[arg(short = 'Q', default_value_t = 0)]
        gap2_open_score: i32,
        /// Penalty for gap2 extension
        #[arg(short = 'P', default_value_t = 0)]
        gap2_extend_score: i32,
        /// The rest of the arguments are FASTA files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Pairwise alignment (edit)
    Edit {
        /// Align mode
        #[arg(short, long, default_value = "global")]
        mode: Option<AlignCliMode>,
        /// Bandwidth for overlap alignment
        #[arg(short = 'W', default_value_t = 0)]
        bandwidth: i32,
        /// Kmer size (<=15)
        #[arg(short = 'k', default_value_t = 13)]
        ksize: usize,
        /// input fasta files
        #[arg(required = true)]
        files: Vec<String>,
    },
}

fn read_records(files: Vec<String>, limit: usize) -> Vec<Record> {
    let mut records = Vec::new();
    let mut count = 0;
    for file in files {
        let mut reader = FastaReaderBuilder::default()
            .build_from_path(file)
            .expect("Failed to open file");
        for record in reader.records() {
            records.push(record.expect("Failed to read record"));
            count += 1;
            if count >= limit {
                break;
            }
        }
    }
    records
}

fn align(
    rseq: &Record,
    qseq: &Record,
    mode: AlignCliMode,
    bandwidth: i32,
    match_score: i32,
    mismatch_score: i32,
    gap_open_score: i32,
    gap_extend_score: i32,
    gap2_open_score: i32,
    gap2_extend_score: i32,
) {
    let mut align_score = AlignScore::pairwise_default();
    align_score.M = match_score;
    align_score.X = mismatch_score;
    align_score.O = gap_open_score;
    align_score.E = gap_extend_score;
    align_score.Q = gap2_open_score;
    align_score.P = gap2_extend_score;

    let param = BsPairwiseParam::default()
        .set_bandwidth(bandwidth as usize)
        .set_align_score(align_score)
        .set_mode(mode.into())
        .set_cigar(true);

    let mut aligner = BsPairwirseAligner::new(param);

    let result = aligner.align_banded_striped_8bit(rseq.sequence(), qseq.sequence());

    let rname = String::from_utf8_lossy(rseq.name()).to_string();
    let qname = String::from_utf8_lossy(qseq.name()).to_string();

    print!(
        "{}\t{}\t+\t{}\t{}\t{}\t{}\t+\t{}\t{}\t",
        rname,
        qseq.sequence().len(),
        result.qb,
        result.qe,
        qname,
        rseq.sequence().len(),
        result.tb,
        result.te,
    );
    println!(
        "{}\t{:.3}\t{}\t{}\t{}\t{}",
        result.score,
        1.0 * result.mat as f32 / result.aln as f32,
        result.mat,
        result.mis,
        result.ins,
        result.del
    );

    let aln = result.to_string();
    println!("{}", aln.tseq());
    println!("{}", aln.alignment());
    println!("{}", aln.qseq());
}

fn edit(rseq: &Record, qseq: &Record, mode: AlignCliMode, bandwidth: i32, ksize: usize) {
    if ksize > 15 {
        eprintln!("Kmer size must be less than or equal to 15");
        std::process::exit(1);
    }

    let param = BsPairwiseParam::default()
        .set_bandwidth(bandwidth as usize)
        .set_mode(mode.into())
        .set_ksize(ksize)
        .set_cigar(true);

    let mut aligner = BsPairwirseAligner::new(param);

    let result = aligner.align_kmer_striped_edit(rseq.sequence(), qseq.sequence());

    let rname = String::from_utf8_lossy(rseq.name()).to_string();
    let qname = String::from_utf8_lossy(qseq.name()).to_string();

    print!(
        "{}\t{}\t+\t{}\t{}\t{}\t{}\t+\t{}\t{}\t",
        rname,
        qseq.sequence().len(),
        result.qb,
        result.qe,
        qname,
        rseq.sequence().len(),
        result.tb,
        result.te,
    );
    println!(
        "{}\t{:.3}\t{}\t{}\t{}\t{}",
        result.score,
        1.0 * result.mat as f32 / result.aln as f32,
        result.mat,
        result.mis,
        result.ins,
        result.del
    );

    let aln = result.to_string();
    println!("{}", aln.tseq());
    println!("{}", aln.alignment());
    println!("{}", aln.qseq());
}

fn main() {
    let command = Cli::parse();
    match command.command {
        Commands::Align {
            mode,
            bandwidth,
            match_score,
            mismatch_score,
            gap_open_score,
            gap_extend_score,
            gap2_open_score,
            gap2_extend_score,
            files,
        } => {
            match mode {
                Some(AlignCliMode::Kmer) => {
                    eprintln!("Only `Edit` command supports kmer mode");
                    std::process::exit(1);
                }
                _ => {}
            }
            let records = read_records(files, 2);
            align(
                &records[0],
                &records[1],
                mode.unwrap_or(AlignCliMode::Overlap),
                bandwidth,
                match_score,
                mismatch_score,
                gap_open_score,
                gap_extend_score,
                gap2_open_score,
                gap2_extend_score,
            );
        }
        Commands::Edit {
            mode,
            bandwidth,
            ksize,
            files,
        } => {
            let records = read_records(files, 2);
            edit(
                &records[0],
                &records[1],
                mode.unwrap_or(AlignCliMode::Overlap),
                bandwidth,
                ksize,
            );
        }
    }
}
