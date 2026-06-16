use std::{
    ops::{Deref, DerefMut},
    os::raw::c_char,
};

use crate::{AlignMode, AlignScore};
use bsalign_sys::bindings;

#[derive(Debug)]
pub struct BsPoaParam(bindings::BSPOAPar);

impl Deref for BsPoaParam {
    type Target = bindings::BSPOAPar;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BsPoaParam {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct BsPoaAligner {
    pub params: BsPoaParam,
    pub poa: *mut bindings::BSPOA,
    metainfo: Option<String>,
    aligned: bool,
    pub nseq: usize,
}

impl Drop for BsPoaAligner {
    fn drop(&mut self) {
        unsafe {
            bindings::bspoa_free(self.poa);
        }
    }
}

impl BsPoaAligner {
    pub fn new(params: BsPoaParam) -> Self {
        let poa = unsafe {
            let poa = bindings::bspoa_init(params.0.clone());
            bindings::bspoa_begin(poa);
            poa
        };
        BsPoaAligner {
            params,
            poa,
            metainfo: None,
            aligned: false,
            nseq: 0,
        }
    }
}

#[inline]
fn base_from_2bit(base: u8) -> u8 {
    match base {
        0 => b'A',
        1 => b'C',
        2 => b'G',
        3 => b'T',
        _ => b'N',
    }
}

#[derive(Debug)]
pub struct Consensus<'a> {
    inner: &'a [u8],
    convert: bool,
}

impl<'a> Consensus<'a> {
    pub fn as_string(&self) -> String {
        if self.convert {
            let mut s = String::with_capacity(self.len());
            for i in 0..self.len() {
                let base = self.get_base(i).unwrap();
                s.push(base as char);
            }
            return s;
        }
        unsafe {
            let s = str::from_utf8_unchecked(self.inner);
            s.into()
        }
    }

    pub fn to_vec(&self, buf: &mut Vec<u8>) {
        buf.clear();
        if self.convert {
            for i in 0..self.len() {
                let base = self.get_base(i).unwrap();
                buf.push(base);
            }
        } else {
            buf.extend_from_slice(self.inner);
        }
    }

    pub fn get_bit(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        let bit = self.inner[index];
        Some(bit)
    }

    pub fn get_base(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        let bit = self.inner[index];
        Some(base_from_2bit(bit))
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Debug)]
pub struct Quality<'a> {
    inner: &'a [u8],
}

impl<'a> Quality<'a> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.len());
        for i in 0..self.len() {
            let base = self.get(i).unwrap();
            s.push((b'!' + base) as char);
        }
        s
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        Some(self.inner[index])
    }
}

#[derive(Debug)]
pub struct Alt<'a> {
    inner: &'a [u8],
}

impl<'a> Alt<'a> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.len());
        for i in 0..self.len() {
            let base = self.get(i).unwrap();
            s.push((b'!' + base) as char);
        }
        s
    }

    pub fn get(&self, index: usize) -> Option<u8> {
        if index >= self.len() {
            return None;
        }
        Some(self.inner[index])
    }
}

pub struct PoaAlignResult<'a> {
    idx: usize,
    poa: &'a BsPoaAligner,
}

impl<'a> Iterator for PoaAlignResult<'a> {
    type Item = AlignmentString<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.poa.nseq {
            return None;
        }
        let a = self.poa.get_alignment(self.idx);
        self.idx += 1;
        a
    }
}

#[derive(Debug)]
pub struct AlignmentString<'a> {
    inner: &'a [u8],
}

impl<'a> AlignmentString<'a> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn as_string(&self) -> String {
        unsafe {
            let s = str::from_utf8_unchecked(self.inner);
            s.into()
        }
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        self.inner
    }
}

