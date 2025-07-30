use bsalign::poa::{BsPoaAligner, BsPoaParam};

fn main() {
    let param = BsPoaParam::default();
    let mut poa = BsPoaAligner::new(param);
    let seq1 = "ACGTACGTACGTACCGTACGTACGTACGTACGTACGTACGTACGT";
    let seq2 = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    let seq3 = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";

    poa.add_sequence(seq1);
    poa.add_sequence(seq2);
    poa.add_sequence(seq3);
    poa.align();
    poa.call_cns();
    let consensus = poa.get_cns();
    let consensus = consensus.as_string();
    println!("CNS: {}", consensus);
    let qlt = poa.get_qlt();
    let qlt = qlt.as_string();
    println!("QLT: {}", qlt);
    let alt = poa.get_alt();
    let alt = alt.as_string();
    println!("ALT: {}", alt);
    for i in 0..3 {
        let alignment = poa.get_alignment(i).unwrap();
        println!("     {}", alignment.as_string());
    }
    let ret = poa.get_alignment_result();
    for alignment in ret {
        println!("     {}", alignment.as_string());
    }
}
