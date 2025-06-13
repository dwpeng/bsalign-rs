#ifndef __WRAPPER_H
#define __WRAPPER_H
#include "bsalign/bsalign.h"
#include "bsalign/bspoa.h"

typedef struct {
  char *path;
  FILE *fp;
  int closeable;
} c_file_t;

c_file_t *c_file_open(const char *path);
void c_file_close(c_file_t *file);

void bs_epi8_seqalign_set_score_matrix(int8_t m[16], int8_t match,
                                       int8_t mismatch);
seqalign_result_t
bs_bs_epi8_seqalign_pairwise(uint8_t *qseq, uint32_t qlen, uint8_t *tseq,
                             uint32_t tlen, b1v *mempool, u4v *cigars, int mode,
                             uint32_t bandwidth, const int8_t m[16],
                             int8_t gap_open, int8_t gap_ext, int8_t gap_open2,
                             int8_t gap_ext2, int verbose);

seqalign_result_t bs_s_seqedit_pairwise(uint8_t *qseq, uint32_t qlen,
                                        uint8_t *tseq, uint32_t tlen, int mode,
                                        uint32_t bandwidth, b1v *mempool,
                                        u4v *cigars, int verbose);
seqalign_result_t bs_ks_seqedit_pairwise(uint8_t ksize, uint8_t *qseq,
                                         uint32_t qlen, uint8_t *tseq,
                                         uint32_t tlen, b1v *mempool,
                                         u4v *cigars, int verbose);
// seqalign_result_t bs_s_epi2_seqedit_pairwise(uint8_t *qseq, uint32_t qlen,
//                                              uint8_t *tseq, uint32_t tlen,
//                                              b1v *mempool, u4v *cigars,
//                                              int mode, int verbose);

void bs_seqalign_cigar2alnstr(u1i *qseq, u1i *tseq, seqalign_result_t *rs,
                              u4v *cigars, char *alnstr[3], int alnlen);

b1v *mempool_init(size_t size, int value, int nhead);
void mempool_free(b1v *mempool);
void mempool_clear(b1v *mempool);

u4v *cigars_init(size_t size);
void cigars_free(u4v *cigars);
void cigars_clear(u4v *cigars);

u1v *bs_u1v_init(size_t size);
void bs_u1v_clear(u1v *v);
void bs_u1v_free(u1v *v);
void bs_u1v_clear_and_encap(u1v *v, size_t size);
void create_bits_from_seq(u1v *bits, const uint8_t *seq, u4i len);

BSPOA *bspoa_init(BSPOAPar poa_params);
void bspoa_free(BSPOA *poa);
void bspoa_clear(BSPOA *poa);
void bspoa_add_sequence(BSPOA *poa, const uint8_t *seq, u4i len);
double bspoa_cns(BSPOA *poa);
void bspoa_simple_cns(BSPOA *poa);
u4i bspoa_msa(BSPOA *poa);
void bspoa_tidy_msa(BSPOA *poa);
void bspoa_call_snvs(BSPOA *poa);
void bspoa_print_snvs(BSPOA *poa, char *label, c_file_t *fp);
void bspoa_print_msa(BSPOA *poa, char *label, u4i mbeg, u4i mend, u4i linewidth,
                     int colorful, c_file_t *fp);
void bspoa_begin(BSPOA *poa);
void bspoa_end(BSPOA *poa);
void bspoa_dump_binary_msa(BSPOA *poa, char *metadata, u4i metalen,
                           const char *filename);
int bspoa_load_binary_msa(BSPOA *poa, const char *filename, String *metadata);
u1i *bspoa_get_cns(BSPOA *poa, u4i *retlen);
u1i *bspoa_get_qlt(BSPOA *poa, u4i *retlen);
u1i *bspoa_get_alt(BSPOA *poa, u4i *retlen);

void string_free(String *s);
String *string_init(size_t size);

const char *bsalign_version();

#endif
