use bsalign::{
    BitSeq,
    poa::{BsPoaAligner, BsPoaParam},
};

fn main() {
    let param = BsPoaParam::default();
    let mut poa = BsPoaAligner::new(param);
    let seq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    poa.add_sequence(seq);
    poa.add_sequence(seq);
    poa.add_sequence(seq);
    poa.align();

    let consensus = poa.get_cns();
    let bitseq: BitSeq = consensus.into();
    let consensus = bitseq.to_string();
    println!("Consensus: {}", consensus);
}
