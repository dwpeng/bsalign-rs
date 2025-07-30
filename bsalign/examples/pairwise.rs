use bsalign::pairwise::{BsPairwirseAligner, BsPairwiseParam};

fn main() {
    let tseq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    let qseq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTAC";
    let param = BsPairwiseParam::default().set_ksize(4);
    let mut aligner = BsPairwirseAligner::new(param);
    let result = aligner.align_banded_striped_8bit(&tseq, &qseq);
    assert_eq!(result.aln, tseq.len());
    let alnstr = result.to_string();
    println!(
        "{}\n{}\n{}",
        alnstr.tseq().to_string(),
        alnstr.alignment().to_string(),
        alnstr.qseq().to_string(),
    );
    println!("Alignment result: {}", result);
}
