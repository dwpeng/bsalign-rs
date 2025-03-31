#include "wrapper.h"

BSPOA *bspoa_init(BSPOAPar p) { return init_bspoa(p); }
void bspoa_free(BSPOA *poa) { free_bspoa(poa); }
void bspoa_clear(BSPOA *poa) { clear_bspoa(poa); }
void bspoa_add_sequence(BSPOA *poa, const uint8_t *seq, u4i len) {
  push_bspoa(poa, (char *)seq, len);
}
double bspoa_cns(BSPOA *poa) { return cns_bspoa(poa); }
u4i bspoa_msa(BSPOA *poa) { return msa_bspoa(poa); }
void bspoa_simple_cns(BSPOA *poa) { simple_cns_bspoa(poa); }
void bspoa_tidy_msa(BSPOA *poa) { tidy_msa_bspoa(poa); }
void bspoa_call_snvs(BSPOA *poa) { call_snvs_bspoa(poa); }
void bspoa_print_snvs(BSPOA *poa, char *label, const char *filename) {
  FILE *out = fopen(filename, "wb");
  print_snvs_bspoa(poa, label, out);
  fclose(out);
}
void bspoa_print_msa(BSPOA *poa, char *label, u4i mbeg, u4i mend, u4i linewidth,
                     int colorful, FILE *out) {
  print_msa_bspoa(poa, label, mbeg, mend, linewidth, colorful, out);
}
void bspoa_begin(BSPOA *poa) { beg_bspoa(poa); }
void bspoa_end(BSPOA *poa) { end_bspoa(poa); }
void bspoa_dump_binary_msa(BSPOA *poa, char *metadata, u4i metalen,
                           const char *filename) {
  FILE *out = fopen(filename, "wb");
  dump_binary_msa_bspoa(poa, metadata, metalen, out);
  fclose(out);
}
int bspoa_load_binary_msa(BSPOA *poa, const char *filename, String *metadata) {
  FILE *in = fopen(filename, "rb");
  int ret = load_binary_msa_bspoa(poa, in, metadata);
  fclose(in);
  return ret;
}

u1i *bspoa_get_cns(BSPOA *poa, u4i *retlen) {
  *retlen = poa->cns->size;
  return poa->cns->buffer;
}

u1i *bspoa_get_qlt(BSPOA *poa, u4i *retlen) {
  *retlen = poa->qlt->size;
  return poa->qlt->buffer;
}

u1i *bspoa_get_alt(BSPOA *poa, u4i *retlen) {
  *retlen = poa->alt->size;
  return poa->alt->buffer;
}

void string_free(String *s) { free_string(s); }
String *string_init(size_t size) { return init_string(size); }

void bs_epi8_seqalign_set_score_matrix(int8_t m[16], int8_t match,
                                       int8_t mismatch) {
  return banded_striped_epi8_seqalign_set_score_matrix(m, match, mismatch);
}

seqalign_result_t
bs_bs_epi8_seqalign_pairwise(uint8_t *qseq, uint32_t qlen, uint8_t *tseq,
                             uint32_t tlen, b1v *mempool, u4v *cigars, int mode,
                             uint32_t bandwidth, const int8_t m[16],
                             int8_t gap_open, int8_t gap_ext, int8_t gap_open2,
                             int8_t gap_ext2, int verbose) {
  return banded_striped_epi8_seqalign_pairwise(
      (u1i *)qseq, qlen, (u1i *)tseq, tlen, mempool, cigars, mode, bandwidth,
      (b1i *)m, gap_open, gap_ext, gap_open2, gap_ext2, verbose);
}

seqalign_result_t bs_s_seqedit_pairwise(uint8_t *qseq, uint32_t qlen,
                                        uint8_t *tseq, uint32_t tlen, int mode,
                                        uint32_t bandwidth, b1v *mempool,
                                        u4v *cigars, int verbose) {
  return striped_seqedit_pairwise((u1i *)qseq, qlen, (u1i *)tseq, tlen, mode,
                                  bandwidth, mempool, cigars, verbose);
}

seqalign_result_t bs_ks_seqedit_pairwise(uint8_t ksize, uint8_t *qseq,
                                         uint32_t qlen, uint8_t *tseq,
                                         uint32_t tlen, b1v *mempool,
                                         u4v *cigars, int verbose) {
  seqalign_result_t rs = kmer_striped_seqedit_pairwise(
      ksize, (u1i *)qseq, qlen, (u1i *)tseq, tlen, mempool, cigars, verbose);
  return rs;
}
// seqalign_result_t bs_s_epi2_seqedit_pairwise(uint8_t *qseq, uint32_t qlen,
//                                              uint8_t *tseq, uint32_t tlen,
//                                              b1v *mempool, u4v *cigars,
//                                              int mode, int verbose) {
//   return striped_epi2_seqedit_pairwise((u1i *)qseq, qlen, (u1i *)tseq, tlen,
//                                        mempool, cigars, mode, verbose);
// }

void bs_seqalign_cigar2alnstr(u1i *qseq, u1i *tseq, seqalign_result_t *rs,
                              u4v *cigars, char *alnstr[3], int alnlen) {
  seqalign_cigar2alnstr(qseq, tseq, rs, cigars, alnstr, alnlen);
}

b1v *mempool_init(size_t size, int value, int nhead) {
  return adv_init_b1v(size, value, WORDSIZE, nhead);
}

void mempool_free(b1v *mempool) { free_b1v(mempool); }
void mempool_clear(b1v *mempool) { clear_b1v(mempool); }

u4v *cigars_init(size_t size) { return init_u4v(size); }
void cigars_free(u4v *cigars) { free_u4v(cigars); }
void cigars_clear(u4v *cigars) { clear_u4v(cigars); }

u1v *bs_u1v_init(size_t size) { return init_u1v(size); }
void bs_u1v_clear(u1v *v) { clear_u1v(v); }
void bs_u1v_free(u1v *v) { free_u1v(v); }
void bs_u1v_clear_and_encap(u1v *v, size_t size) {
  clear_and_encap_u1v(v, size);
}
void create_bits_from_seq(u1v *bits, const uint8_t *seq, u4i len) {
  clear_and_encap_u1v(bits, len);
  u4i i = 0;
  for (; i < len; ++i) {
    bits->buffer[i] = base_bit_table[seq[i]] & 0x3;
  }
}

const char *bsalign_version() {
#ifdef VERSION
  return VERSION;
#else
  return "unknown";
#endif
}
