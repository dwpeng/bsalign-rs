use std::fmt::Display;

use crate::AlignMode;
use crate::AlignScore;
use crate::vector::{Mempool, U1V, U4V};
use bsalign_sys::bindings::{self, seqalign_result_t};

#[derive(Debug, Clone, Copy)]
pub struct BsPairwiseParam {
    pub align_score: AlignScore,
    pub bandwidth: Option<usize>,
    pub ksize: usize,
    pub mode: AlignMode,
    pub cigar: bool,
}

impl Default for BsPairwiseParam {
    fn default() -> Self {
        BsPairwiseParam {
            align_score: AlignScore::pairwise_default(),
            bandwidth: None,
            ksize: 13,
            mode: AlignMode::default(),
            cigar: true,
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

    pub fn set_cigar(self, cigar: bool) -> Self {
        let mut p = self;
        p.cigar = cigar;
        p
    }
}

#[derive(Debug)]
pub struct Cigars {
    inner: U4V,
}

impl Cigars {
    fn new() -> Self {
        Cigars { inner: U4V::new() }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    fn as_ptr(&self) -> *mut bindings::u4v {
        self.inner.as_ptr()
    }
}

impl Cigars {
    pub fn get(&self, index: usize) -> Option<(CigarType, usize)> {
        if self.inner.is_null() || index >= self.len() {
            return None;
        }
        unsafe {
            let cigar = *(*self.as_ptr()).buffer.add(index);
            let cigar_type = CigarType::from((cigar & 0x0f) as u8);
            let length = (cigar >> 4) as usize;
            Some((cigar_type, length))
        }
    }

    fn clear(&mut self) {
        self.inner.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CigarType {
    Match = 0,
    Insertion = 1,
    Deletion = 2,
    Skip = 3,
    SoftClip = 4,
    HardClip = 5,
    Padding = 6,
    Equal = 7,
    Mismatch = 8,
}

impl From<u8> for CigarType {
    fn from(value: u8) -> Self {
        match value {
            0 => CigarType::Match,
            1 => CigarType::Insertion,
            2 => CigarType::Deletion,
            3 => CigarType::Skip,
            4 => CigarType::SoftClip,
            5 => CigarType::HardClip,
            6 => CigarType::Padding,
            7 => CigarType::Equal,
            8 => CigarType::Mismatch,
            _ => panic!("Invalid cigar type"),
        }
    }
}

#[derive(Debug)]
pub struct BsPairwirseAligner {
    pub param: BsPairwiseParam,
    mempool: Mempool,
    cigars: Cigars,
    score_matrix: [i8; 16],
    qseq: U1V,
    tseq: U1V,
}

impl BsPairwirseAligner {
    pub fn new(param: BsPairwiseParam) -> Self {
        let cigars = if param.cigar {
            Cigars::new()
        } else {
            Cigars {
                inner: U4V::empty(),
            }
        };
        let mut p = Self {
            param,
            mempool: Mempool::with_capacity(1024 * 1024),
            cigars,
            score_matrix: [0; 16],
            qseq: U1V::with_capacity(1024),
            tseq: U1V::with_capacity(1024),
        };
        unsafe {
            bindings::bs_epi8_seqalign_set_score_matrix(
                p.score_matrix.as_mut_ptr(),
                param.align_score.M as i8,
                param.align_score.X as i8,
            );
        }
        p
    }
}

#[derive(Debug)]
pub struct PsaAlignResult<'a> {
    qseq: &'a U1V,
    tseq: &'a U1V,
    pub cigars: &'a Cigars,
    pub qb: usize,
    pub qe: usize,
    pub tb: usize,
    pub te: usize,
    pub aln: usize,
    pub mat: usize,
    pub mis: usize,
    pub ins: usize,
    pub del: usize,
    pub score: i32,
}

impl<'a> PsaAlignResult<'a> {
    fn init_with(
        qseq: &'a U1V,
        tseq: &'a U1V,
        cigars: &'a Cigars,
        result: seqalign_result_t,
    ) -> Self {
        PsaAlignResult {
            qseq,
            tseq,
            cigars,
            qb: result.qb as usize,
            qe: result.qe as usize,
            tb: result.tb as usize,
            te: result.te as usize,
            aln: result.aln as usize,
            mat: result.mat as usize,
            mis: result.mis as usize,
            ins: result.ins as usize,
            del: result.del as usize,
            score: result.score,
        }
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
    tseq: U1V,
    alignment: U1V,
    qseq: U1V,
}

impl AlignmentString {
    pub fn with_capacity(capacity: usize) -> Self {
        AlignmentString {
            len: capacity,
            tseq: U1V::with_capacity(capacity + 1),
            alignment: U1V::with_capacity(capacity + 1),
            qseq: U1V::with_capacity(capacity + 1),
        }
    }
}

impl AlignmentString {
    pub fn tseq(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.tseq.buffer(),
                self.len as usize,
            ))
        }
    }

    pub fn qseq(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.qseq.buffer(),
                self.len as usize,
            ))
        }
    }

    pub fn alignment(&self) -> &str {
        unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                self.alignment.buffer(),
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
                alignment.tseq.buffer(),
                alignment.qseq.buffer(),
                alignment.alignment.buffer(),
            ];
            let mut ret = seqalign_result_t::default();
            bindings::bs_seqalign_cigar2alnstr(
                self.qseq.buffer(),
                self.tseq.buffer(),
                &mut ret,
                self.cigars.as_ptr(),
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
            bindings::bs_u1v_clear_and_encap(alignment.tseq.as_ptr(), len);
            bindings::bs_u1v_clear_and_encap(alignment.alignment.as_ptr(), len);
            bindings::bs_u1v_clear_and_encap(alignment.qseq.as_ptr(), len);
            let mut straln = [
                alignment.tseq.buffer(),
                alignment.qseq.buffer(),
                alignment.alignment.buffer(),
            ];
            let mut ret = seqalign_result_t::default();
            bindings::bs_seqalign_cigar2alnstr(
                self.qseq.buffer(),
                self.tseq.buffer(),
                &mut ret,
                self.cigars.as_ptr(),
                straln.as_mut_ptr() as *mut *mut i8,
                len as i32,
            );
        }
        alignment
    }
}

