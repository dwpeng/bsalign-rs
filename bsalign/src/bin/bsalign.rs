use bsalign::pairwise::{BsPairwirseAligner, BsPairwiseParam};
use bsalign::poa::{BsPoaAligner, BsPoaParam};
use bsalign::{AlignMode, AlignScore};

use bsalign_sys::bindings;
use clap::*;
use noodles::fasta::io::reader::Builder as FastaReaderBuilder;
use noodles::fasta::record::Record;
use std::os::raw::c_char;

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
        mode: AlignCliMode,
        /// Bandwidth for alignment
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
        mode: AlignCliMode,
        /// Bandwidth for alignment
        #[arg(short = 'W', default_value_t = 0)]
        bandwidth: i32,
        /// Kmer size (<=15)
        #[arg(short = 'k', default_value_t = 13)]
        ksize: usize,
        /// input fasta files
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Partial order alignment (multi-sequence alignment)
    Poa {
        /// Align mode
        #[arg(
            short,
            long,
            default_value = "global",
            hide_long_help = true,
            hide_possible_values = true,
            help = "Align mode: global/extend/overlap"
        )]
        mode: AlignCliMode,
        /// Bandwidth for alignment
        #[arg(short = 'W', default_value_t = 128)]
        bandwidth: i32,
        /// Match score
        #[arg(short = 'M', default_value_t = 2)]
        match_score: i32,
        /// Mismatch score
        #[arg(short = 'X', default_value_t = -6)]
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
        /// Limit the number of sequence to align
        #[arg(short = 'n', default_value_t = 0)]
        limit: usize,
        /// The output file name [default: stdout]
        #[arg(short = 'o')]
        output: Option<String>,
        /// Print MSA in 'one seq one line'
        #[arg(short = 'L')]
        line_format: bool,
        /// Print MSA with color
        #[arg(short = 'C')]
        color: bool,
        /// fasta files
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
                AlignCliMode::Kmer => {
                    eprintln!("Only `edit` command supports kmer mode");
                    std::process::exit(1);
                }
                _ => {}
            }
            let records = read_records(files, 2);
            align(
                &records[0],
                &records[1],
                mode,
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
            edit(&records[0], &records[1], mode, bandwidth, ksize);
        }
        Commands::Poa {
            mode,
            bandwidth,
            match_score,
            mismatch_score,
            gap_open_score,
            gap_extend_score,
            gap2_open_score,
            gap2_extend_score,
            line_format,
            color,
            limit,
            files,
            output,
        } => {
            let mut align_score = AlignScore::poa_default();
            align_score.M = match_score;
            align_score.X = mismatch_score;
            align_score.O = gap_open_score;
            align_score.E = gap_extend_score;
            align_score.Q = gap2_open_score;
            align_score.P = gap2_extend_score;

            let mut poa_params = BsPoaParam::default();
            poa_params = poa_params
                .set_bandwidth(bandwidth as u32)
                .set_score(align_score)
                .set_alnmode(mode.into());

            let mut poa = BsPoaAligner::new(poa_params);
            let limit = if limit == 0 { usize::MAX } else { limit };
            let mut count = 0;
            for file in files {
                let mut reader = FastaReaderBuilder::default()
                    .build_from_path(file)
                    .expect("Failed to open file");
                for record in reader.records() {
                    let record = record.expect("Failed to read record");
                    poa.add_sequence(record.sequence());
                    count += 1;
                    if count >= limit {
                        break;
                    }
                }
            }
            poa.align();
            poa.tidy_msa();
            poa.call_snvs();
            let (color, output) = match output {
                Some(file) => (0, file),
                None => (color as i32, String::from("-\0")),
            };
            let linewidth = if line_format { 0 } else { 100 };
            unsafe {
                let mut label = String::from("BSALIGN");
                let cfile = bindings::c_file_open(output.as_ptr() as *mut c_char);

                bindings::bspoa_print_msa(
                    poa.poa,
                    label.as_mut_ptr() as *mut c_char,
                    0,
                    0,
                    linewidth,
                    color,
                    cfile,
                );
                bindings::bspoa_print_snvs(poa.poa, label.as_mut_ptr() as *mut c_char, cfile);
                bindings::c_file_close(cfile);
            }
        }
    }
}
