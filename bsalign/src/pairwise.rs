use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::AlignMode;
use crate::AlignScore;
use bsalign_sys::bindings;

#[derive(Debug, Clone, Copy)]
pub struct BsPairwiseParam {
    pub align_score: AlignScore,
    pub bandwidth: Option<usize>,
    pub ksize: usize,
    pub mode: AlignMode,
}

impl Default for BsPairwiseParam {
    fn default() -> Self {
        BsPairwiseParam {
            align_score: AlignScore::pairwise_default(),
            bandwidth: None,
            ksize: 13,
            mode: AlignMode::default(),
        }
    }
}

impl BsPairwiseParam {
    pub fn set_align_score(self, align_score: AlignScore) -> Self {
        let mut p = self;
        p.align_score = align_score;
        p
    }

    pub fn set_bandwidth(self, bandwidth: usize) -> Self {
        let mut p = self;
        p.bandwidth = Some(bandwidth);
        p
    }

    pub fn set_ksize(self, ksize: usize) -> Self {
        if ksize < 1 || ksize > 15 {
            panic!("ksize must be between 1 and 15");
        }
        let mut p = self;
        p.ksize = ksize;
        p
    }

    pub fn set_mode(self, mode: AlignMode) -> Self {
        let mut p = self;
        p.mode = mode;
        p
    }
}

#[derive(Debug)]
pub struct Cigars(*mut bindings::u4v);

impl Deref for Cigars {
    type Target = *mut bindings::u4v;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Cigars {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for Cigars {
    fn drop(&mut self) {
        unsafe {
            bindings::cigars_free(self.0);
        }
    }
}

impl Cigars {
    pub fn new() -> Self {
        let cigars = unsafe { bindings::cigars_init(32) };
        Cigars(cigars)
    }

    pub fn empty() -> Self {
        Cigars(std::ptr::null_mut())
    }

    pub fn clear(&mut self) {
        unsafe {
            bindings::cigars_clear(self.0);
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut bindings::u4v {
        self.0
    }
}

#[derive(Debug)]
pub struct BsPairwirseAligner {
    pub param: BsPairwiseParam,
    mempool: *mut bindings::b1v,
    pub cigars: Cigars,
    score_matrix: [i8; 16],
    pub qseq: *mut bindings::u1v,
    pub tseq: *mut bindings::u1v,
}

impl BsPairwirseAligner {
    pub fn new(param: BsPairwiseParam) -> Self {
        let mut p = Self {
            param,
            mempool: std::ptr::null_mut(),
            cigars: Cigars::empty(),
            score_matrix: [0; 16],
            qseq: std::ptr::null_mut(),
            tseq: std::ptr::null_mut(),
        };
        unsafe {
            bindings::bs_epi8_seqalign_set_score_matrix(
                p.score_matrix.as_mut_ptr(),
                param.align_score.M as i8,
                param.align_score.X as i8,
            );
            p.mempool = bindings::mempool_init(1024 * 1024, 0, 0);
            p.cigars = Cigars::new();
            p.qseq = bindings::bs_u1v_init(1024);
            p.tseq = bindings::bs_u1v_init(1024);
        }
        p
    }
}

impl Drop for BsPairwirseAligner {
    fn drop(&mut self) {
        unsafe {
            bindings::mempool_free(self.mempool);
            bindings::bs_u1v_free(self.qseq);
            bindings::bs_u1v_free(self.tseq);
        }
    }
}

#[derive(Debug)]
pub struct PsaAlignResult<'a> {
    result: bindings::seqalign_result_t,
    qseq: *mut bindings::u1v,
    tseq: *mut bindings::u1v,
    cigars: &'a Cigars,
}

impl<'a> Deref for PsaAlignResult<'a> {
    type Target = bindings::seqalign_result_t;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<'a> DerefMut for PsaAlignResult<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
}

impl<'a> Display for PsaAlignResult<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "QRY\t{}\t{}\tREF\t{}\t{}\tmat={}\tmis={}\tins={}\tdel={}\taln={}\n",
            self.qb, self.qe, self.tb, self.te, self.mat, self.mis, self.ins, self.del, self.aln
        )
    }
}

#[derive(Debug)]
pub struct AlignmentString {
    pub len: usize,
    tseq: *mut bindings::u1v,
    alignment: *mut bindings::u1v,
    qseq: *mut bindings::u1v,
}

impl Drop for AlignmentString {
    fn drop(&mut self) {
        unsafe {
            bindings::bs_u1v_free(self.tseq);
            bindings::bs_u1v_free(self.alignment);
            bindings::bs_u1v_free(self.qseq);
        }
    }
}

impl AlignmentString {
    pub fn with_capacity(capacity: usize) -> Self {
        unsafe {
            AlignmentString {
                len: capacity,
                tseq: bindings::bs_u1v_init(capacity + 1),
                alignment: bindings::bs_u1v_init(capacity + 1),
                qseq: bindings::bs_u1v_init(capacity + 1),
            }
        }
    }
}

impl AlignmentString {
    pub fn tseq(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.tseq.as_mut().unwrap().buffer,
                self.len as usize,
            ))
        }
    }

    pub fn qseq(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.qseq.as_mut().unwrap().buffer,
                self.len as usize,
            ))
        }
    }

    pub fn alignment(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.alignment.as_mut().unwrap().buffer,
                self.len as usize,
            ))
        }
    }
}

