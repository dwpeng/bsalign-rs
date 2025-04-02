use bsalign::poa::{BsPoaAligner, BsPoaParam};

fn main() {
    let param = BsPoaParam::default();
    let mut poa = BsPoaAligner::new(param);
    let seq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    poa.add_sequence(seq);
    poa.add_sequence(seq);
    poa.add_sequence(seq);
    poa.align();
    let consensus = poa.get_cns();
    let consensus = consensus.as_string();
    println!("CNS: {}", consensus);
    let qlt = poa.get_qlt();
    let qlt = qlt.as_string();
    println!("QLT: {}", qlt);
    let alt = poa.get_alt();
    let alt = alt.as_string();
    println!("ALT: {}", alt);
}