impl BsPoaAligner {
    pub fn add_sequence<T>(&mut self, seq: &T)
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let s = seq.as_ref();
        unsafe {
            bindings::bspoa_add_sequence(self.poa, s.as_ptr(), s.len() as u32);
            self.aligned = false;
        }
        self.nseq += 1;
    }

    /// Reset poa so that start next loop align.
    /// !!Note: Call this method before `add_sequence`
    pub fn reset(&mut self) {
        unsafe {
            self.aligned = false;
            self.metainfo = None;
            self.nseq = 0;
            bindings::bspoa_begin(self.poa);
        }
    }

    pub fn align(&mut self) {
        unsafe {
            bindings::bspoa_end(self.poa);
            bindings::bspoa_tidy_msa(self.poa);
            self.aligned = true;
        }
    }

    pub fn get_alignment_result(&self) -> PoaAlignResult<'_> {
        if !self.aligned {
            panic!("Align sequences before calling dump_msa, call `align` first");
        }
        return PoaAlignResult { idx: 0, poa: self };
    }

    pub fn call_snvs(&mut self) {
        if !self.aligned {
            panic!("Align sequences before calling call_snvs, call `align` first");
        }
        unsafe {
            bindings::bspoa_call_snvs(self.poa);
        }
    }

    pub fn metainfo(&self) -> Option<&String> {
        self.metainfo.as_ref()
    }

    pub fn set_metainfo(&mut self, metainfo: String) {
        self.metainfo = Some(metainfo);
    }

    pub fn dump_msa(&self, filename: &str) {
        if !self.aligned {
            panic!("Align sequences before calling dump_msa, call `align` first");
        }
        unsafe {
            let cfilename = std::ffi::CString::new(filename).unwrap();
            if let Some(metainfo) = self.metainfo() {
                bindings::bspoa_dump_binary_msa(
                    self.poa,
                    metainfo.as_ptr() as *mut i8,
                    metainfo.len() as u32,
                    cfilename.as_ptr() as *mut i8,
                );
            } else {
                bindings::bspoa_dump_binary_msa(
                    self.poa,
                    std::ptr::null_mut(),
                    0,
                    cfilename.as_ptr() as *mut i8,
                );
            }
        }
    }

    pub fn get_cns_with_indel(&self) -> Consensus<'_> {
        if !self.aligned {
            panic!("POA must be aligned before getting consensus");
        }
        let alignment = self.get_alignment(self.nseq);
        Consensus {
            inner: alignment.unwrap().inner,
            convert: false,
        }
    }

    pub fn get_cns(&self) -> Consensus<'_> {
        if !self.aligned {
            panic!("Align sequences before calling get_consensus, call `align` first");
        }
        let c = unsafe {
            let mut len = 0;
            let cns = bindings::bspoa_get_cns(self.poa, &mut len);
            std::slice::from_raw_parts(cns, len as usize)
        };
        Consensus {
            inner: c,
            convert: true,
        }
    }

    pub fn get_qlt(&self) -> Quality<'_> {
        if !self.aligned {
            panic!("Align sequences before calling get_qlt, call `align` first");
        }
        let q = unsafe {
            let mut len = 0;
            let qlt = bindings::bspoa_get_qlt(self.poa, &mut len);
            std::slice::from_raw_parts(qlt, len as usize)
        };
        Quality { inner: q }
    }

    pub fn get_alignment(&self, idx: usize) -> Option<AlignmentString<'_>> {
        if !self.aligned {
            panic!("Align sequences before calling get_qlt, call `align` first");
        }
        if idx > self.nseq {
            return None;
        }
        let idx = idx + 1;
        let mut len: usize = 0;
        let inner = unsafe {
            let buffer = bindings::bspoa_get_rid_alignment(self.poa, idx as i32, &mut len);
            assert!(len > 1, "Alignment length should be greater than 0");
            std::slice::from_raw_parts(buffer, len - 1)
        };
        Some(AlignmentString { inner })
    }

    pub fn get_alt(&self) -> Alt<'_> {
        if !self.aligned {
            panic!("Align sequences before calling get_alt, call `align` first");
        }
        let a = unsafe {
            let mut len = 0;
            let alt = bindings::bspoa_get_alt(self.poa, &mut len);
            std::slice::from_raw_parts(alt, len as usize)
        };
        Alt { inner: a }
    }
    /// Print SNVs to a file
    pub fn print_snvs(&self, label: Option<&str>, filename: &str) {
        if !self.aligned {
            panic!("Align sequences before calling print_snvs, call `align` first");
        }
        unsafe {
            let cfilename = std::ffi::CString::new(filename).unwrap();
            let clabel = match label {
                None => std::ffi::CString::new("SNV").unwrap(),
                Some(s) => std::ffi::CString::new(s).unwrap(),
            };

            let cfile = bindings::c_file_open(cfilename.as_ptr() as *mut c_char);
            bindings::bspoa_print_snvs(self.poa, clabel.as_ptr() as *mut i8, cfile);
            bindings::c_file_close(cfile);
        }
    }
}

