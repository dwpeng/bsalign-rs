# bsalign-rs

A rust binding for the [bsalign](https://github.com/ruanjue/bsalign) library.

## Install

```bash
cargo install bsalign
```
or install as library
```bash
cargo add bsalign
```

## Pairwise sequence alignment

```rust
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
```

## Multiple sequence alignment

```rust
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
```