impl BsPairwirseAligner {
    pub fn reset(&mut self) {
        self.mempool.clear();
        self.cigars.clear();
        self.qseq.clear();
        self.tseq.clear();
    }

    pub fn align_banded_striped_8bit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq.as_ptr(), qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq.as_ptr(), tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_bs_epi8_seqalign_pairwise(
                self.qseq.buffer(),
                len1 as u32,
                self.tseq.buffer(),
                len2 as u32,
                self.mempool.as_ptr(),
                self.cigars.as_ptr(),
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

        PsaAlignResult::init_with(&self.qseq, &self.tseq, &self.cigars, r)
    }

    // // unsable for 2bit alignment, so commented out
    // pub unsafe fn align_striped_edit_2bit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    // where
    //     T: AsRef<[u8]> + ?Sized,
    // {
    //     let qseq = qseq.as_ref();
    //     let tseq = tseq.as_ref();
    //     unsafe {
    //         bindings::create_bits_from_seq(self.qseq, qseq.as_ptr(), qseq.len() as u32);
    //         bindings::create_bits_from_seq(self.tseq, tseq.as_ptr(), tseq.len() as u32);
    //     }
    //     let len1 = qseq.len();
    //     let len2 = tseq.len();
    //     let r = unsafe {
    //         bindings::bs_s_epi2_seqedit_pairwise(
    //             self.qseq.as_mut().unwrap().buffer,
    //             len1 as u32,
    //             tseq.as_ptr() as *mut u8,
    //             len2 as u32,
    //             self.mempool,
    //             self.cigars.as_mut_ptr(),
    //             self.param.mode as i32,
    //             0,
    //         )
    //     };
    //     PsaAlignResult {
    //         result: r,
    //         qseq: self.qseq,
    //         tseq: self.tseq,
    //         cigars: &self.cigars,
    //     }
    // }

    pub fn align_striped_edit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq.as_ptr(), qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq.as_ptr(), tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_s_seqedit_pairwise(
                self.qseq.buffer(),
                len1 as u32,
                self.tseq.buffer(),
                len2 as u32,
                self.param.mode as i32,
                self.param.bandwidth.unwrap_or(0) as u32,
                self.mempool.as_ptr(),
                self.cigars.as_ptr(),
                0,
            )
        };
        PsaAlignResult::init_with(&self.qseq, &self.tseq, &self.cigars, r)
    }

    pub fn align_kmer_striped_edit<T>(&mut self, qseq: &T, tseq: &T) -> PsaAlignResult
    where
        T: AsRef<[u8]> + ?Sized,
    {
        let qseq = qseq.as_ref();
        let tseq = tseq.as_ref();
        unsafe {
            bindings::create_bits_from_seq(self.qseq.as_ptr(), qseq.as_ptr(), qseq.len() as u32);
            bindings::create_bits_from_seq(self.tseq.as_ptr(), tseq.as_ptr(), tseq.len() as u32);
        }
        let len1 = qseq.len();
        let len2 = tseq.len();
        let r = unsafe {
            bindings::bs_ks_seqedit_pairwise(
                self.param.ksize as u8,
                self.qseq.buffer(),
                len1 as u32,
                self.tseq.buffer(),
                len2 as u32,
                self.mempool.as_ptr(),
                self.cigars.as_ptr(),
                0,
            )
        };
        PsaAlignResult::init_with(&self.qseq, &self.tseq, &self.cigars, r)
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
        assert_eq!(result.aln, qseq.len());
    }

    // #[test]
    // fn test_striped_edit_align_2bit() {
    //     let qseq = include_str!("../../test-data/seq.seq");
    //     let param = BsPairwiseParam::default().set_ksize(4);
    //     let mut aligner = BsPairwirseAligner::new(param);
    //     let result = unsafe { aligner.align_striped_edit_2bit(qseq, qseq) };
    //     eprintln!("result: {:?}", result);
    //     assert_eq!(result.aln, qseq.len() as i32);
    // }

    #[test]
    fn test_kmer_striped_edit_align() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default().set_ksize(4);
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_kmer_striped_edit(qseq, qseq);
        assert_eq!(result.aln, qseq.len());
    }

    #[test]
    fn test_banded_striped_8bit_align() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default();
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len());
    }

    #[test]
    fn test_drop() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default();
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len());
        drop(aligner);
    }

    #[test]
    fn test_cigars() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default();
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len());
        let cigars = result.cigars;
        assert!(cigars.as_ptr() != std::ptr::null_mut());
        let cigar = cigars.get(0);
        assert!(cigar.is_some());
        let (cigar_type, length) = cigar.unwrap();
        assert_eq!(cigar_type, CigarType::Match);
        assert_eq!(length, qseq.len());
        assert_eq!(cigars.len(), 1);
    }

    #[test]
    fn test_cigars_empty() {
        let qseq = include_str!("../../test-data/seq.seq");
        let param = BsPairwiseParam::default().set_cigar(false);
        let mut aligner = BsPairwirseAligner::new(param);
        let result = aligner.align_banded_striped_8bit(qseq, qseq);
        assert_eq!(result.aln, qseq.len());
        let cigars = result.cigars;
        assert_eq!(cigars.as_ptr(), std::ptr::null_mut());
        assert_eq!(cigars.len(), 0);
    }
}