impl BsPoaAligner {
    pub fn load_msa(filename: &str) -> Option<BsPoaAligner> {
        let p = BsPoaParam::default();
        let mut poa = BsPoaAligner::new(p);
        let cfilename = std::ffi::CString::new(filename).unwrap();
        unsafe {
            let s = bindings::string_init(32);
            let ret = bindings::bspoa_load_binary_msa(poa.poa, cfilename.as_ptr() as *mut i8, s);
            if ret != 0 {
                bindings::string_free(s);
                return None;
            }
            let olds = s;
            let s = s.as_ref().unwrap();
            if s.size > 0 {
                let slice = std::slice::from_raw_parts(s.string as *const u8, s.size as usize);
                let metainfo = String::from_utf8_lossy(slice).to_string();
                poa.set_metainfo(metainfo);
            }
            bindings::string_free(olds);
            Some(poa)
        }
    }
}

impl Default for BsPoaParam {
    fn default() -> Self {
        let p = bindings::BSPOAPar {
            refmode: 0,
            shuffle: 1,
            alnmode: bindings::SEQALIGN_MODE_OVERLAP as i32,
            realn: 3,
            seqcore: 40,
            nrec: 20,
            ksz: 15,
            bwtrigger: 1,
            bandwidth: 128,
            M: 2,
            X: -6,
            O: -3,
            E: -2,
            Q: -8,
            P: -1,
            T: 20,
            refbonus: 1,
            editbw: 64,
            althi: 5,
            qlthi: 70,
            psub: 0.10,
            pins: 0.10,
            pdel: 0.15,
            piex: 0.15,
            pdex: 0.20,
            hins: 0.20,
            hdel: 0.40,
            min_varcnt: 3,
            min_covfrq: 0.5,
            min_snvqlt: 5,
        };
        BsPoaParam(p)
    }
}

#[derive(Debug)]
pub enum PoaParamRefMode {
    NoReference = 0,
    Reference = 1,
}

impl Default for PoaParamRefMode {
    fn default() -> Self {
        PoaParamRefMode::NoReference
    }
}

#[derive(Debug)]
pub struct PoaParamCns {
    pub psub: f32,
    pub pins: f32,
    pub pdel: f32,
    pub piex: f32,
    pub pdex: f32,
    pub hins: f32,
    pub hdel: f32,
}

impl Default for PoaParamCns {
    fn default() -> Self {
        PoaParamCns {
            psub: 0.10,
            pins: 0.10,
            pdel: 0.15,
            piex: 0.15,
            pdex: 0.20,
            hins: 0.20,
            hdel: 0.40,
        }
    }
}

impl BsPoaParam {
    /// Set reference mode
    /// 1: The first sequence is the reference
    /// 0: No reference
    pub fn set_refmode(self, refmode: PoaParamRefMode) -> Self {
        let mut p = self;
        p.refmode = refmode as i32;
        p
    }

    /// whether to shuffle/sort the reads according to most kmers matches first
    pub fn set_shuffle(self, enable: bool) -> Self {
        let mut p = self;
        if enable {
            p.shuffle = 1;
        } else {
            p.shuffle = 0;
        }
        p
    }

    /// Set alignment mode
    ///
    /// global/extend/overlap
    pub fn set_alnmode(self, alnmode: AlignMode) -> Self {
        let mut p = self;
        p.alnmode = alnmode as i32;
        p
    }

    /// Set rounds of realignment
    pub fn set_realn(self, realn: u32) -> Self {
        let mut p = self;
        p.realn = realn as i32;
        p
    }

    /// number of seqs in core MSA, addtional reads will be realigned (realn > 0) against core MSA profile to build a full MSA
    pub fn set_seqcore(self, seqcore: u32) -> Self {
        let mut p = self;
        p.seqcore = seqcore;
        p
    }

    /// every query read is aligning against previous <nrec> reads on graph, 0 to all the previous
    pub fn set_nrec(self, nrec: u32) -> Self {
        let mut p = self;
        p.nrec = nrec as i32;
        p
    }

    /// kmer size for graph construction
    pub fn set_ksz(self, ksz: u32) -> Self {
        let mut p = self;
        p.ksz = ksz as i32;
        p
    }

    /// when <trigger> > 0 and <-W> < query length, genrates CNS per after <trigger> reads, and trigger banded alignment
    pub fn set_bwtrigger(self, bwtrigger: i32) -> Self {
        let mut p = self;
        p.bwtrigger = bwtrigger;
        p
    }

    /// bandwidth for realignment
    pub fn set_bandwidth(self, bandwidth: u32) -> Self {
        let mut p = self;
        p.bandwidth = bandwidth as i32;
        p
    }

