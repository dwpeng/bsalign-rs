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
    nseq: usize,
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
}

impl<'a> Consensus<'a> {
    pub fn as_string(&self) -> String {
        let mut s = String::with_capacity(self.len());
        for i in 0..self.len() {
            let base = self.get_base(i).unwrap();
            s.push(base as char);
        }
        s
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
            self.aligned = true;
        }
    }

    pub fn tidy_msa(&mut self) {
        if !self.aligned {
            panic!("Align sequences before calling tidy_msa, call `align` first");
        }
        unsafe {
            bindings::bspoa_tidy_msa(self.poa);
        }
    }

    pub fn msa(&mut self) -> u32 {
        if !self.aligned {
            panic!("Align sequences before calling msa, call `align` first");
        }
        unsafe {
            return bindings::bspoa_msa(self.poa) as u32;
        }
    }

    pub fn call_snvs(&mut self) {
        if !self.aligned {
            panic!("Align sequences before calling call_snvs, call `align` first");
        }
        unsafe {
            bindings::bspoa_call_snvs(self.poa);
        }
    }

    pub fn call_simple_cns(&mut self) {
        if !self.aligned {
            panic!("Align sequences before calling call_simple_cns, call `align` first");
        }
        unsafe {
            bindings::bspoa_simple_cns(self.poa);
        }
    }

    pub fn call_cns(&mut self) {
        if !self.aligned {
            panic!("Align sequences before calling call_cns, call `align` first");
        }
        unsafe {
            bindings::bspoa_cns(self.poa);
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

    pub fn get_cns(&self) -> Consensus<'_> {
        if !self.aligned {
            panic!("Align sequences before calling get_consensus, call `align` first");
        }
        let c = unsafe {
            let mut len = 0;
            let cns = bindings::bspoa_get_cns(self.poa, &mut len);
            std::slice::from_raw_parts(cns, len as usize)
        };
        Consensus { inner: c }
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

    pub fn get_alignment(&mut self, idx: usize) -> Option<AlignmentString<'_>> {
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
            std::slice::from_raw_parts(buffer, len)
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
        println!("Consensus: {}", cns.as_string());
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
}
