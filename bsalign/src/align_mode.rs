use bsalign_sys::bindings;

#[derive(Debug, Clone, Copy)]
pub enum AlignMode {
    Global = bindings::SEQALIGN_MODE_GLOBAL as isize,
    Overlap = bindings::SEQALIGN_MODE_OVERLAP as isize,
    Extend = bindings::SEQALIGN_MODE_EXTEND as isize,
    Kmer = bindings::SEQALIGN_MODE_KMER as isize,
}

impl Default for AlignMode {
    fn default() -> Self {
        AlignMode::Global
    }
}