    /// Set score parameters
    pub fn set_score(self, score: AlignScore) -> Self {
        let mut p = self;
        p.M = score.M;
        p.X = score.X;
        p.O = score.O;
        p.E = score.E;
        p.Q = score.Q;
        p.P = score.P;
        p
    }

    /// base match score on reference will be M + refbonus
    pub fn set_refbonus(self, refbonus: u32) -> Self {
        let mut p = self;
        p.refbonus = refbonus as i32;
        p
    }

    /// Set edit bandwidth
    pub fn set_editbw(self, editbw: u32) -> Self {
        let mut p = self;
        p.editbw = editbw as i32;
        p
    }

    /// cutoff of high alt base quality, used in tidying out possible SNV sites
    pub fn set_althi(self, althi: u32) -> Self {
        let mut p = self;
        p.althi = althi as i32;
        p
    }

    /// cutoff of high cns base quality, used in print colorful MSA
    pub fn set_qlthi(self, qlthi: u32) -> Self {
        let mut p = self;
        p.qlthi = qlthi as i32;
        p
    }

    /// min count of variant base in SNV sites
    pub fn set_varcnt(self, min_varcnt: u32) -> Self {
        let mut p = self;
        p.min_varcnt = min_varcnt as i32;
        p
    }

    /// min spanned_reads / total_reads in SNV sites
    pub fn set_covfrq(self, min_covfrq: f32) -> Self {
        let mut p = self;
        p.min_covfrq = min_covfrq;
        p
    }

    /// -log10(p-value), 5 ~= 1e-5
    pub fn set_snvqlt(self, min_snvqlt: u32) -> Self {
        let mut p = self;
        p.min_snvqlt = min_snvqlt;
        p
    }

    pub fn set_consensus_param(self, cns: PoaParamCns) -> Self {
        let mut p = self;
        p.psub = cns.psub;
        p.pins = cns.pins;
        p.pdel = cns.pdel;
        p.piex = cns.piex;
        p.pdex = cns.pdex;
        p.hins = cns.hins;
        p.hdel = cns.hdel;
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poa() {
        let bspoa_params = BsPoaParam::default().set_varcnt(1);
        let seq1 = "TCATCTGATGCTAGTAcccccccc";
        let seq2 = "TCCTGATCTAGCTAGTA";
        let seq3 = "TCGATCTGATAGCTAGTA";
        let mut poa = BsPoaAligner::new(bspoa_params);
        poa.add_sequence(seq1);
        poa.add_sequence(seq2);
        poa.add_sequence(seq3);
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "TCATCTGATTAGCTAGTACCCCCCCC");
        let qlt = poa.get_qlt();
        assert_eq!(qlt.len(), cns.len());
        poa.set_metainfo("test".to_string());
        poa.dump_msa("test.msa");
        let poa = BsPoaAligner::load_msa("test.msa");
        assert!(poa.is_some());
        if let Some(poa) = poa {
            assert_eq!(poa.metainfo().unwrap(), "test");
        }
        std::fs::remove_file("test.msa").unwrap();
    }

