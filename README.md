# bsalign-rs

A rust binding for the [bsalign](https://github.com/ruanjue/bsalign) library.

## Install

```bash
cargo install bsalign
```

## Pairwise alignment

```rust
use bsalign::pairwise::{BsPairwirseAligner, BsPairwiseParam};

fn main() {
    let seq = "ACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGTACGT";
    let param = BsPairwiseParam::default().set_ksize(4);
    let mut aligner = BsPairwirseAligner::new(param);
    let result = aligner.align_banded_striped_8bit(&seq, &seq);
    assert_eq!(result.aln, qseq.len() as i32);
    let alnstr = result.to_string();
    println!(
        "{}\n{}\n{}",
        alnstr.tseq().to_string(),
        alnstr.alignment().to_string(),
        alnstr.qseq().to_string(),
    );
    println!("Alignment result: {}", result);
}
```

## Multiple sequence alignment

```rust
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

    let consensus = poa.get_consensus();
    let bitseq: BitSeq = consensus.into();
    let consensus = bitseq.to_string();
    println!("Consensus: {}", consensus);
}
```
