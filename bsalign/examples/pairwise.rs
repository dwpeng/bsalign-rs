use bsalign::pairwise::{BsPairwirseAligner, BsPairwiseParam};

fn main() {
    let seq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    let param = BsPairwiseParam::default().set_ksize(4);
    let mut aligner = BsPairwirseAligner::new(param);
    let result = aligner.align_banded_striped_8bit(&seq, &seq);
    assert_eq!(result.aln, seq.len());
    let alnstr = result.to_string();
    println!(
        "{}\n{}\n{}",
        alnstr.tseq().to_string(),
        alnstr.alignment().to_string(),
        alnstr.qseq().to_string(),
    );
    println!("Alignment result: {}", result);
}