    #[test]
    fn test_identical_sequences() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        let seq = "ACGTACGTACGT";
        poa.add_sequence(seq);
        poa.add_sequence(seq);
        poa.add_sequence(seq);
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGT");
    }

    #[test]
    fn test_single_sequence() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGT");
        assert_eq!(poa.nseq, 1);
    }

    #[test]
    fn test_consensus_reflects_majority() {
        // 3 sequences agree on A, 1 disagrees with T at position 4
        let mut poa = BsPoaAligner::new(BsPoaParam::default().set_varcnt(1));
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT"); // A instead of T at pos 3
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_insertion_in_one_sequence() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTNNNACGT"); // insertion of NNN
        poa.add_sequence("ACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        // 2/3 sequences have no insertion, consensus should match the majority
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_deletion_in_one_sequence() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGACGT"); // deletion of T at pos 3
        poa.add_sequence("ACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        // 2/3 sequences have the T, consensus should match the majority
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_alignment_iterator() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.align();
        let results: Vec<AlignmentString> = poa.get_alignment_result().collect();
        assert_eq!(results.len(), 3);
        for r in &results {
            assert!(r.len() > 0);
            let _ = r.as_string();
        }
    }

    #[test]
    fn test_get_alignment_by_index() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.align();
        assert!(poa.get_alignment(0).is_some());
        assert!(poa.get_alignment(1).is_some());
        // nseq is 2, get_alignment(nseq) returns the consensus row
        assert!(poa.get_alignment(2).is_some());
        // out of range
        assert!(poa.get_alignment(100).is_none());
    }

    #[test]
    fn test_quality_values() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.align();
        let qlt = poa.get_qlt();
        assert!(qlt.len() > 0);
        // Quality values should be valid (0-93 range for Phred+33 encoding)
        for i in 0..qlt.len() {
            let q = qlt.get(i).unwrap();
            assert!(
                q <= 93,
                "quality value {} at index {} exceeds Phred range",
                q,
                i
            );
        }
        let qlt_str = qlt.as_string();
        assert_eq!(qlt_str.len(), qlt.len());
    }

    #[test]
    fn test_quality_out_of_bounds() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.align();
        let qlt = poa.get_qlt();
        assert!(qlt.get(qlt.len()).is_none());
    }

    #[test]
    fn test_consensus_accessors() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.align();
        let cns = poa.get_cns();
        // get_base returns ASCII bases
        assert_eq!(cns.get_base(0), Some(b'A'));
        assert_eq!(cns.get_base(1), Some(b'C'));
        assert_eq!(cns.get_base(2), Some(b'G'));
        assert_eq!(cns.get_base(3), Some(b'T'));
        assert_eq!(cns.get_base(4), None);
        // get_bit returns 2-bit encoded values (0=A, 1=C, 2=G, 3=T)
        assert_eq!(cns.get_bit(0), Some(0));
        assert_eq!(cns.get_bit(1), Some(1));
        assert_eq!(cns.get_bit(2), Some(2));
        assert_eq!(cns.get_bit(3), Some(3));
        assert_eq!(cns.get_bit(4), None);
    }

    #[test]
    fn test_alt_values() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default().set_varcnt(1));
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT"); // A instead of T at pos 3
        poa.align();
        let alt = poa.get_alt();
        assert!(alt.len() > 0);
        for i in 0..alt.len() {
            assert!(alt.get(i).is_some());
        }
        assert_eq!(alt.as_string().len(), alt.len());
    }

    #[test]
    fn test_alt_out_of_bounds() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.align();
        let alt = poa.get_alt();
        assert!(alt.get(alt.len()).is_none());
    }

    #[test]
    fn test_reset_and_reuse() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        // First round
        poa.add_sequence("AAAA");
        poa.add_sequence("AAAA");
        poa.align();
        assert_eq!(poa.get_cns().as_string(), "AAAA");
        assert_eq!(poa.nseq, 2);

        // Reset for second round
        poa.reset();
        assert_eq!(poa.nseq, 0);
        poa.add_sequence("CCCC");
        poa.add_sequence("CCCC");
        poa.add_sequence("CCCC");
        poa.align();
        assert_eq!(poa.get_cns().as_string(), "CCCC");
        assert_eq!(poa.nseq, 3);
    }

    #[test]
    fn test_alignment_string_as_bytes() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.align();
        let aln = poa.get_alignment(0).unwrap();
        let bytes = aln.as_bytes();
        assert_eq!(bytes.len(), aln.len());
        assert_eq!(aln.as_string().as_bytes(), bytes);
    }

    #[test]
    fn test_nseq_tracking() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        assert_eq!(poa.nseq, 0);
        poa.add_sequence("ACGT");
        assert_eq!(poa.nseq, 1);
        poa.add_sequence("ACGT");
        assert_eq!(poa.nseq, 2);
        poa.add_sequence("ACGT");
        assert_eq!(poa.nseq, 3);
    }

    #[test]
    fn test_param_builder_chain() {
        let param = BsPoaParam::default()
            .set_refmode(PoaParamRefMode::Reference)
            .set_shuffle(false)
            .set_alnmode(AlignMode::Global)
            .set_realn(5)
            .set_seqcore(10)
            .set_nrec(10)
            .set_ksz(11)
            .set_bwtrigger(0)
            .set_bandwidth(64)
            .set_score(AlignScore {
                M: 1,
                X: -4,
                O: -2,
                E: -1,
                Q: -6,
                P: -1,
            })
            .set_refbonus(2)
            .set_editbw(32)
            .set_althi(10)
            .set_qlthi(50)
            .set_varcnt(5)
            .set_covfrq(0.8)
            .set_snvqlt(10)
            .set_consensus_param(PoaParamCns {
                psub: 0.05,
                pins: 0.05,
                pdel: 0.10,
                piex: 0.10,
                pdex: 0.15,
                hins: 0.15,
                hdel: 0.30,
            });

        assert_eq!(param.refmode, PoaParamRefMode::Reference as i32);
        assert_eq!(param.shuffle, 0);
        assert_eq!(param.alnmode, AlignMode::Global as i32);
        assert_eq!(param.realn, 5);
        assert_eq!(param.seqcore, 10);
        assert_eq!(param.M, 1);
        assert_eq!(param.X, -4);
        assert_eq!(param.refbonus, 2);
        assert_eq!(param.editbw, 32);
        assert_eq!(param.psub, 0.05);
    }

    #[test]
    fn test_refmode_first_sequence_as_reference() {
        let param = BsPoaParam::default()
            .set_refmode(PoaParamRefMode::Reference)
            .set_varcnt(1);
        let mut poa = BsPoaAligner::new(param);
        // First sequence is the reference
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT"); // variant at pos 3
        poa.add_sequence("ACGAACGT"); // variant at pos 3
        poa.align();
        let cns = poa.get_cns();
        let cns_str = cns.as_string();
        // With reference mode, consensus should be influenced by the reference
        assert!(cns_str.len() > 0);
    }

    #[test]
    fn test_many_sequences() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default().set_varcnt(1));
        let base = "ACGTACGTACGTACGT";
        for _ in 0..20 {
            poa.add_sequence(base);
        }
        // Add one sequence with a variant
        poa.add_sequence("ACGTACGAACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        // 20/21 sequences agree, so consensus should match the majority
        assert_eq!(cns.as_string(), base);
    }

    #[test]
    fn test_dump_and_load_msa_without_metainfo() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.align();
        // Dump without setting metainfo
        poa.dump_msa("test_no_meta.msa");
        let loaded = BsPoaAligner::load_msa("test_no_meta.msa");
        assert!(loaded.is_some());
        if let Some(loaded) = loaded {
            assert!(loaded.metainfo().is_none());
        }
        std::fs::remove_file("test_no_meta.msa").unwrap();
    }

    #[test]
    #[should_panic(expected = "Align sequences before calling")]
    fn test_get_cns_before_align_panics() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.get_cns(); // should panic
    }

    #[test]
    #[should_panic(expected = "Align sequences before calling")]
    fn test_get_qlt_before_align_panics() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.get_qlt(); // should panic
    }

    #[test]
    #[should_panic(expected = "Align sequences before calling")]
    fn test_get_alignment_result_before_align_panics() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.get_alignment_result(); // should panic
    }

    #[test]
    #[should_panic(expected = "Align sequences before calling")]
    fn test_call_snvs_before_align_panics() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.call_snvs(); // should panic
    }

    #[test]
    #[should_panic(expected = "Align sequences before calling")]
    fn test_get_alt_before_align_panics() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.get_alt(); // should panic
    }

    #[test]
    fn test_cns_with_indel() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.align();
        let cns_indel = poa.get_cns_with_indel();
        let s = cns_indel.as_string();
        // get_cns_with_indel returns the raw alignment row (may include trailing gaps)
        assert!(s.starts_with("ACGTACGT"));
        assert!(cns_indel.len() > 0);
    }

    #[test]
    fn test_call_snvs_and_print() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default().set_varcnt(1));
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT"); // variant at pos 3
        poa.align();
        poa.call_snvs();
        poa.print_snvs(Some("test_label"), "/dev/null");
    }

    #[test]
    fn test_print_snvs_no_label() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default().set_varcnt(1));
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT");
        poa.align();
        poa.call_snvs();
        poa.print_snvs(None, "/dev/null");
    }

    #[test]
    fn test_get_alignment_result_count_matches_nseq() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.align();
        let count = poa.get_alignment_result().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_long_sequences() {
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        // Generate two 1000bp sequences with a few variants
        let mut seq1 = String::with_capacity(1000);
        let mut seq2 = String::with_capacity(1000);
        for i in 0..1000 {
            seq1.push(match i % 4 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T',
            });
            seq2.push(match i % 4 {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                _ => 'T',
            });
        }
        // Introduce a few mutations in seq2
        unsafe {
            seq2.as_bytes_mut()[100] = b'T';
        }
        unsafe {
            seq2.as_bytes_mut()[500] = b'A';
        }
        unsafe {
            seq2.as_bytes_mut()[900] = b'C';
        }

        poa.add_sequence(&seq1);
        poa.add_sequence(&seq2);
        poa.align();
        let cns = poa.get_cns();
        // 2 sequences with only 3 mutations out of 1000bp — majority wins
        assert_eq!(cns.as_string(), seq1);
    }

    #[test]
    fn test_dna_bases() {
        // Verify all 4 DNA bases are handled correctly in consensus
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        poa.add_sequence("ACGT");
        poa.add_sequence("ACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.get_base(0), Some(b'A'));
        assert_eq!(cns.get_base(1), Some(b'C'));
        assert_eq!(cns.get_base(2), Some(b'G'));
        assert_eq!(cns.get_base(3), Some(b'T'));
    }

    #[test]
    fn test_bytes_input() {
        // Verify sequences can be added as byte slices
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        let seq: &[u8] = b"ACGTACGT";
        poa.add_sequence(seq);
        poa.add_sequence(seq);
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_vec_u8_input() {
        // Verify sequences can be added as Vec<u8>
        let mut poa = BsPoaAligner::new(BsPoaParam::default());
        let seq: Vec<u8> = b"ACGTACGT".to_vec();
        poa.add_sequence(&seq);
        poa.add_sequence(&seq);
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_bandwidth_default() {
        let param = BsPoaParam::default();
        assert_eq!(param.bandwidth, 128);
    }

    #[test]
    fn test_bandwidth_small() {
        // Small bandwidth should still produce correct consensus for identical sequences
        let param = BsPoaParam::default().set_bandwidth(4).set_editbw(4);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGT");
    }

    #[test]
    fn test_bandwidth_large() {
        // Large bandwidth for high-divergence sequences
        let param = BsPoaParam::default().set_bandwidth(512).set_editbw(256);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGTACGTACGT");
    }

    #[test]
    fn test_bandwidth_affects_alignment_with_indels() {
        // Sequences with an insertion — bandwidth should handle the gap
        let param = BsPoaParam::default().set_bandwidth(32).set_editbw(32);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGTAAAACGTACGT"); // insertion of AAA
        poa.add_sequence("ACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        // 2/3 sequences have no insertion, consensus should match the majority
        assert_eq!(cns.as_string(), "ACGTACGTACGT");
    }

    #[test]
    fn test_bwtrigger_disabled() {
        // bwtrigger=0 disables banded triggering
        let param = BsPoaParam::default().set_bwtrigger(0).set_bandwidth(64);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGT");
    }

    #[test]
    fn test_bwtrigger_enabled() {
        // bwtrigger > 0 enables banded alignment after trigger reads
        let param = BsPoaParam::default()
            .set_bwtrigger(2)
            .set_bandwidth(64)
            .set_editbw(32);
        let mut poa = BsPoaAligner::new(param);
        for _ in 0..5 {
            poa.add_sequence("ACGTACGTACGTACGT");
        }
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGTACGT");
    }

    #[test]
    fn test_editbw_small() {
        // Small edit bandwidth — works for identical or near-identical sequences
        let param = BsPoaParam::default().set_editbw(8);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGTACGT");
        poa.add_sequence("ACGAACGT"); // 1 substitution
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGT");
    }

    #[test]
    fn test_editbw_large() {
        // Large edit bandwidth for sequences with more gaps
        let param = BsPoaParam::default().set_editbw(128);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGTACGT");
        poa.add_sequence("ACGTACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGTACGT");
    }

    #[test]
    fn test_bandwidth_and_editbw_combined() {
        // Both bandwidth and editbw set to moderate values
        let param = BsPoaParam::default()
            .set_bandwidth(16)
            .set_editbw(16)
            .set_bwtrigger(1);
        let mut poa = BsPoaAligner::new(param);
        let seqs: Vec<&str> = vec![
            "ACGTACGTACGTACGTACGT",
            "ACGTACGTACGTACGTACGT",
            "ACGTACGTACGAACGTACGT", // 1 substitution
            "ACGTACGTACGTACGTACGT",
        ];
        for s in &seqs {
            poa.add_sequence(s);
        }
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGTACGTACGT");
    }

    #[test]
    fn test_bandwidth_with_many_sequences() {
        // Many sequences with moderate bandwidth
        let param = BsPoaParam::default()
            .set_bandwidth(64)
            .set_editbw(64)
            .set_seqcore(10);
        let mut poa = BsPoaAligner::new(param);
        for _ in 0..15 {
            poa.add_sequence("ACGTACGTACGTACGT");
        }
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGTACGT");
    }

    #[test]
    fn test_bandwidth_with_snv_detection() {
        // Bandwidth settings should not break SNV detection
        let param = BsPoaParam::default()
            .set_bandwidth(32)
            .set_editbw(32)
            .set_varcnt(1);
        let mut poa = BsPoaAligner::new(param);
        poa.add_sequence("ACGTACGTACGT");
        poa.add_sequence("ACGAACGTACGT"); // SNV at pos 3: T->A
        poa.add_sequence("ACGTACGTACGT");
        poa.align();
        let cns = poa.get_cns();
        assert_eq!(cns.as_string(), "ACGTACGTACGT");
        // Alt should reflect the variant
        let alt = poa.get_alt();
        assert!(alt.len() > 0);
    }

    /// Read current process RSS in bytes from /proc/self/statm (Linux only).
    fn current_rss_bytes() -> usize {
        let statm = std::fs::read_to_string("/proc/self/statm").unwrap();
        // Field 1 is RSS in pages
        let rss_pages: usize = statm.split_whitespace().nth(1).unwrap().parse().unwrap();
        let page_size = 4096usize; // standard page size on x86_64 Linux
        rss_pages * page_size
    }

    #[test]
    fn test_reset_memory_stable() {
        // Verify that repeatedly reusing an aligner via reset() does not cause
        // unbounded memory growth.  Run many iterations with varying-length
        // sequences and assert that RSS stabilises after an initial warm-up.
        let params = BsPoaParam::default()
            .set_refmode(PoaParamRefMode::Reference)
            .set_alnmode(AlignMode::Overlap)
            .set_realn(0)
            .set_ksz(15)
            .set_bandwidth(32)
            .set_editbw(32)
            .set_shuffle(true);
        let mut poa = BsPoaAligner::new(params);

        let warmup_iters = 32; // let buffers reach high-water mark
        let measure_iters = 128; // measure after stabilisation
        let seq_count = 30;
        let seq_len = 5000;

        // Pre-allocate all sequences once to avoid allocator noise in RSS.
        let mut rng_seed: u64 = 0x1234_5678_9ABC_DEF0;
        let next_u64 = |seed: &mut u64| -> u64 {
            *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            *seed
        };
        let all_seqs: Vec<Vec<Vec<u8>>> = (0..warmup_iters + measure_iters)
            .map(|_| {
                (0..seq_count)
                    .map(|_| {
                        let mut s = Vec::with_capacity(seq_len);
                        for _ in 0..seq_len {
                            let val = next_u64(&mut rng_seed);
                            let base = match (val >> 33) & 0x3 {
                                0 => b'A',
                                1 => b'C',
                                2 => b'G',
                                _ => b'T',
                            };
                            s.push(base);
                        }
                        s
                    })
                    .collect()
            })
            .collect();

        // --- warm-up phase: fill buffers to their high-water mark ---
        for (idx, batch) in all_seqs[..warmup_iters].iter().enumerate() {
            if idx % 16 == 0 {
                eprintln!("[warmup {}/{}]", idx, warmup_iters);
            }
            poa.reset();
            for seq in batch {
                poa.add_sequence(seq);
            }
            poa.align();
            let _ = poa.get_cns();
        }

        // --- measurement phase: collect RSS samples ---
        // All sequences are pre-allocated so the only varying factor is the
        // POA aligner's internal memory.
        let mut samples = Vec::with_capacity(measure_iters / 16 + 1);
        for (i, batch) in all_seqs[warmup_iters..].iter().enumerate() {
            if i % 64 == 0 {
                eprintln!("[measure {}/{}]", i, measure_iters);
            }
            poa.reset();
            for seq in batch {
                poa.add_sequence(seq);
            }
            poa.align();
            let _ = poa.get_cns();
            if i % 16 == 0 {
                samples.push(current_rss_bytes());
            }
        }

        // --- assertions ---
        // Compare the first third of samples against the last third.
        // RSS should not grow by more than 5 % after warm-up.
        let n = samples.len();
        let early: f64 = samples[..n / 3].iter().sum::<usize>() as f64 / (n / 3) as f64;
        let late: f64 = samples[n * 2 / 3..].iter().sum::<usize>() as f64 / (n - n * 2 / 3) as f64;
        let growth_pct = ((late - early) / early) * 100.0;

        eprintln!(
            "Memory test: early_avg={:.2} MB, late_avg={:.2} MB, growth={:.2}%",
            early / (1024.0 * 1024.0),
            late / (1024.0 * 1024.0),
            growth_pct,
        );

        assert!(
            growth_pct < 5.0,
            "Memory grew by {:.2}% after warm-up — possible leak (early={:.2} MB, late={:.2} MB)",
            growth_pct,
            early / (1024.0 * 1024.0),
            late / (1024.0 * 1024.0),
        );
    }
}