impl<'a> PsaAlignResult<'a> {
    pub fn to_string(&self) -> AlignmentString {
        let len = self.aln as usize;
        let alignment = AlignmentString::with_capacity(len);
        unsafe {
            let mut straln = [
                alignment.tseq.as_mut().unwrap().buffer,
                alignment.qseq.as_mut().unwrap().buffer,
                alignment.alignment.as_mut().unwrap().buffer,
            ];
            let mut ret = self.result.clone();
            bindings::bs_seqalign_cigar2alnstr(
                self.qseq.as_ref().unwrap().buffer,
                self.tseq.as_ref().unwrap().buffer,
                &mut ret,
                self.cigars.0,
                straln.as_mut_ptr() as *mut *mut i8,
                len as i32,
            );
        }
        alignment
    }

    pub fn to_string_with_buffer(&self, alignment: AlignmentString) -> AlignmentString {
        let len = self.aln as usize;
        let mut alignment = alignment;
        if alignment.len < len {
            alignment.len = len;
        }
        unsafe {
            bindings::bs_u1v_clear_and_encap(alignment.tseq, len);
            bindings::bs_u1v_clear_and_encap(alignment.alignment, len);
            bindings::bs_u1v_clear_and_encap(alignment.qseq, len);
            let mut straln = [
                alignment.tseq.as_mut().unwrap().buffer,
                alignment.qseq.as_mut().unwrap().buffer,
                alignment.alignment.as_mut().unwrap().buffer,
            ];
            let mut ret = self.result.clone();
            bindings::bs_seqalign_cigar2alnstr(
                self.qseq.as_ref().unwrap().buffer,
                self.tseq.as_ref().unwrap().buffer,
                &mut ret,
                self.cigars.0,
                straln.as_mut_ptr() as *mut *mut i8,
                len as i32,
            );
        }
        alignment
    }
}

impl BsPairwirseAligner {
    pub fn reset(&mut self) {
        unsafe {
            bindings::mempool_clear(self.mempool);
            self.cigars.clear();
            bindings::bs_u1v_clear(self.qseq);
            bindings::bs_u1v_clear(self.tseq);
        }
    }

    pub fn align_banded_striped_8bit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq, qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq, tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_bs_epi8_seqalign_pairwise(
                self.qseq.as_mut().unwrap().buffer,
                len1 as u32,
                self.tseq.as_mut().unwrap().buffer,
                len2 as u32,
                self.mempool,
                self.cigars.as_mut_ptr(),
                self.param.mode as i32,
                self.param.bandwidth.unwrap_or(0) as u32,
                self.score_matrix.as_ptr(),
                self.param.align_score.O as i8,
                self.param.align_score.E as i8,
                self.param.align_score.P as i8,
                self.param.align_score.Q as i8,
                0,
            )
        };

        PsaAlignResult {
            result: r,
            qseq: self.qseq.clone(),
            tseq: self.tseq.clone(),
            cigars: &self.cigars,
        }
    }

    pub unsafe fn align_striped_edit_2bit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq, qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq, tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_s_epi2_seqedit_pairwise(
                self.qseq.as_mut().unwrap().buffer,
                len1 as u32,
                tseq.as_ptr() as *mut u8,
                len2 as u32,
                self.mempool,
                self.cigars.as_mut_ptr(),
                self.param.mode as i32,
                0,
            )
        };
        PsaAlignResult {
            result: r,
            qseq: self.qseq,
            tseq: self.tseq,
            cigars: &self.cigars,
        }
    }

    pub fn align_striped_edit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq, qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq, tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_s_seqedit_pairwise(
                self.qseq.as_mut().unwrap().buffer,
                len1 as u32,
                self.tseq.as_mut().unwrap().buffer,
                len2 as u32,
                self.param.mode as i32,
                self.param.bandwidth.unwrap_or(0) as u32,
                self.mempool,
                self.cigars.as_mut_ptr(),
                0,
            )
        };
        PsaAlignResult {
            result: r,
            qseq: self.qseq,
            tseq: self.tseq,
            cigars: &self.cigars,
        }
    }

    pub fn align_kmer_striped_edit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq, qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq, tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_ks_seqedit_pairwise(
                self.param.ksize as u8,
                self.qseq.as_mut().unwrap().buffer,
                len1 as u32,
                self.tseq.as_mut().unwrap().buffer,
                len2 as u32,
                self.mempool,
                self.cigars.as_mut_ptr(),
                0,
            )
        };
        PsaAlignResult {
            result: r,
            qseq: self.qseq,
            tseq: self.tseq,
            cigars: &self.cigars,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_striped_edit_align() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default().set_ksize(4);
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_striped_edit(qseq, qseq);
        assert_eq!(result.aln, qseq.len() as i32);
    }

    #[test]
    fn test_striped_edit_align_2bit() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default().set_ksize(4);
        let mut aligner = BsPairwirseAligner::new(param);
        let result = unsafe { aligner.align_striped_edit_2bit(qseq, qseq) };
        eprintln!("result: {:?}", result);
        assert_eq!(result.aln, qseq.len() as i32);
    }

    #[test]
    fn test_kmer_striped_edit_align() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default().set_ksize(4);
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_kmer_striped_edit(qseq, qseq);
        assert_eq!(result.aln, qseq.len() as i32);
    }

    #[test]
    fn test_banded_striped_8bit_align() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default();
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len() as i32);
    }

    #[test]
    fn test_drop() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default();
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len() as i32);
        drop(aligner);
    }
}
