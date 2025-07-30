#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte =
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
pub const _FEATURES_H: u8 = 1;
pub const _DEFAULT_SOURCE: u8 = 1;
pub const __GLIBC_USE_ISOC2X: u8 = 0;
pub const __USE_POSIX_IMPLICITLY: u8 = 1;
pub const _POSIX_SOURCE: u8 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u8 = 1;
pub const __USE_POSIX2: u8 = 1;
pub const __USE_POSIX199309: u8 = 1;
pub const __USE_POSIX199506: u8 = 1;
pub const __USE_XOPEN2K: u8 = 1;
pub const __USE_ISOC95: u8 = 1;
pub const __USE_ISOC99: u8 = 1;
pub const __USE_XOPEN2K8: u8 = 1;
pub const _ATFILE_SOURCE: u8 = 1;
pub const __WORDSIZE: u8 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u8 = 1;
pub const __SYSCALL_WORDSIZE: u8 = 64;
pub const __TIMESIZE: u8 = 64;
pub const __USE_MISC: u8 = 1;
pub const __USE_ATFILE: u8 = 1;
pub const __USE_FORTIFY_LEVEL: u8 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u8 = 1;
pub const __GLIBC_USE_DEPRECATED_SCANF: u8 = 0;
pub const _STDC_PREDEF_H: u8 = 1;
pub const __STDC_IEC_559__: u8 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u8 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u8 = 6;
pub const __GLIBC__: u8 = 2;
pub const __GLIBC_MINOR__: u8 = 36;
pub const _SYS_CDEFS_H: u8 = 1;
pub const __glibc_c99_flexarr_available: u8 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u8 = 0;
pub const __HAVE_GENERIC_SELECTION: u8 = 1;
pub const __GLIBC_USE_LIB_EXT2: u8 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u8 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u8 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u8 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u8 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u8 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u8 = 0;
pub const _STDLIB_H: u8 = 1;
pub const WNOHANG: u8 = 1;
pub const WUNTRACED: u8 = 2;
pub const WSTOPPED: u8 = 2;
pub const WEXITED: u8 = 4;
pub const WCONTINUED: u8 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u16 = 65535;
pub const __WCOREFLAG: u8 = 128;
pub const __HAVE_FLOAT128: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u8 = 0;
pub const __HAVE_FLOAT64X: u8 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u8 = 1;
pub const __HAVE_FLOAT16: u8 = 0;
pub const __HAVE_FLOAT32: u8 = 1;
pub const __HAVE_FLOAT64: u8 = 1;
pub const __HAVE_FLOAT32X: u8 = 1;
pub const __HAVE_FLOAT128X: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u8 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u8 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u8 = 0;
pub const __ldiv_t_defined: u8 = 1;
pub const __lldiv_t_defined: u8 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u8 = 1;
pub const EXIT_SUCCESS: u8 = 0;
pub const _SYS_TYPES_H: u8 = 1;
pub const _BITS_TYPES_H: u8 = 1;
pub const _BITS_TYPESIZES_H: u8 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u8 = 1;
pub const __INO_T_MATCHES_INO64_T: u8 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u8 = 1;
pub const __STATFS_MATCHES_STATFS64: u8 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u8 = 1;
pub const __FD_SETSIZE: u16 = 1024;
pub const _BITS_TIME64_H: u8 = 1;
pub const __clock_t_defined: u8 = 1;
pub const __clockid_t_defined: u8 = 1;
pub const __time_t_defined: u8 = 1;
pub const __timer_t_defined: u8 = 1;
pub const _BITS_STDINT_INTN_H: u8 = 1;
pub const __BIT_TYPES_DEFINED__: u8 = 1;
pub const _ENDIAN_H: u8 = 1;
pub const _BITS_ENDIAN_H: u8 = 1;
pub const __LITTLE_ENDIAN: u16 = 1234;
pub const __BIG_ENDIAN: u16 = 4321;
pub const __PDP_ENDIAN: u16 = 3412;
pub const _BITS_ENDIANNESS_H: u8 = 1;
pub const __BYTE_ORDER: u16 = 1234;
pub const __FLOAT_WORD_ORDER: u16 = 1234;
pub const LITTLE_ENDIAN: u16 = 1234;
pub const BIG_ENDIAN: u16 = 4321;
pub const PDP_ENDIAN: u16 = 3412;
pub const BYTE_ORDER: u16 = 1234;
pub const _BITS_BYTESWAP_H: u8 = 1;
pub const _BITS_UINTN_IDENTITY_H: u8 = 1;
pub const _SYS_SELECT_H: u8 = 1;
pub const __sigset_t_defined: u8 = 1;
pub const __timeval_defined: u8 = 1;
pub const _STRUCT_TIMESPEC: u8 = 1;
pub const FD_SETSIZE: u16 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u8 = 1;
pub const _THREAD_SHARED_TYPES_H: u8 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u8 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u8 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u8 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u8 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u8 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u8 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u8 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u8 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u8 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u8 = 4;
pub const _THREAD_MUTEX_INTERNAL_H: u8 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u8 = 1;
pub const __have_pthread_attr_t: u8 = 1;
pub const _ALLOCA_H: u8 = 1;
pub const _STRING_H: u8 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u8 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u8 = 1;
pub const _STRINGS_H: u8 = 1;
pub const _STDINT_H: u8 = 1;
pub const _BITS_WCHAR_H: u8 = 1;
pub const _BITS_STDINT_UINTN_H: u8 = 1;
pub const INT8_MIN: i8 = -128;
pub const INT16_MIN: i16 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u8 = 127;
pub const INT16_MAX: u16 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u8 = 255;
pub const UINT16_MAX: u16 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i8 = -128;
pub const INT_LEAST16_MIN: i16 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u8 = 127;
pub const INT_LEAST16_MAX: u16 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u8 = 255;
pub const UINT_LEAST16_MAX: u16 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i8 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u8 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u8 = 255;
pub const UINT_FAST16_MAX: i8 = -1;
pub const UINT_FAST32_MAX: i8 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i8 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i8 = -1;
pub const WINT_MIN: u8 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _STDIO_H: u8 = 1;
pub const __GNUC_VA_LIST: u8 = 1;
pub const _____fpos_t_defined: u8 = 1;
pub const ____mbstate_t_defined: u8 = 1;
pub const _____fpos64_t_defined: u8 = 1;
pub const ____FILE_defined: u8 = 1;
pub const __FILE_defined: u8 = 1;
pub const __struct_FILE_defined: u8 = 1;
pub const _IO_EOF_SEEN: u8 = 16;
pub const _IO_ERR_SEEN: u8 = 32;
pub const _IO_USER_LOCK: u16 = 32768;
pub const _IOFBF: u8 = 0;
pub const _IOLBF: u8 = 1;
pub const _IONBF: u8 = 2;
pub const BUFSIZ: u16 = 8192;
pub const EOF: i8 = -1;
pub const SEEK_SET: u8 = 0;
pub const SEEK_CUR: u8 = 1;
pub const SEEK_END: u8 = 2;
pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u8 = 1;
pub const L_tmpnam: u8 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u16 = 4096;
pub const L_ctermid: u8 = 9;
pub const FOPEN_MAX: u8 = 16;
pub const _SYS_STAT_H: u8 = 1;
pub const _BITS_STAT_H: u8 = 1;
pub const _BITS_STRUCT_STAT_H: u8 = 1;
pub const __S_IFMT: u16 = 61440;
pub const __S_IFDIR: u16 = 16384;
pub const __S_IFCHR: u16 = 8192;
pub const __S_IFBLK: u16 = 24576;
pub const __S_IFREG: u16 = 32768;
pub const __S_IFIFO: u16 = 4096;
pub const __S_IFLNK: u16 = 40960;
pub const __S_IFSOCK: u16 = 49152;
pub const __S_ISUID: u16 = 2048;
pub const __S_ISGID: u16 = 1024;
pub const __S_ISVTX: u16 = 512;
pub const __S_IREAD: u16 = 256;
pub const __S_IWRITE: u8 = 128;
pub const __S_IEXEC: u8 = 64;
pub const UTIME_NOW: u32 = 1073741823;
pub const UTIME_OMIT: u32 = 1073741822;
pub const S_IFMT: u16 = 61440;
pub const S_IFDIR: u16 = 16384;
pub const S_IFCHR: u16 = 8192;
pub const S_IFBLK: u16 = 24576;
pub const S_IFREG: u16 = 32768;
pub const S_IFIFO: u16 = 4096;
pub const S_IFLNK: u16 = 40960;
pub const S_IFSOCK: u16 = 49152;
pub const S_ISUID: u16 = 2048;
pub const S_ISGID: u16 = 1024;
pub const S_ISVTX: u16 = 512;
pub const S_IRUSR: u16 = 256;
pub const S_IWUSR: u8 = 128;
pub const S_IXUSR: u8 = 64;
pub const S_IRWXU: u16 = 448;
pub const S_IREAD: u16 = 256;
pub const S_IWRITE: u8 = 128;
pub const S_IEXEC: u8 = 64;
pub const S_IRGRP: u8 = 32;
pub const S_IWGRP: u8 = 16;
pub const S_IXGRP: u8 = 8;
pub const S_IRWXG: u8 = 56;
pub const S_IROTH: u8 = 4;
pub const S_IWOTH: u8 = 2;
pub const S_IXOTH: u8 = 1;
pub const S_IRWXO: u8 = 7;
pub const ACCESSPERMS: u16 = 511;
pub const ALLPERMS: u16 = 4095;
pub const DEFFILEMODE: u16 = 438;
pub const S_BLKSIZE: u16 = 512;
pub const _SYS_MMAN_H: u8 = 1;
pub const MAP_32BIT: u8 = 64;
pub const MAP_GROWSDOWN: u16 = 256;
pub const MAP_DENYWRITE: u16 = 2048;
pub const MAP_EXECUTABLE: u16 = 4096;
pub const MAP_LOCKED: u16 = 8192;
pub const MAP_NORESERVE: u16 = 16384;
pub const MAP_POPULATE: u16 = 32768;
pub const MAP_NONBLOCK: u32 = 65536;
pub const MAP_STACK: u32 = 131072;
pub const MAP_HUGETLB: u32 = 262144;
pub const MAP_SYNC: u32 = 524288;
pub const MAP_FIXED_NOREPLACE: u32 = 1048576;
pub const PROT_READ: u8 = 1;
pub const PROT_WRITE: u8 = 2;
pub const PROT_EXEC: u8 = 4;
pub const PROT_NONE: u8 = 0;
pub const PROT_GROWSDOWN: u32 = 16777216;
pub const PROT_GROWSUP: u32 = 33554432;
pub const MAP_SHARED: u8 = 1;
pub const MAP_PRIVATE: u8 = 2;
pub const MAP_SHARED_VALIDATE: u8 = 3;
pub const MAP_TYPE: u8 = 15;
pub const MAP_FIXED: u8 = 16;
pub const MAP_FILE: u8 = 0;
pub const MAP_ANONYMOUS: u8 = 32;
pub const MAP_ANON: u8 = 32;
pub const MAP_HUGE_SHIFT: u8 = 26;
pub const MAP_HUGE_MASK: u8 = 63;
pub const MS_ASYNC: u8 = 1;
pub const MS_SYNC: u8 = 4;
pub const MS_INVALIDATE: u8 = 2;
pub const MADV_NORMAL: u8 = 0;
pub const MADV_RANDOM: u8 = 1;
pub const MADV_SEQUENTIAL: u8 = 2;
pub const MADV_WILLNEED: u8 = 3;
pub const MADV_DONTNEED: u8 = 4;
pub const MADV_FREE: u8 = 8;
pub const MADV_REMOVE: u8 = 9;
pub const MADV_DONTFORK: u8 = 10;
pub const MADV_DOFORK: u8 = 11;
pub const MADV_MERGEABLE: u8 = 12;
pub const MADV_UNMERGEABLE: u8 = 13;
pub const MADV_HUGEPAGE: u8 = 14;
pub const MADV_NOHUGEPAGE: u8 = 15;
pub const MADV_DONTDUMP: u8 = 16;
pub const MADV_DODUMP: u8 = 17;
pub const MADV_WIPEONFORK: u8 = 18;
pub const MADV_KEEPONFORK: u8 = 19;
pub const MADV_COLD: u8 = 20;
pub const MADV_PAGEOUT: u8 = 21;
pub const MADV_POPULATE_READ: u8 = 22;
pub const MADV_POPULATE_WRITE: u8 = 23;
pub const MADV_DONTNEED_LOCKED: u8 = 24;
pub const MADV_HWPOISON: u8 = 100;
pub const POSIX_MADV_NORMAL: u8 = 0;
pub const POSIX_MADV_RANDOM: u8 = 1;
pub const POSIX_MADV_SEQUENTIAL: u8 = 2;
pub const POSIX_MADV_WILLNEED: u8 = 3;
pub const POSIX_MADV_DONTNEED: u8 = 4;
pub const MCL_CURRENT: u8 = 1;
pub const MCL_FUTURE: u8 = 2;
pub const MCL_ONFAULT: u8 = 4;
pub const _SYS_TIME_H: u8 = 1;
pub const _BITS_SIGNUM_GENERIC_H: u8 = 1;
pub const SIGINT: u8 = 2;
pub const SIGILL: u8 = 4;
pub const SIGABRT: u8 = 6;
pub const SIGFPE: u8 = 8;
pub const SIGSEGV: u8 = 11;
pub const SIGTERM: u8 = 15;
pub const SIGHUP: u8 = 1;
pub const SIGQUIT: u8 = 3;
pub const SIGTRAP: u8 = 5;
pub const SIGKILL: u8 = 9;
pub const SIGPIPE: u8 = 13;
pub const SIGALRM: u8 = 14;
pub const SIGIOT: u8 = 6;
pub const _BITS_SIGNUM_ARCH_H: u8 = 1;
pub const SIGSTKFLT: u8 = 16;
pub const SIGPWR: u8 = 30;
pub const SIGBUS: u8 = 7;
pub const SIGSYS: u8 = 31;
pub const SIGURG: u8 = 23;
pub const SIGSTOP: u8 = 19;
pub const SIGTSTP: u8 = 20;
pub const SIGCONT: u8 = 18;
pub const SIGCHLD: u8 = 17;
pub const SIGTTIN: u8 = 21;
pub const SIGTTOU: u8 = 22;
pub const SIGPOLL: u8 = 29;
pub const SIGXFSZ: u8 = 25;
pub const SIGXCPU: u8 = 24;
pub const SIGVTALRM: u8 = 26;
pub const SIGPROF: u8 = 27;
pub const SIGUSR1: u8 = 10;
pub const SIGUSR2: u8 = 12;
pub const SIGWINCH: u8 = 28;
pub const SIGIO: u8 = 29;
pub const SIGCLD: u8 = 17;
pub const __SIGRTMIN: u8 = 32;
pub const __SIGRTMAX: u8 = 64;
pub const _NSIG: u8 = 65;
pub const __sig_atomic_t_defined: u8 = 1;
pub const __siginfo_t_defined: u8 = 1;
pub const __SI_MAX_SIZE: u8 = 128;
pub const _BITS_SIGINFO_ARCH_H: u8 = 1;
pub const __SI_ERRNO_THEN_CODE: u8 = 1;
pub const __SI_HAVE_SIGSYS: u8 = 1;
pub const _BITS_SIGINFO_CONSTS_H: u8 = 1;
pub const __SI_ASYNCIO_AFTER_SIGIO: u8 = 1;
pub const __sigevent_t_defined: u8 = 1;
pub const __SIGEV_MAX_SIZE: u8 = 64;
pub const _BITS_SIGEVENT_CONSTS_H: u8 = 1;
pub const NSIG: u8 = 65;
pub const _BITS_SIGACTION_H: u8 = 1;
pub const SA_NOCLDSTOP: u8 = 1;
pub const SA_NOCLDWAIT: u8 = 2;
pub const SA_SIGINFO: u8 = 4;
pub const SA_ONSTACK: u32 = 134217728;
pub const SA_RESTART: u32 = 268435456;
pub const SA_NODEFER: u32 = 1073741824;
pub const SA_RESETHAND: u32 = 2147483648;
pub const SA_INTERRUPT: u32 = 536870912;
pub const SA_NOMASK: u32 = 1073741824;
pub const SA_ONESHOT: u32 = 2147483648;
pub const SA_STACK: u32 = 134217728;
pub const SIG_BLOCK: u8 = 0;
pub const SIG_UNBLOCK: u8 = 1;
pub const SIG_SETMASK: u8 = 2;
pub const _BITS_SIGCONTEXT_H: u8 = 1;
pub const FP_XSTATE_MAGIC1: u32 = 1179670611;
pub const FP_XSTATE_MAGIC2: u32 = 1179670597;
pub const __stack_t_defined: u8 = 1;
pub const _SYS_UCONTEXT_H: u8 = 1;
pub const __NGREG: u8 = 23;
pub const NGREG: u8 = 23;
pub const _BITS_SIGSTACK_H: u8 = 1;
pub const MINSIGSTKSZ: u16 = 2048;
pub const SIGSTKSZ: u16 = 8192;
pub const _BITS_SS_FLAGS_H: u8 = 1;
pub const __sigstack_defined: u8 = 1;
pub const _BITS_SIGTHREAD_H: u8 = 1;
pub const _UNISTD_H: u8 = 1;
pub const _POSIX_VERSION: u32 = 200809;
pub const __POSIX2_THIS_VERSION: u32 = 200809;
pub const _POSIX2_VERSION: u32 = 200809;
pub const _POSIX2_C_VERSION: u32 = 200809;
pub const _POSIX2_C_BIND: u32 = 200809;
pub const _POSIX2_C_DEV: u32 = 200809;
pub const _POSIX2_SW_DEV: u32 = 200809;
pub const _POSIX2_LOCALEDEF: u32 = 200809;
pub const _XOPEN_VERSION: u16 = 700;
pub const _XOPEN_XCU_VERSION: u8 = 4;
pub const _XOPEN_XPG2: u8 = 1;
pub const _XOPEN_XPG3: u8 = 1;
pub const _XOPEN_XPG4: u8 = 1;
pub const _XOPEN_UNIX: u8 = 1;
pub const _XOPEN_ENH_I18N: u8 = 1;
pub const _XOPEN_LEGACY: u8 = 1;
pub const _BITS_POSIX_OPT_H: u8 = 1;
pub const _POSIX_JOB_CONTROL: u8 = 1;
pub const _POSIX_SAVED_IDS: u8 = 1;
pub const _POSIX_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_SYNCHRONIZED_IO: u32 = 200809;
pub const _POSIX_FSYNC: u32 = 200809;
pub const _POSIX_MAPPED_FILES: u32 = 200809;
pub const _POSIX_MEMLOCK: u32 = 200809;
pub const _POSIX_MEMLOCK_RANGE: u32 = 200809;
pub const _POSIX_MEMORY_PROTECTION: u32 = 200809;
pub const _POSIX_CHOWN_RESTRICTED: u8 = 0;
pub const _POSIX_VDISABLE: u8 = 0u8;
pub const _POSIX_NO_TRUNC: u8 = 1;
pub const _XOPEN_REALTIME: u8 = 1;
pub const _XOPEN_REALTIME_THREADS: u8 = 1;
pub const _XOPEN_SHM: u8 = 1;
pub const _POSIX_THREADS: u32 = 200809;
pub const _POSIX_REENTRANT_FUNCTIONS: u8 = 1;
pub const _POSIX_THREAD_SAFE_FUNCTIONS: u32 = 200809;
pub const _POSIX_THREAD_PRIORITY_SCHEDULING: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKSIZE: u32 = 200809;
pub const _POSIX_THREAD_ATTR_STACKADDR: u32 = 200809;
pub const _POSIX_THREAD_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_PRIO_PROTECT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_INHERIT: u32 = 200809;
pub const _POSIX_THREAD_ROBUST_PRIO_PROTECT: i8 = -1;
pub const _POSIX_SEMAPHORES: u32 = 200809;
pub const _POSIX_REALTIME_SIGNALS: u32 = 200809;
pub const _POSIX_ASYNCHRONOUS_IO: u32 = 200809;
pub const _POSIX_ASYNC_IO: u8 = 1;
pub const _LFS_ASYNCHRONOUS_IO: u8 = 1;
pub const _POSIX_PRIORITIZED_IO: u32 = 200809;
pub const _LFS64_ASYNCHRONOUS_IO: u8 = 1;
pub const _LFS_LARGEFILE: u8 = 1;
pub const _LFS64_LARGEFILE: u8 = 1;
pub const _LFS64_STDIO: u8 = 1;
pub const _POSIX_SHARED_MEMORY_OBJECTS: u32 = 200809;
pub const _POSIX_CPUTIME: u8 = 0;
pub const _POSIX_THREAD_CPUTIME: u8 = 0;
pub const _POSIX_REGEXP: u8 = 1;
pub const _POSIX_READER_WRITER_LOCKS: u32 = 200809;
pub const _POSIX_SHELL: u8 = 1;
pub const _POSIX_TIMEOUTS: u32 = 200809;
pub const _POSIX_SPIN_LOCKS: u32 = 200809;
pub const _POSIX_SPAWN: u32 = 200809;
pub const _POSIX_TIMERS: u32 = 200809;
pub const _POSIX_BARRIERS: u32 = 200809;
pub const _POSIX_MESSAGE_PASSING: u32 = 200809;
pub const _POSIX_THREAD_PROCESS_SHARED: u32 = 200809;
pub const _POSIX_MONOTONIC_CLOCK: u8 = 0;
pub const _POSIX_CLOCK_SELECTION: u32 = 200809;
pub const _POSIX_ADVISORY_INFO: u32 = 200809;
pub const _POSIX_IPV6: u32 = 200809;
pub const _POSIX_RAW_SOCKETS: u32 = 200809;
pub const _POSIX2_CHAR_TERM: u32 = 200809;
pub const _POSIX_SPORADIC_SERVER: i8 = -1;
pub const _POSIX_THREAD_SPORADIC_SERVER: i8 = -1;
pub const _POSIX_TRACE: i8 = -1;
pub const _POSIX_TRACE_EVENT_FILTER: i8 = -1;
pub const _POSIX_TRACE_INHERIT: i8 = -1;
pub const _POSIX_TRACE_LOG: i8 = -1;
pub const _POSIX_TYPED_MEMORY_OBJECTS: i8 = -1;
pub const _POSIX_V7_LPBIG_OFFBIG: i8 = -1;
pub const _POSIX_V6_LPBIG_OFFBIG: i8 = -1;
pub const _XBS5_LPBIG_OFFBIG: i8 = -1;
pub const _POSIX_V7_LP64_OFF64: u8 = 1;
pub const _POSIX_V6_LP64_OFF64: u8 = 1;
pub const _XBS5_LP64_OFF64: u8 = 1;
pub const __ILP32_OFF32_CFLAGS: &[u8; 5] = b"-m32\0";
pub const __ILP32_OFF32_LDFLAGS: &[u8; 5] = b"-m32\0";
pub const __ILP32_OFFBIG_CFLAGS: &[u8; 48] = b"-m32 -D_LARGEFILE_SOURCE -D_FILE_OFFSET_BITS=64\0";
pub const __ILP32_OFFBIG_LDFLAGS: &[u8; 5] = b"-m32\0";
pub const __LP64_OFF64_CFLAGS: &[u8; 5] = b"-m64\0";
pub const __LP64_OFF64_LDFLAGS: &[u8; 5] = b"-m64\0";
pub const STDIN_FILENO: u8 = 0;
pub const STDOUT_FILENO: u8 = 1;
pub const STDERR_FILENO: u8 = 2;
pub const R_OK: u8 = 4;
pub const W_OK: u8 = 2;
pub const X_OK: u8 = 1;
pub const F_OK: u8 = 0;
pub const L_SET: u8 = 0;
pub const L_INCR: u8 = 1;
pub const L_XTND: u8 = 2;
pub const _GETOPT_POSIX_H: u8 = 1;
pub const _GETOPT_CORE_H: u8 = 1;
pub const F_ULOCK: u8 = 0;
pub const F_LOCK: u8 = 1;
pub const F_TLOCK: u8 = 2;
pub const F_TEST: u8 = 3;
pub const _FCNTL_H: u8 = 1;
pub const __O_LARGEFILE: u8 = 0;
pub const F_GETLK64: u8 = 5;
pub const F_SETLK64: u8 = 6;
pub const F_SETLKW64: u8 = 7;
pub const O_ACCMODE: u8 = 3;
pub const O_RDONLY: u8 = 0;
pub const O_WRONLY: u8 = 1;
pub const O_RDWR: u8 = 2;
pub const O_CREAT: u8 = 64;
pub const O_EXCL: u8 = 128;
pub const O_NOCTTY: u16 = 256;
pub const O_TRUNC: u16 = 512;
pub const O_APPEND: u16 = 1024;
pub const O_NONBLOCK: u16 = 2048;
pub const O_NDELAY: u16 = 2048;
pub const O_SYNC: u32 = 1052672;
pub const O_FSYNC: u32 = 1052672;
pub const O_ASYNC: u16 = 8192;
pub const __O_DIRECTORY: u32 = 65536;
pub const __O_NOFOLLOW: u32 = 131072;
pub const __O_CLOEXEC: u32 = 524288;
pub const __O_DIRECT: u16 = 16384;
pub const __O_NOATIME: u32 = 262144;
pub const __O_PATH: u32 = 2097152;
pub const __O_DSYNC: u16 = 4096;
pub const __O_TMPFILE: u32 = 4259840;
pub const F_GETLK: u8 = 5;
pub const F_SETLK: u8 = 6;
pub const F_SETLKW: u8 = 7;
pub const O_DIRECTORY: u32 = 65536;
pub const O_NOFOLLOW: u32 = 131072;
pub const O_CLOEXEC: u32 = 524288;
pub const O_DSYNC: u16 = 4096;
pub const O_RSYNC: u32 = 1052672;
pub const F_DUPFD: u8 = 0;
pub const F_GETFD: u8 = 1;
pub const F_SETFD: u8 = 2;
pub const F_GETFL: u8 = 3;
pub const F_SETFL: u8 = 4;
pub const __F_SETOWN: u8 = 8;
pub const __F_GETOWN: u8 = 9;
pub const F_SETOWN: u8 = 8;
pub const F_GETOWN: u8 = 9;
pub const __F_SETSIG: u8 = 10;
pub const __F_GETSIG: u8 = 11;
pub const __F_SETOWN_EX: u8 = 15;
pub const __F_GETOWN_EX: u8 = 16;
pub const F_DUPFD_CLOEXEC: u16 = 1030;
pub const FD_CLOEXEC: u8 = 1;
pub const F_RDLCK: u8 = 0;
pub const F_WRLCK: u8 = 1;
pub const F_UNLCK: u8 = 2;
pub const F_EXLCK: u8 = 4;
pub const F_SHLCK: u8 = 8;
pub const LOCK_SH: u8 = 1;
pub const LOCK_EX: u8 = 2;
pub const LOCK_NB: u8 = 4;
pub const LOCK_UN: u8 = 8;
pub const FAPPEND: u16 = 1024;
pub const FFSYNC: u32 = 1052672;
pub const FASYNC: u16 = 8192;
pub const FNONBLOCK: u16 = 2048;
pub const FNDELAY: u16 = 2048;
pub const __POSIX_FADV_DONTNEED: u8 = 4;
pub const __POSIX_FADV_NOREUSE: u8 = 5;
pub const POSIX_FADV_NORMAL: u8 = 0;
pub const POSIX_FADV_RANDOM: u8 = 1;
pub const POSIX_FADV_SEQUENTIAL: u8 = 2;
pub const POSIX_FADV_WILLNEED: u8 = 3;
pub const POSIX_FADV_DONTNEED: u8 = 4;
pub const POSIX_FADV_NOREUSE: u8 = 5;
pub const AT_FDCWD: i8 = -100;
pub const AT_SYMLINK_NOFOLLOW: u16 = 256;
pub const AT_REMOVEDIR: u16 = 512;
pub const AT_SYMLINK_FOLLOW: u16 = 1024;
pub const AT_EACCESS: u16 = 512;
pub const _ERRNO_H: u8 = 1;
pub const _BITS_ERRNO_H: u8 = 1;
pub const EPERM: u8 = 1;
pub const ENOENT: u8 = 2;
pub const ESRCH: u8 = 3;
pub const EINTR: u8 = 4;
pub const EIO: u8 = 5;
pub const ENXIO: u8 = 6;
pub const E2BIG: u8 = 7;
pub const ENOEXEC: u8 = 8;
pub const EBADF: u8 = 9;
pub const ECHILD: u8 = 10;
pub const EAGAIN: u8 = 11;
pub const ENOMEM: u8 = 12;
pub const EACCES: u8 = 13;
pub const EFAULT: u8 = 14;
pub const ENOTBLK: u8 = 15;
pub const EBUSY: u8 = 16;
pub const EEXIST: u8 = 17;
pub const EXDEV: u8 = 18;
pub const ENODEV: u8 = 19;
pub const ENOTDIR: u8 = 20;
pub const EISDIR: u8 = 21;
pub const EINVAL: u8 = 22;
pub const ENFILE: u8 = 23;
pub const EMFILE: u8 = 24;
pub const ENOTTY: u8 = 25;
pub const ETXTBSY: u8 = 26;
pub const EFBIG: u8 = 27;
pub const ENOSPC: u8 = 28;
pub const ESPIPE: u8 = 29;
pub const EROFS: u8 = 30;
pub const EMLINK: u8 = 31;
pub const EPIPE: u8 = 32;
pub const EDOM: u8 = 33;
pub const ERANGE: u8 = 34;
pub const EDEADLK: u8 = 35;
pub const ENAMETOOLONG: u8 = 36;
pub const ENOLCK: u8 = 37;
pub const ENOSYS: u8 = 38;
pub const ENOTEMPTY: u8 = 39;
pub const ELOOP: u8 = 40;
pub const EWOULDBLOCK: u8 = 11;
pub const ENOMSG: u8 = 42;
pub const EIDRM: u8 = 43;
pub const ECHRNG: u8 = 44;
pub const EL2NSYNC: u8 = 45;
pub const EL3HLT: u8 = 46;
pub const EL3RST: u8 = 47;
pub const ELNRNG: u8 = 48;
pub const EUNATCH: u8 = 49;
pub const ENOCSI: u8 = 50;
pub const EL2HLT: u8 = 51;
pub const EBADE: u8 = 52;
pub const EBADR: u8 = 53;
pub const EXFULL: u8 = 54;
pub const ENOANO: u8 = 55;
pub const EBADRQC: u8 = 56;
pub const EBADSLT: u8 = 57;
pub const EDEADLOCK: u8 = 35;
pub const EBFONT: u8 = 59;
pub const ENOSTR: u8 = 60;
pub const ENODATA: u8 = 61;
pub const ETIME: u8 = 62;
pub const ENOSR: u8 = 63;
pub const ENONET: u8 = 64;
pub const ENOPKG: u8 = 65;
pub const EREMOTE: u8 = 66;
pub const ENOLINK: u8 = 67;
pub const EADV: u8 = 68;
pub const ESRMNT: u8 = 69;
pub const ECOMM: u8 = 70;
pub const EPROTO: u8 = 71;
pub const EMULTIHOP: u8 = 72;
pub const EDOTDOT: u8 = 73;
pub const EBADMSG: u8 = 74;
pub const EOVERFLOW: u8 = 75;
pub const ENOTUNIQ: u8 = 76;
pub const EBADFD: u8 = 77;
pub const EREMCHG: u8 = 78;
pub const ELIBACC: u8 = 79;
pub const ELIBBAD: u8 = 80;
pub const ELIBSCN: u8 = 81;
pub const ELIBMAX: u8 = 82;
pub const ELIBEXEC: u8 = 83;
pub const EILSEQ: u8 = 84;
pub const ERESTART: u8 = 85;
pub const ESTRPIPE: u8 = 86;
pub const EUSERS: u8 = 87;
pub const ENOTSOCK: u8 = 88;
pub const EDESTADDRREQ: u8 = 89;
pub const EMSGSIZE: u8 = 90;
pub const EPROTOTYPE: u8 = 91;
pub const ENOPROTOOPT: u8 = 92;
pub const EPROTONOSUPPORT: u8 = 93;
pub const ESOCKTNOSUPPORT: u8 = 94;
pub const EOPNOTSUPP: u8 = 95;
pub const EPFNOSUPPORT: u8 = 96;
pub const EAFNOSUPPORT: u8 = 97;
pub const EADDRINUSE: u8 = 98;
pub const EADDRNOTAVAIL: u8 = 99;
pub const ENETDOWN: u8 = 100;
pub const ENETUNREACH: u8 = 101;
pub const ENETRESET: u8 = 102;
pub const ECONNABORTED: u8 = 103;
pub const ECONNRESET: u8 = 104;
pub const ENOBUFS: u8 = 105;
pub const EISCONN: u8 = 106;
pub const ENOTCONN: u8 = 107;
pub const ESHUTDOWN: u8 = 108;
pub const ETOOMANYREFS: u8 = 109;
pub const ETIMEDOUT: u8 = 110;
pub const ECONNREFUSED: u8 = 111;
pub const EHOSTDOWN: u8 = 112;
pub const EHOSTUNREACH: u8 = 113;
pub const EALREADY: u8 = 114;
pub const EINPROGRESS: u8 = 115;
pub const ESTALE: u8 = 116;
pub const EUCLEAN: u8 = 117;
pub const ENOTNAM: u8 = 118;
pub const ENAVAIL: u8 = 119;
pub const EISNAM: u8 = 120;
pub const EREMOTEIO: u8 = 121;
pub const EDQUOT: u8 = 122;
pub const ENOMEDIUM: u8 = 123;
pub const EMEDIUMTYPE: u8 = 124;
pub const ECANCELED: u8 = 125;
pub const ENOKEY: u8 = 126;
pub const EKEYEXPIRED: u8 = 127;
pub const EKEYREVOKED: u8 = 128;
pub const EKEYREJECTED: u8 = 129;
pub const EOWNERDEAD: u8 = 130;
pub const ENOTRECOVERABLE: u8 = 131;
pub const ERFKILL: u8 = 132;
pub const EHWPOISON: u8 = 133;
pub const ENOTSUP: u8 = 95;
pub const _EXECINFO_H: u8 = 1;
pub const _TIME_H: u8 = 1;
pub const _BITS_TIME_H: u8 = 1;
pub const CLOCK_REALTIME: u8 = 0;
pub const CLOCK_MONOTONIC: u8 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u8 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u8 = 3;
pub const CLOCK_MONOTONIC_RAW: u8 = 4;
pub const CLOCK_REALTIME_COARSE: u8 = 5;
pub const CLOCK_MONOTONIC_COARSE: u8 = 6;
pub const CLOCK_BOOTTIME: u8 = 7;
pub const CLOCK_REALTIME_ALARM: u8 = 8;
pub const CLOCK_BOOTTIME_ALARM: u8 = 9;
pub const CLOCK_TAI: u8 = 11;
pub const TIMER_ABSTIME: u8 = 1;
pub const __struct_tm_defined: u8 = 1;
pub const __itimerspec_defined: u8 = 1;
pub const _PTHREAD_H: u8 = 1;
pub const _SCHED_H: u8 = 1;
pub const _BITS_SCHED_H: u8 = 1;
pub const SCHED_OTHER: u8 = 0;
pub const SCHED_FIFO: u8 = 1;
pub const SCHED_RR: u8 = 2;
pub const _BITS_TYPES_STRUCT_SCHED_PARAM: u8 = 1;
pub const _BITS_CPU_SET_H: u8 = 1;
pub const __CPU_SETSIZE: u16 = 1024;
pub const _BITS_SETJMP_H: u8 = 1;
pub const __jmp_buf_tag_defined: u8 = 1;
pub const PTHREAD_STACK_MIN: u16 = 16384;
pub const PTHREAD_ONCE_INIT: u8 = 0;
pub const PTHREAD_BARRIER_SERIAL_THREAD: i8 = -1;
pub const _ASSERT_H: u8 = 1;
pub const MAX_VALUE_U1: u8 = 255;
pub const MAX_U1: u8 = 255;
pub const MAX_VALUE_U2: u16 = 65535;
pub const MAX_U2: u16 = 65535;
pub const MAX_VALUE_U4: u32 = 4294967295;
pub const MAX_U4: u32 = 4294967295;
pub const MAX_VALUE_U8: i8 = -1;
pub const MAX_U8: i8 = -1;
pub const MAX_VALUE_B1: u8 = 127;
pub const MAX_B1: u8 = 127;
pub const MAX_VALUE_B2: u16 = 32767;
pub const MAX_B2: u16 = 32767;
pub const MAX_VALUE_B4: u32 = 2147483647;
pub const MAX_B4: u32 = 2147483647;
pub const MAX_VALUE_B8: u64 = 9223372036854775807;
pub const MAX_B8: u64 = 9223372036854775807;
pub const SYS_ALIGNED_BASE: u8 = 8;
pub const _DEBUG_LOGFILE_NO_: u8 = 2;
pub const STEP_WISE_BUFF_SIZE: u16 = 65535;
pub const OBJ_DESC_MAX_CHILD: u8 = 64;
pub const MEM_PTR_TYPE_DUMP: u8 = 1;
pub const MEM_PTR_TYPE_POINTER: u8 = 2;
pub const LIST_MAX_N_HEAD: u8 = 15;
pub const _MM_HINT_ET0: u8 = 7;
pub const _MM_HINT_ET1: u8 = 6;
pub const _MM_HINT_T0: u8 = 3;
pub const _MM_HINT_T1: u8 = 2;
pub const _MM_HINT_T2: u8 = 1;
pub const _MM_HINT_NTA: u8 = 0;
pub const _MM_EXCEPT_INVALID: u8 = 1;
pub const _MM_EXCEPT_DENORM: u8 = 2;
pub const _MM_EXCEPT_DIV_ZERO: u8 = 4;
pub const _MM_EXCEPT_OVERFLOW: u8 = 8;
pub const _MM_EXCEPT_UNDERFLOW: u8 = 16;
pub const _MM_EXCEPT_INEXACT: u8 = 32;
pub const _MM_EXCEPT_MASK: u8 = 63;
pub const _MM_MASK_INVALID: u8 = 128;
pub const _MM_MASK_DENORM: u16 = 256;
pub const _MM_MASK_DIV_ZERO: u16 = 512;
pub const _MM_MASK_OVERFLOW: u16 = 1024;
pub const _MM_MASK_UNDERFLOW: u16 = 2048;
pub const _MM_MASK_INEXACT: u16 = 4096;
pub const _MM_MASK_MASK: u16 = 8064;
pub const _MM_ROUND_NEAREST: u8 = 0;
pub const _MM_ROUND_DOWN: u16 = 8192;
pub const _MM_ROUND_UP: u16 = 16384;
pub const _MM_ROUND_TOWARD_ZERO: u16 = 24576;
pub const _MM_ROUND_MASK: u16 = 24576;
pub const _MM_FLUSH_ZERO_MASK: u16 = 32768;
pub const _MM_FLUSH_ZERO_ON: u16 = 32768;
pub const _MM_FLUSH_ZERO_OFF: u8 = 0;
pub const _MM_DENORMALS_ZERO_ON: u8 = 64;
pub const _MM_DENORMALS_ZERO_OFF: u8 = 0;
pub const _MM_DENORMALS_ZERO_MASK: u8 = 64;
pub const _MM_FROUND_TO_NEAREST_INT: u8 = 0;
pub const _MM_FROUND_TO_NEG_INF: u8 = 1;
pub const _MM_FROUND_TO_POS_INF: u8 = 2;
pub const _MM_FROUND_TO_ZERO: u8 = 3;
pub const _MM_FROUND_CUR_DIRECTION: u8 = 4;
pub const _MM_FROUND_RAISE_EXC: u8 = 0;
pub const _MM_FROUND_NO_EXC: u8 = 8;
pub const _MM_FROUND_NINT: u8 = 0;
pub const _MM_FROUND_FLOOR: u8 = 1;
pub const _MM_FROUND_CEIL: u8 = 2;
pub const _MM_FROUND_TRUNC: u8 = 3;
pub const _MM_FROUND_RINT: u8 = 4;
pub const _MM_FROUND_NEARBYINT: u8 = 12;
pub const _SIDD_UBYTE_OPS: u8 = 0;
pub const _SIDD_UWORD_OPS: u8 = 1;
pub const _SIDD_SBYTE_OPS: u8 = 2;
pub const _SIDD_SWORD_OPS: u8 = 3;
pub const _SIDD_CMP_EQUAL_ANY: u8 = 0;
pub const _SIDD_CMP_RANGES: u8 = 4;
pub const _SIDD_CMP_EQUAL_EACH: u8 = 8;
pub const _SIDD_CMP_EQUAL_ORDERED: u8 = 12;
pub const _SIDD_POSITIVE_POLARITY: u8 = 0;
pub const _SIDD_NEGATIVE_POLARITY: u8 = 16;
pub const _SIDD_MASKED_POSITIVE_POLARITY: u8 = 32;
pub const _SIDD_MASKED_NEGATIVE_POLARITY: u8 = 48;
pub const _SIDD_LEAST_SIGNIFICANT: u8 = 0;
pub const _SIDD_MOST_SIGNIFICANT: u8 = 64;
pub const _SIDD_BIT_MASK: u8 = 0;
pub const _SIDD_UNIT_MASK: u8 = 64;
pub const SEQALIGN_MODE_GLOBAL: u8 = 0;
pub const SEQALIGN_MODE_OVERLAP: u8 = 1;
pub const SEQALIGN_MODE_EXTEND: u8 = 2;
pub const SEQALIGN_MODE_KMER: u8 = 3;
pub const SEQALIGN_MODEMASK_TYPE: u8 = 3;
pub const SEQALIGN_MODE_QPROF: u8 = 4;
pub const SEQALIGN_MODE_MEMRESV: u8 = 8;
pub const SEQALIGN_MODE_CIGRESV: u8 = 16;
pub const SEQALIGN_BT_M: u8 = 0;
pub const SEQALIGN_BT_I: u8 = 1;
pub const SEQALIGN_BT_D: u8 = 2;
pub const SEQALIGN_BT1_IE: u8 = 4;
pub const SEQALIGN_BT1_DE: u8 = 8;
pub const SEQALIGN_BT2_I1: u8 = 1;
pub const SEQALIGN_BT2_D1: u8 = 2;
pub const SEQALIGN_BT2_I2: u8 = 3;
pub const SEQALIGN_BT2_D2: u8 = 4;
pub const SEQALIGN_BT2_IE1: u8 = 8;
pub const SEQALIGN_BT2_DE1: u8 = 16;
pub const SEQALIGN_BT2_IE2: u8 = 32;
pub const SEQALIGN_BT2_DE2: u8 = 64;
pub const SEQALIGN_SCORE_EPI8_MIN: i8 = -63;
pub const SEQALIGN_SCORE_EPI8_MAX: u8 = 63;
pub const SEQALIGN_SCORE_MIN: i32 = -536870911;
pub const SEQALIGN_SCORE_MAX: u32 = 536870911;
pub const SEQALIGN_CIGAR_M: u8 = 0;
pub const SEQALIGN_CIGAR_I: u8 = 1;
pub const SEQALIGN_CIGAR_D: u8 = 2;
pub const SEQALIGN_CIGAR_N: u8 = 3;
pub const SEQALIGN_CIGAR_S: u8 = 4;
pub const SEQALIGN_CIGAR_H: u8 = 5;
pub const SEQALIGN_CIGAR_P: u8 = 6;
pub const SEQALIGN_CIGAR_E: u8 = 7;
pub const SEQALIGN_CIGAR_X: u8 = 8;
pub const WORDSIZE: u8 = 16;
pub const WORDSHIFT: u8 = 4;
pub const _MATH_H: u8 = 1;
pub const _BITS_LIBM_SIMD_DECL_STUBS_H: u8 = 1;
pub const __FP_LOGB0_IS_MIN: u8 = 1;
pub const __FP_LOGBNAN_IS_MIN: u8 = 1;
pub const FP_ILOGB0: i32 = -2147483648;
pub const FP_ILOGBNAN: i32 = -2147483648;
pub const __MATH_DECLARING_DOUBLE: u8 = 1;
pub const __MATH_DECLARING_FLOATN: u8 = 0;
pub const __MATH_DECLARE_LDOUBLE: u8 = 1;
pub const MATH_ERRNO: u8 = 1;
pub const MATH_ERREXCEPT: u8 = 2;
pub const math_errhandling: u8 = 3;
pub const M_E: f64 = 2.718281828459045;
pub const M_LOG2E: f64 = 1.4426950408889634;
pub const M_LOG10E: f64 = 0.4342944819032518;
pub const M_LN2: f64 = 0.6931471805599453;
pub const M_LN10: f64 = 2.302585092994046;
pub const M_PI: f64 = 3.141592653589793;
pub const M_PI_2: f64 = 1.5707963267948966;
pub const M_PI_4: f64 = 0.7853981633974483;
pub const M_1_PI: f64 = 0.3183098861837907;
pub const M_2_PI: f64 = 0.6366197723675814;
pub const M_2_SQRTPI: f64 = 1.1283791670955126;
pub const M_SQRT2: f64 = 1.4142135623730951;
pub const M_SQRT1_2: f64 = 0.7071067811865476;
pub const SEQALIGN_MODE_POA: u8 = 16;
pub const BSPOA_RDLEN_MAX: u32 = 268435455;
pub const BSPOA_RDCNT_MAX: u16 = 16383;
pub const BSPOA_VST_MAX: u16 = 65535;
pub const BSPOA_MIN_LOGVAL: i32 = -1000000000;
pub const BSPOA_QLT_MAX: u8 = 90;
pub const BSPOA_EMOVTYPE_MOVALL: u16 = 3855;
pub const BSPOA_EMOVTYPE_KPTONE: u16 = 7695;
pub const BSPOA_EMOVTYPE_MOVONE: u16 = 57840;
pub const BSPOA_RDNODE_CUTEDGE: u8 = 1;
pub const BSPOA_RDNODE_CUTNODE: u8 = 2;
pub const BSPOA_RDNODE_CUTALL: u8 = 3;
pub const MAX_LOG_CACHE: u16 = 1000;
pub const BS_M_SQRT2: f64 = 1.4142135623731;
pub const BSPOA_HSP_MINLEN: u8 = 3;
pub type wchar_t = ::std::os::raw::c_int;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of div_t"][::std::mem::size_of::<div_t>() - 8usize];
    ["Alignment of div_t"][::std::mem::align_of::<div_t>() - 4usize];
    ["Offset of field: div_t::quot"][::std::mem::offset_of!(div_t, quot) - 0usize];
    ["Offset of field: div_t::rem"][::std::mem::offset_of!(div_t, rem) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ldiv_t"][::std::mem::size_of::<ldiv_t>() - 16usize];
    ["Alignment of ldiv_t"][::std::mem::align_of::<ldiv_t>() - 8usize];
    ["Offset of field: ldiv_t::quot"][::std::mem::offset_of!(ldiv_t, quot) - 0usize];
    ["Offset of field: ldiv_t::rem"][::std::mem::offset_of!(ldiv_t, rem) - 8usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of lldiv_t"][::std::mem::size_of::<lldiv_t>() - 16usize];
    ["Alignment of lldiv_t"][::std::mem::align_of::<lldiv_t>() - 8usize];
    ["Offset of field: lldiv_t::quot"][::std::mem::offset_of!(lldiv_t, quot) - 0usize];
    ["Offset of field: lldiv_t::rem"][::std::mem::offset_of!(lldiv_t, rem) - 8usize];
};
unsafe extern "C" {
    pub fn __ctype_get_mb_cur_max() -> usize;
}
unsafe extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
unsafe extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
unsafe extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
unsafe extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
unsafe extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
unsafe extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
unsafe extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
    ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
    ["Offset of field: __fsid_t::__val"][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
};
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __sigset_t"][::std::mem::size_of::<__sigset_t>() - 128usize];
    ["Alignment of __sigset_t"][::std::mem::align_of::<__sigset_t>() - 8usize];
    ["Offset of field: __sigset_t::__val"][::std::mem::offset_of!(__sigset_t, __val) - 0usize];
};
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timeval"][::std::mem::size_of::<timeval>() - 16usize];
    ["Alignment of timeval"][::std::mem::align_of::<timeval>() - 8usize];
    ["Offset of field: timeval::tv_sec"][::std::mem::offset_of!(timeval, tv_sec) - 0usize];
    ["Offset of field: timeval::tv_usec"][::std::mem::offset_of!(timeval, tv_usec) - 8usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timespec"][::std::mem::size_of::<timespec>() - 16usize];
    ["Alignment of timespec"][::std::mem::align_of::<timespec>() - 8usize];
    ["Offset of field: timespec::tv_sec"][::std::mem::offset_of!(timespec, tv_sec) - 0usize];
    ["Offset of field: timespec::tv_nsec"][::std::mem::offset_of!(timespec, tv_nsec) - 8usize];
};
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of fd_set"][::std::mem::size_of::<fd_set>() - 128usize];
    ["Alignment of fd_set"][::std::mem::align_of::<fd_set>() - 8usize];
    ["Offset of field: fd_set::__fds_bits"][::std::mem::offset_of!(fd_set, __fds_bits) - 0usize];
};
pub type fd_mask = __fd_mask;
unsafe extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::std::os::raw::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __atomic_wide_counter__bindgen_ty_1"]
        [::std::mem::size_of::<__atomic_wide_counter__bindgen_ty_1>() - 8usize];
    ["Alignment of __atomic_wide_counter__bindgen_ty_1"]
        [::std::mem::align_of::<__atomic_wide_counter__bindgen_ty_1>() - 4usize];
    ["Offset of field: __atomic_wide_counter__bindgen_ty_1::__low"]
        [::std::mem::offset_of!(__atomic_wide_counter__bindgen_ty_1, __low) - 0usize];
    ["Offset of field: __atomic_wide_counter__bindgen_ty_1::__high"]
        [::std::mem::offset_of!(__atomic_wide_counter__bindgen_ty_1, __high) - 4usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __atomic_wide_counter"][::std::mem::size_of::<__atomic_wide_counter>() - 8usize];
    ["Alignment of __atomic_wide_counter"]
        [::std::mem::align_of::<__atomic_wide_counter>() - 8usize];
    ["Offset of field: __atomic_wide_counter::__value64"]
        [::std::mem::offset_of!(__atomic_wide_counter, __value64) - 0usize];
    ["Offset of field: __atomic_wide_counter::__value32"]
        [::std::mem::offset_of!(__atomic_wide_counter, __value32) - 0usize];
};
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_internal_list"][::std::mem::size_of::<__pthread_internal_list>() - 16usize];
    ["Alignment of __pthread_internal_list"]
        [::std::mem::align_of::<__pthread_internal_list>() - 8usize];
    ["Offset of field: __pthread_internal_list::__prev"]
        [::std::mem::offset_of!(__pthread_internal_list, __prev) - 0usize];
    ["Offset of field: __pthread_internal_list::__next"]
        [::std::mem::offset_of!(__pthread_internal_list, __next) - 8usize];
};
impl Default for __pthread_internal_list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_internal_slist"]
        [::std::mem::size_of::<__pthread_internal_slist>() - 8usize];
    ["Alignment of __pthread_internal_slist"]
        [::std::mem::align_of::<__pthread_internal_slist>() - 8usize];
    ["Offset of field: __pthread_internal_slist::__next"]
        [::std::mem::offset_of!(__pthread_internal_slist, __next) - 0usize];
};
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_mutex_s"][::std::mem::size_of::<__pthread_mutex_s>() - 40usize];
    ["Alignment of __pthread_mutex_s"][::std::mem::align_of::<__pthread_mutex_s>() - 8usize];
    ["Offset of field: __pthread_mutex_s::__lock"]
        [::std::mem::offset_of!(__pthread_mutex_s, __lock) - 0usize];
    ["Offset of field: __pthread_mutex_s::__count"]
        [::std::mem::offset_of!(__pthread_mutex_s, __count) - 4usize];
    ["Offset of field: __pthread_mutex_s::__owner"]
        [::std::mem::offset_of!(__pthread_mutex_s, __owner) - 8usize];
    ["Offset of field: __pthread_mutex_s::__nusers"]
        [::std::mem::offset_of!(__pthread_mutex_s, __nusers) - 12usize];
    ["Offset of field: __pthread_mutex_s::__kind"]
        [::std::mem::offset_of!(__pthread_mutex_s, __kind) - 16usize];
    ["Offset of field: __pthread_mutex_s::__spins"]
        [::std::mem::offset_of!(__pthread_mutex_s, __spins) - 20usize];
    ["Offset of field: __pthread_mutex_s::__elision"]
        [::std::mem::offset_of!(__pthread_mutex_s, __elision) - 22usize];
    ["Offset of field: __pthread_mutex_s::__list"]
        [::std::mem::offset_of!(__pthread_mutex_s, __list) - 24usize];
};
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_rwlock_arch_t"][::std::mem::size_of::<__pthread_rwlock_arch_t>() - 56usize];
    ["Alignment of __pthread_rwlock_arch_t"]
        [::std::mem::align_of::<__pthread_rwlock_arch_t>() - 8usize];
    ["Offset of field: __pthread_rwlock_arch_t::__readers"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __readers) - 0usize];
    ["Offset of field: __pthread_rwlock_arch_t::__writers"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __writers) - 4usize];
    ["Offset of field: __pthread_rwlock_arch_t::__wrphase_futex"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __wrphase_futex) - 8usize];
    ["Offset of field: __pthread_rwlock_arch_t::__writers_futex"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __writers_futex) - 12usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad3"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad3) - 16usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad4"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad4) - 20usize];
    ["Offset of field: __pthread_rwlock_arch_t::__cur_writer"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __cur_writer) - 24usize];
    ["Offset of field: __pthread_rwlock_arch_t::__shared"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __shared) - 28usize];
    ["Offset of field: __pthread_rwlock_arch_t::__rwelision"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __rwelision) - 32usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad1"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad1) - 33usize];
    ["Offset of field: __pthread_rwlock_arch_t::__pad2"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __pad2) - 40usize];
    ["Offset of field: __pthread_rwlock_arch_t::__flags"]
        [::std::mem::offset_of!(__pthread_rwlock_arch_t, __flags) - 48usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_cond_s"][::std::mem::size_of::<__pthread_cond_s>() - 48usize];
    ["Alignment of __pthread_cond_s"][::std::mem::align_of::<__pthread_cond_s>() - 8usize];
    ["Offset of field: __pthread_cond_s::__wseq"]
        [::std::mem::offset_of!(__pthread_cond_s, __wseq) - 0usize];
    ["Offset of field: __pthread_cond_s::__g1_start"]
        [::std::mem::offset_of!(__pthread_cond_s, __g1_start) - 8usize];
    ["Offset of field: __pthread_cond_s::__g_refs"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_refs) - 16usize];
    ["Offset of field: __pthread_cond_s::__g_size"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_size) - 24usize];
    ["Offset of field: __pthread_cond_s::__g1_orig_size"]
        [::std::mem::offset_of!(__pthread_cond_s, __g1_orig_size) - 32usize];
    ["Offset of field: __pthread_cond_s::__wrefs"]
        [::std::mem::offset_of!(__pthread_cond_s, __wrefs) - 36usize];
    ["Offset of field: __pthread_cond_s::__g_signals"]
        [::std::mem::offset_of!(__pthread_cond_s, __g_signals) - 40usize];
};
impl Default for __pthread_cond_s {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __tss_t = ::std::os::raw::c_uint;
pub type __thrd_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __once_flag {
    pub __data: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __once_flag"][::std::mem::size_of::<__once_flag>() - 4usize];
    ["Alignment of __once_flag"][::std::mem::align_of::<__once_flag>() - 4usize];
    ["Offset of field: __once_flag::__data"][::std::mem::offset_of!(__once_flag, __data) - 0usize];
};
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_mutexattr_t"][::std::mem::size_of::<pthread_mutexattr_t>() - 4usize];
    ["Alignment of pthread_mutexattr_t"][::std::mem::align_of::<pthread_mutexattr_t>() - 4usize];
    ["Offset of field: pthread_mutexattr_t::__size"]
        [::std::mem::offset_of!(pthread_mutexattr_t, __size) - 0usize];
    ["Offset of field: pthread_mutexattr_t::__align"]
        [::std::mem::offset_of!(pthread_mutexattr_t, __align) - 0usize];
};
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_condattr_t"][::std::mem::size_of::<pthread_condattr_t>() - 4usize];
    ["Alignment of pthread_condattr_t"][::std::mem::align_of::<pthread_condattr_t>() - 4usize];
    ["Offset of field: pthread_condattr_t::__size"]
        [::std::mem::offset_of!(pthread_condattr_t, __size) - 0usize];
    ["Offset of field: pthread_condattr_t::__align"]
        [::std::mem::offset_of!(pthread_condattr_t, __align) - 0usize];
};
impl Default for pthread_condattr_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_attr_t"][::std::mem::size_of::<pthread_attr_t>() - 56usize];
    ["Alignment of pthread_attr_t"][::std::mem::align_of::<pthread_attr_t>() - 8usize];
    ["Offset of field: pthread_attr_t::__size"]
        [::std::mem::offset_of!(pthread_attr_t, __size) - 0usize];
    ["Offset of field: pthread_attr_t::__align"]
        [::std::mem::offset_of!(pthread_attr_t, __align) - 0usize];
};
impl Default for pthread_attr_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_mutex_t"][::std::mem::size_of::<pthread_mutex_t>() - 40usize];
    ["Alignment of pthread_mutex_t"][::std::mem::align_of::<pthread_mutex_t>() - 8usize];
    ["Offset of field: pthread_mutex_t::__data"]
        [::std::mem::offset_of!(pthread_mutex_t, __data) - 0usize];
    ["Offset of field: pthread_mutex_t::__size"]
        [::std::mem::offset_of!(pthread_mutex_t, __size) - 0usize];
    ["Offset of field: pthread_mutex_t::__align"]
        [::std::mem::offset_of!(pthread_mutex_t, __align) - 0usize];
};
impl Default for pthread_mutex_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_cond_t"][::std::mem::size_of::<pthread_cond_t>() - 48usize];
    ["Alignment of pthread_cond_t"][::std::mem::align_of::<pthread_cond_t>() - 8usize];
    ["Offset of field: pthread_cond_t::__data"]
        [::std::mem::offset_of!(pthread_cond_t, __data) - 0usize];
    ["Offset of field: pthread_cond_t::__size"]
        [::std::mem::offset_of!(pthread_cond_t, __size) - 0usize];
    ["Offset of field: pthread_cond_t::__align"]
        [::std::mem::offset_of!(pthread_cond_t, __align) - 0usize];
};
impl Default for pthread_cond_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_rwlock_t"][::std::mem::size_of::<pthread_rwlock_t>() - 56usize];
    ["Alignment of pthread_rwlock_t"][::std::mem::align_of::<pthread_rwlock_t>() - 8usize];
    ["Offset of field: pthread_rwlock_t::__data"]
        [::std::mem::offset_of!(pthread_rwlock_t, __data) - 0usize];
    ["Offset of field: pthread_rwlock_t::__size"]
        [::std::mem::offset_of!(pthread_rwlock_t, __size) - 0usize];
    ["Offset of field: pthread_rwlock_t::__align"]
        [::std::mem::offset_of!(pthread_rwlock_t, __align) - 0usize];
};
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_rwlockattr_t"][::std::mem::size_of::<pthread_rwlockattr_t>() - 8usize];
    ["Alignment of pthread_rwlockattr_t"][::std::mem::align_of::<pthread_rwlockattr_t>() - 8usize];
    ["Offset of field: pthread_rwlockattr_t::__size"]
        [::std::mem::offset_of!(pthread_rwlockattr_t, __size) - 0usize];
    ["Offset of field: pthread_rwlockattr_t::__align"]
        [::std::mem::offset_of!(pthread_rwlockattr_t, __align) - 0usize];
};
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_barrier_t"][::std::mem::size_of::<pthread_barrier_t>() - 32usize];
    ["Alignment of pthread_barrier_t"][::std::mem::align_of::<pthread_barrier_t>() - 8usize];
    ["Offset of field: pthread_barrier_t::__size"]
        [::std::mem::offset_of!(pthread_barrier_t, __size) - 0usize];
    ["Offset of field: pthread_barrier_t::__align"]
        [::std::mem::offset_of!(pthread_barrier_t, __align) - 0usize];
};
impl Default for pthread_barrier_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of pthread_barrierattr_t"][::std::mem::size_of::<pthread_barrierattr_t>() - 4usize];
    ["Alignment of pthread_barrierattr_t"]
        [::std::mem::align_of::<pthread_barrierattr_t>() - 4usize];
    ["Offset of field: pthread_barrierattr_t::__size"]
        [::std::mem::offset_of!(pthread_barrierattr_t, __size) - 0usize];
    ["Offset of field: pthread_barrierattr_t::__align"]
        [::std::mem::offset_of!(pthread_barrierattr_t, __align) - 0usize];
};
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of random_data"][::std::mem::size_of::<random_data>() - 48usize];
    ["Alignment of random_data"][::std::mem::align_of::<random_data>() - 8usize];
    ["Offset of field: random_data::fptr"][::std::mem::offset_of!(random_data, fptr) - 0usize];
    ["Offset of field: random_data::rptr"][::std::mem::offset_of!(random_data, rptr) - 8usize];
    ["Offset of field: random_data::state"][::std::mem::offset_of!(random_data, state) - 16usize];
    ["Offset of field: random_data::rand_type"]
        [::std::mem::offset_of!(random_data, rand_type) - 24usize];
    ["Offset of field: random_data::rand_deg"]
        [::std::mem::offset_of!(random_data, rand_deg) - 28usize];
    ["Offset of field: random_data::rand_sep"]
        [::std::mem::offset_of!(random_data, rand_sep) - 32usize];
    ["Offset of field: random_data::end_ptr"]
        [::std::mem::offset_of!(random_data, end_ptr) - 40usize];
};
impl Default for random_data {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
unsafe extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn drand48() -> f64;
}
unsafe extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
unsafe extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
unsafe extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
unsafe extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of drand48_data"][::std::mem::size_of::<drand48_data>() - 24usize];
    ["Alignment of drand48_data"][::std::mem::align_of::<drand48_data>() - 8usize];
    ["Offset of field: drand48_data::__x"][::std::mem::offset_of!(drand48_data, __x) - 0usize];
    ["Offset of field: drand48_data::__old_x"]
        [::std::mem::offset_of!(drand48_data, __old_x) - 6usize];
    ["Offset of field: drand48_data::__c"][::std::mem::offset_of!(drand48_data, __c) - 12usize];
    ["Offset of field: drand48_data::__init"]
        [::std::mem::offset_of!(drand48_data, __init) - 14usize];
    ["Offset of field: drand48_data::__a"][::std::mem::offset_of!(drand48_data, __a) - 16usize];
};
unsafe extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn arc4random() -> __uint32_t;
}
unsafe extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __size: usize);
}
unsafe extern "C" {
    pub fn arc4random_uniform(__upper_bound: __uint32_t) -> __uint32_t;
}
unsafe extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn calloc(
        __nmemb: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn alloca(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn abort() -> !;
}
unsafe extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int) -> !;
}
unsafe extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int) -> !;
}
unsafe extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
unsafe extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
unsafe extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
unsafe extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
unsafe extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
unsafe extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn qecvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn qfcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn qgcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn qecvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn qfcvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
unsafe extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
unsafe extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __memcmpeq(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __locale_struct"][::std::mem::size_of::<__locale_struct>() - 232usize];
    ["Alignment of __locale_struct"][::std::mem::align_of::<__locale_struct>() - 8usize];
    ["Offset of field: __locale_struct::__locales"]
        [::std::mem::offset_of!(__locale_struct, __locales) - 0usize];
    ["Offset of field: __locale_struct::__ctype_b"]
        [::std::mem::offset_of!(__locale_struct, __ctype_b) - 104usize];
    ["Offset of field: __locale_struct::__ctype_tolower"]
        [::std::mem::offset_of!(__locale_struct, __ctype_tolower) - 112usize];
    ["Offset of field: __locale_struct::__ctype_toupper"]
        [::std::mem::offset_of!(__locale_struct, __ctype_toupper) - 120usize];
    ["Offset of field: __locale_struct::__names"]
        [::std::mem::offset_of!(__locale_struct, __names) - 128usize];
};
impl Default for __locale_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
unsafe extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
unsafe extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
unsafe extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
unsafe extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: ::std::os::raw::c_ulong);
}
unsafe extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
unsafe extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t__bindgen_ty_1"]
        [::std::mem::size_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    ["Alignment of __mbstate_t__bindgen_ty_1"]
        [::std::mem::align_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    ["Offset of field: __mbstate_t__bindgen_ty_1::__wch"]
        [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wch) - 0usize];
    ["Offset of field: __mbstate_t__bindgen_ty_1::__wchb"]
        [::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wchb) - 0usize];
};
impl Default for __mbstate_t__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 8usize];
    ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 4usize];
    ["Offset of field: __mbstate_t::__count"]
        [::std::mem::offset_of!(__mbstate_t, __count) - 0usize];
    ["Offset of field: __mbstate_t::__value"]
        [::std::mem::offset_of!(__mbstate_t, __value) - 4usize];
};
impl Default for __mbstate_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos_t"][::std::mem::size_of::<_G_fpos_t>() - 16usize];
    ["Alignment of _G_fpos_t"][::std::mem::align_of::<_G_fpos_t>() - 8usize];
    ["Offset of field: _G_fpos_t::__pos"][::std::mem::offset_of!(_G_fpos_t, __pos) - 0usize];
    ["Offset of field: _G_fpos_t::__state"][::std::mem::offset_of!(_G_fpos_t, __state) - 8usize];
};
impl Default for _G_fpos_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos64_t"][::std::mem::size_of::<_G_fpos64_t>() - 16usize];
    ["Alignment of _G_fpos64_t"][::std::mem::align_of::<_G_fpos64_t>() - 8usize];
    ["Offset of field: _G_fpos64_t::__pos"][::std::mem::offset_of!(_G_fpos64_t, __pos) - 0usize];
    ["Offset of field: _G_fpos64_t::__state"]
        [::std::mem::offset_of!(_G_fpos64_t, __state) - 8usize];
};
impl Default for _G_fpos64_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _IO_FILE"][::std::mem::size_of::<_IO_FILE>() - 216usize];
    ["Alignment of _IO_FILE"][::std::mem::align_of::<_IO_FILE>() - 8usize];
    ["Offset of field: _IO_FILE::_flags"][::std::mem::offset_of!(_IO_FILE, _flags) - 0usize];
    ["Offset of field: _IO_FILE::_IO_read_ptr"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_ptr) - 8usize];
    ["Offset of field: _IO_FILE::_IO_read_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_end) - 16usize];
    ["Offset of field: _IO_FILE::_IO_read_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_read_base) - 24usize];
    ["Offset of field: _IO_FILE::_IO_write_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_base) - 32usize];
    ["Offset of field: _IO_FILE::_IO_write_ptr"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_ptr) - 40usize];
    ["Offset of field: _IO_FILE::_IO_write_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_write_end) - 48usize];
    ["Offset of field: _IO_FILE::_IO_buf_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_buf_base) - 56usize];
    ["Offset of field: _IO_FILE::_IO_buf_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_buf_end) - 64usize];
    ["Offset of field: _IO_FILE::_IO_save_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_save_base) - 72usize];
    ["Offset of field: _IO_FILE::_IO_backup_base"]
        [::std::mem::offset_of!(_IO_FILE, _IO_backup_base) - 80usize];
    ["Offset of field: _IO_FILE::_IO_save_end"]
        [::std::mem::offset_of!(_IO_FILE, _IO_save_end) - 88usize];
    ["Offset of field: _IO_FILE::_markers"][::std::mem::offset_of!(_IO_FILE, _markers) - 96usize];
    ["Offset of field: _IO_FILE::_chain"][::std::mem::offset_of!(_IO_FILE, _chain) - 104usize];
    ["Offset of field: _IO_FILE::_fileno"][::std::mem::offset_of!(_IO_FILE, _fileno) - 112usize];
    ["Offset of field: _IO_FILE::_flags2"][::std::mem::offset_of!(_IO_FILE, _flags2) - 116usize];
    ["Offset of field: _IO_FILE::_old_offset"]
        [::std::mem::offset_of!(_IO_FILE, _old_offset) - 120usize];
    ["Offset of field: _IO_FILE::_cur_column"]
        [::std::mem::offset_of!(_IO_FILE, _cur_column) - 128usize];
    ["Offset of field: _IO_FILE::_vtable_offset"]
        [::std::mem::offset_of!(_IO_FILE, _vtable_offset) - 130usize];
    ["Offset of field: _IO_FILE::_shortbuf"]
        [::std::mem::offset_of!(_IO_FILE, _shortbuf) - 131usize];
    ["Offset of field: _IO_FILE::_lock"][::std::mem::offset_of!(_IO_FILE, _lock) - 136usize];
    ["Offset of field: _IO_FILE::_offset"][::std::mem::offset_of!(_IO_FILE, _offset) - 144usize];
    ["Offset of field: _IO_FILE::_codecvt"][::std::mem::offset_of!(_IO_FILE, _codecvt) - 152usize];
    ["Offset of field: _IO_FILE::_wide_data"]
        [::std::mem::offset_of!(_IO_FILE, _wide_data) - 160usize];
    ["Offset of field: _IO_FILE::_freeres_list"]
        [::std::mem::offset_of!(_IO_FILE, _freeres_list) - 168usize];
    ["Offset of field: _IO_FILE::_freeres_buf"]
        [::std::mem::offset_of!(_IO_FILE, _freeres_buf) - 176usize];
    ["Offset of field: _IO_FILE::__pad5"][::std::mem::offset_of!(_IO_FILE, __pad5) - 184usize];
    ["Offset of field: _IO_FILE::_mode"][::std::mem::offset_of!(_IO_FILE, _mode) - 192usize];
    ["Offset of field: _IO_FILE::_unused2"][::std::mem::offset_of!(_IO_FILE, _unused2) - 196usize];
};
impl Default for _IO_FILE {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type fpos_t = __fpos_t;
unsafe extern "C" {
    pub static mut stdin: *mut FILE;
}
unsafe extern "C" {
    pub static mut stdout: *mut FILE;
}
unsafe extern "C" {
    pub static mut stderr: *mut FILE;
}
unsafe extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
unsafe extern "C" {
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
unsafe extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
unsafe extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn gets(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
unsafe extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
unsafe extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
unsafe extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __s: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
unsafe extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
unsafe extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
unsafe extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
unsafe extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
unsafe extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::std::os::raw::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stat"][::std::mem::size_of::<stat>() - 144usize];
    ["Alignment of stat"][::std::mem::align_of::<stat>() - 8usize];
    ["Offset of field: stat::st_dev"][::std::mem::offset_of!(stat, st_dev) - 0usize];
    ["Offset of field: stat::st_ino"][::std::mem::offset_of!(stat, st_ino) - 8usize];
    ["Offset of field: stat::st_nlink"][::std::mem::offset_of!(stat, st_nlink) - 16usize];
    ["Offset of field: stat::st_mode"][::std::mem::offset_of!(stat, st_mode) - 24usize];
    ["Offset of field: stat::st_uid"][::std::mem::offset_of!(stat, st_uid) - 28usize];
    ["Offset of field: stat::st_gid"][::std::mem::offset_of!(stat, st_gid) - 32usize];
    ["Offset of field: stat::__pad0"][::std::mem::offset_of!(stat, __pad0) - 36usize];
    ["Offset of field: stat::st_rdev"][::std::mem::offset_of!(stat, st_rdev) - 40usize];
    ["Offset of field: stat::st_size"][::std::mem::offset_of!(stat, st_size) - 48usize];
    ["Offset of field: stat::st_blksize"][::std::mem::offset_of!(stat, st_blksize) - 56usize];
    ["Offset of field: stat::st_blocks"][::std::mem::offset_of!(stat, st_blocks) - 64usize];
    ["Offset of field: stat::st_atim"][::std::mem::offset_of!(stat, st_atim) - 72usize];
    ["Offset of field: stat::st_mtim"][::std::mem::offset_of!(stat, st_mtim) - 88usize];
    ["Offset of field: stat::st_ctim"][::std::mem::offset_of!(stat, st_ctim) - 104usize];
    ["Offset of field: stat::__glibc_reserved"]
        [::std::mem::offset_of!(stat, __glibc_reserved) - 120usize];
};
unsafe extern "C" {
    pub fn stat(__file: *const ::std::os::raw::c_char, __buf: *mut stat) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fstat(__fd: ::std::os::raw::c_int, __buf: *mut stat) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fstatat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __buf: *mut stat,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lstat(__file: *const ::std::os::raw::c_char, __buf: *mut stat) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn chmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lchmod(__file: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fchmod(__fd: ::std::os::raw::c_int, __mode: __mode_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fchmodat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn umask(__mask: __mode_t) -> __mode_t;
}
unsafe extern "C" {
    pub fn mkdir(__path: *const ::std::os::raw::c_char, __mode: __mode_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mkdirat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mknod(
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mknodat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
        __dev: __dev_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mkfifo(__path: *const ::std::os::raw::c_char, __mode: __mode_t)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mkfifoat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __mode: __mode_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn utimensat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __times: *const timespec,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn futimens(__fd: ::std::os::raw::c_int, __times: *const timespec)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mmap(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __prot: ::std::os::raw::c_int,
        __flags: ::std::os::raw::c_int,
        __fd: ::std::os::raw::c_int,
        __offset: __off_t,
    ) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn munmap(__addr: *mut ::std::os::raw::c_void, __len: usize) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mprotect(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __prot: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn msync(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn madvise(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __advice: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn posix_madvise(
        __addr: *mut ::std::os::raw::c_void,
        __len: usize,
        __advice: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mlock(__addr: *const ::std::os::raw::c_void, __len: usize) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn munlock(__addr: *const ::std::os::raw::c_void, __len: usize) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mlockall(__flags: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn munlockall() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn mincore(
        __start: *mut ::std::os::raw::c_void,
        __len: usize,
        __vec: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn shm_open(
        __name: *const ::std::os::raw::c_char,
        __oflag: ::std::os::raw::c_int,
        __mode: mode_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn shm_unlink(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: ::std::os::raw::c_int,
    pub tz_dsttime: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of timezone"][::std::mem::size_of::<timezone>() - 8usize];
    ["Alignment of timezone"][::std::mem::align_of::<timezone>() - 4usize];
    ["Offset of field: timezone::tz_minuteswest"]
        [::std::mem::offset_of!(timezone, tz_minuteswest) - 0usize];
    ["Offset of field: timezone::tz_dsttime"]
        [::std::mem::offset_of!(timezone, tz_dsttime) - 4usize];
};
unsafe extern "C" {
    pub fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> ::std::os::raw::c_int;
}
pub const __itimer_which_ITIMER_REAL: __itimer_which = 0;
pub const __itimer_which_ITIMER_VIRTUAL: __itimer_which = 1;
pub const __itimer_which_ITIMER_PROF: __itimer_which = 2;
pub type __itimer_which = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of itimerval"][::std::mem::size_of::<itimerval>() - 32usize];
    ["Alignment of itimerval"][::std::mem::align_of::<itimerval>() - 8usize];
    ["Offset of field: itimerval::it_interval"]
        [::std::mem::offset_of!(itimerval, it_interval) - 0usize];
    ["Offset of field: itimerval::it_value"][::std::mem::offset_of!(itimerval, it_value) - 16usize];
};
pub type __itimer_which_t = ::std::os::raw::c_int;
unsafe extern "C" {
    pub fn getitimer(__which: __itimer_which_t, __value: *mut itimerval) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn utimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lutimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn futimes(__fd: ::std::os::raw::c_int, __tvp: *const timeval) -> ::std::os::raw::c_int;
}
pub type sig_atomic_t = __sig_atomic_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigval"][::std::mem::size_of::<sigval>() - 8usize];
    ["Alignment of sigval"][::std::mem::align_of::<sigval>() - 8usize];
    ["Offset of field: sigval::sival_int"][::std::mem::offset_of!(sigval, sival_int) - 0usize];
    ["Offset of field: sigval::sival_ptr"][::std::mem::offset_of!(sigval, sival_ptr) - 0usize];
};
impl Default for sigval {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type __sigval_t = sigval;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub __pad0: ::std::os::raw::c_int,
    pub _sifields: siginfo_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union siginfo_t__bindgen_ty_1 {
    pub _pad: [::std::os::raw::c_int; 28usize],
    pub _kill: siginfo_t__bindgen_ty_1__bindgen_ty_1,
    pub _timer: siginfo_t__bindgen_ty_1__bindgen_ty_2,
    pub _rt: siginfo_t__bindgen_ty_1__bindgen_ty_3,
    pub _sigchld: siginfo_t__bindgen_ty_1__bindgen_ty_4,
    pub _sigfault: siginfo_t__bindgen_ty_1__bindgen_ty_5,
    pub _sigpoll: siginfo_t__bindgen_ty_1__bindgen_ty_6,
    pub _sigsys: siginfo_t__bindgen_ty_1__bindgen_ty_7,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_1 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_1>() - 8usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_1>() - 4usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_1::si_pid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_1, si_pid) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_1::si_uid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_1, si_uid) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_2 {
    pub si_tid: ::std::os::raw::c_int,
    pub si_overrun: ::std::os::raw::c_int,
    pub si_sigval: __sigval_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_2>() - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_2"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_2>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_2::si_tid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_2, si_tid) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_2::si_overrun"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_2, si_overrun) - 4usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_2::si_sigval"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_2, si_sigval) - 8usize];
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_2 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_3 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_3>() - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_3"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_3>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_3::si_pid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_3, si_pid) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_3::si_uid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_3, si_uid) - 4usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_3::si_sigval"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_3, si_sigval) - 8usize];
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_3 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_4 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_4"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_4>() - 32usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_4"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_4>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_4::si_pid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_4, si_pid) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_4::si_uid"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_4, si_uid) - 4usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_4::si_status"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_4, si_status) - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_4::si_utime"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_4, si_utime) - 16usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_4::si_stime"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_4, si_stime) - 24usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5 {
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_addr_lsb: ::std::os::raw::c_short,
    pub _bounds: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    pub _addr_bnd: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1,
    pub _pkey: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    pub _lower: *mut ::std::os::raw::c_void,
    pub _upper: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1>(
        ) - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1>(
        ) - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1::_lower"] [:: std :: mem :: offset_of ! (siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 , _lower) - 0usize] ;
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1::_upper"] [:: std :: mem :: offset_of ! (siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 , _upper) - 8usize] ;
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1>() - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1::_addr_bnd"][::std::mem::offset_of!(
        siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
        _addr_bnd
    ) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1::_pkey"][::std::mem::offset_of!(
        siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1,
        _pkey
    ) - 0usize];
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_5__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_5"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5>() - 32usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_5"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_5>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5::si_addr"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_5, si_addr) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5::si_addr_lsb"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_5, si_addr_lsb) - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_5::_bounds"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_5, _bounds) - 16usize];
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_5 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_6 {
    pub si_band: ::std::os::raw::c_long,
    pub si_fd: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_6"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_6>() - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_6"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_6>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_6::si_band"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_6, si_band) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_6::si_fd"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_6, si_fd) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct siginfo_t__bindgen_ty_1__bindgen_ty_7 {
    pub _call_addr: *mut ::std::os::raw::c_void,
    pub _syscall: ::std::os::raw::c_int,
    pub _arch: ::std::os::raw::c_uint,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1__bindgen_ty_7"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1__bindgen_ty_7>() - 16usize];
    ["Alignment of siginfo_t__bindgen_ty_1__bindgen_ty_7"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1__bindgen_ty_7>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_7::_call_addr"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_7, _call_addr) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_7::_syscall"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_7, _syscall) - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1__bindgen_ty_7::_arch"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1__bindgen_ty_7, _arch) - 12usize];
};
impl Default for siginfo_t__bindgen_ty_1__bindgen_ty_7 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t__bindgen_ty_1"]
        [::std::mem::size_of::<siginfo_t__bindgen_ty_1>() - 112usize];
    ["Alignment of siginfo_t__bindgen_ty_1"]
        [::std::mem::align_of::<siginfo_t__bindgen_ty_1>() - 8usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_pad"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _pad) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_kill"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _kill) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_timer"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _timer) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_rt"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _rt) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_sigchld"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _sigchld) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_sigfault"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _sigfault) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_sigpoll"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _sigpoll) - 0usize];
    ["Offset of field: siginfo_t__bindgen_ty_1::_sigsys"]
        [::std::mem::offset_of!(siginfo_t__bindgen_ty_1, _sigsys) - 0usize];
};
impl Default for siginfo_t__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of siginfo_t"][::std::mem::size_of::<siginfo_t>() - 128usize];
    ["Alignment of siginfo_t"][::std::mem::align_of::<siginfo_t>() - 8usize];
    ["Offset of field: siginfo_t::si_signo"][::std::mem::offset_of!(siginfo_t, si_signo) - 0usize];
    ["Offset of field: siginfo_t::si_errno"][::std::mem::offset_of!(siginfo_t, si_errno) - 4usize];
    ["Offset of field: siginfo_t::si_code"][::std::mem::offset_of!(siginfo_t, si_code) - 8usize];
    ["Offset of field: siginfo_t::__pad0"][::std::mem::offset_of!(siginfo_t, __pad0) - 12usize];
    ["Offset of field: siginfo_t::_sifields"]
        [::std::mem::offset_of!(siginfo_t, _sifields) - 16usize];
};
impl Default for siginfo_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const SI_ASYNCNL: _bindgen_ty_1 = -60;
pub const SI_DETHREAD: _bindgen_ty_1 = -7;
pub const SI_TKILL: _bindgen_ty_1 = -6;
pub const SI_SIGIO: _bindgen_ty_1 = -5;
pub const SI_ASYNCIO: _bindgen_ty_1 = -4;
pub const SI_MESGQ: _bindgen_ty_1 = -3;
pub const SI_TIMER: _bindgen_ty_1 = -2;
pub const SI_QUEUE: _bindgen_ty_1 = -1;
pub const SI_USER: _bindgen_ty_1 = 0;
pub const SI_KERNEL: _bindgen_ty_1 = 128;
pub type _bindgen_ty_1 = ::std::os::raw::c_int;
pub const ILL_ILLOPC: _bindgen_ty_2 = 1;
pub const ILL_ILLOPN: _bindgen_ty_2 = 2;
pub const ILL_ILLADR: _bindgen_ty_2 = 3;
pub const ILL_ILLTRP: _bindgen_ty_2 = 4;
pub const ILL_PRVOPC: _bindgen_ty_2 = 5;
pub const ILL_PRVREG: _bindgen_ty_2 = 6;
pub const ILL_COPROC: _bindgen_ty_2 = 7;
pub const ILL_BADSTK: _bindgen_ty_2 = 8;
pub const ILL_BADIADDR: _bindgen_ty_2 = 9;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const FPE_INTDIV: _bindgen_ty_3 = 1;
pub const FPE_INTOVF: _bindgen_ty_3 = 2;
pub const FPE_FLTDIV: _bindgen_ty_3 = 3;
pub const FPE_FLTOVF: _bindgen_ty_3 = 4;
pub const FPE_FLTUND: _bindgen_ty_3 = 5;
pub const FPE_FLTRES: _bindgen_ty_3 = 6;
pub const FPE_FLTINV: _bindgen_ty_3 = 7;
pub const FPE_FLTSUB: _bindgen_ty_3 = 8;
pub const FPE_FLTUNK: _bindgen_ty_3 = 14;
pub const FPE_CONDTRAP: _bindgen_ty_3 = 15;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const SEGV_MAPERR: _bindgen_ty_4 = 1;
pub const SEGV_ACCERR: _bindgen_ty_4 = 2;
pub const SEGV_BNDERR: _bindgen_ty_4 = 3;
pub const SEGV_PKUERR: _bindgen_ty_4 = 4;
pub const SEGV_ACCADI: _bindgen_ty_4 = 5;
pub const SEGV_ADIDERR: _bindgen_ty_4 = 6;
pub const SEGV_ADIPERR: _bindgen_ty_4 = 7;
pub const SEGV_MTEAERR: _bindgen_ty_4 = 8;
pub const SEGV_MTESERR: _bindgen_ty_4 = 9;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub const BUS_ADRALN: _bindgen_ty_5 = 1;
pub const BUS_ADRERR: _bindgen_ty_5 = 2;
pub const BUS_OBJERR: _bindgen_ty_5 = 3;
pub const BUS_MCEERR_AR: _bindgen_ty_5 = 4;
pub const BUS_MCEERR_AO: _bindgen_ty_5 = 5;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
pub const CLD_EXITED: _bindgen_ty_6 = 1;
pub const CLD_KILLED: _bindgen_ty_6 = 2;
pub const CLD_DUMPED: _bindgen_ty_6 = 3;
pub const CLD_TRAPPED: _bindgen_ty_6 = 4;
pub const CLD_STOPPED: _bindgen_ty_6 = 5;
pub const CLD_CONTINUED: _bindgen_ty_6 = 6;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
pub const POLL_IN: _bindgen_ty_7 = 1;
pub const POLL_OUT: _bindgen_ty_7 = 2;
pub const POLL_MSG: _bindgen_ty_7 = 3;
pub const POLL_ERR: _bindgen_ty_7 = 4;
pub const POLL_PRI: _bindgen_ty_7 = 5;
pub const POLL_HUP: _bindgen_ty_7 = 6;
pub type _bindgen_ty_7 = ::std::os::raw::c_uint;
pub type sigval_t = __sigval_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_value: __sigval_t,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_notify: ::std::os::raw::c_int,
    pub _sigev_un: sigevent__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigevent__bindgen_ty_1 {
    pub _pad: [::std::os::raw::c_int; 12usize],
    pub _tid: __pid_t,
    pub _sigev_thread: sigevent__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent__bindgen_ty_1__bindgen_ty_1 {
    pub _function: ::std::option::Option<unsafe extern "C" fn(arg1: __sigval_t)>,
    pub _attribute: *mut pthread_attr_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigevent__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::size_of::<sigevent__bindgen_ty_1__bindgen_ty_1>() - 16usize];
    ["Alignment of sigevent__bindgen_ty_1__bindgen_ty_1"]
        [::std::mem::align_of::<sigevent__bindgen_ty_1__bindgen_ty_1>() - 8usize];
    ["Offset of field: sigevent__bindgen_ty_1__bindgen_ty_1::_function"]
        [::std::mem::offset_of!(sigevent__bindgen_ty_1__bindgen_ty_1, _function) - 0usize];
    ["Offset of field: sigevent__bindgen_ty_1__bindgen_ty_1::_attribute"]
        [::std::mem::offset_of!(sigevent__bindgen_ty_1__bindgen_ty_1, _attribute) - 8usize];
};
impl Default for sigevent__bindgen_ty_1__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigevent__bindgen_ty_1"][::std::mem::size_of::<sigevent__bindgen_ty_1>() - 48usize];
    ["Alignment of sigevent__bindgen_ty_1"]
        [::std::mem::align_of::<sigevent__bindgen_ty_1>() - 8usize];
    ["Offset of field: sigevent__bindgen_ty_1::_pad"]
        [::std::mem::offset_of!(sigevent__bindgen_ty_1, _pad) - 0usize];
    ["Offset of field: sigevent__bindgen_ty_1::_tid"]
        [::std::mem::offset_of!(sigevent__bindgen_ty_1, _tid) - 0usize];
    ["Offset of field: sigevent__bindgen_ty_1::_sigev_thread"]
        [::std::mem::offset_of!(sigevent__bindgen_ty_1, _sigev_thread) - 0usize];
};
impl Default for sigevent__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigevent"][::std::mem::size_of::<sigevent>() - 64usize];
    ["Alignment of sigevent"][::std::mem::align_of::<sigevent>() - 8usize];
    ["Offset of field: sigevent::sigev_value"]
        [::std::mem::offset_of!(sigevent, sigev_value) - 0usize];
    ["Offset of field: sigevent::sigev_signo"]
        [::std::mem::offset_of!(sigevent, sigev_signo) - 8usize];
    ["Offset of field: sigevent::sigev_notify"]
        [::std::mem::offset_of!(sigevent, sigev_notify) - 12usize];
    ["Offset of field: sigevent::_sigev_un"][::std::mem::offset_of!(sigevent, _sigev_un) - 16usize];
};
impl Default for sigevent {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type sigevent_t = sigevent;
pub const SIGEV_SIGNAL: _bindgen_ty_8 = 0;
pub const SIGEV_NONE: _bindgen_ty_8 = 1;
pub const SIGEV_THREAD: _bindgen_ty_8 = 2;
pub const SIGEV_THREAD_ID: _bindgen_ty_8 = 4;
pub type _bindgen_ty_8 = ::std::os::raw::c_uint;
pub type __sighandler_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
unsafe extern "C" {
    pub fn __sysv_signal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t)
        -> __sighandler_t;
}
unsafe extern "C" {
    pub fn signal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
unsafe extern "C" {
    pub fn kill(__pid: __pid_t, __sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn killpg(__pgrp: __pid_t, __sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn raise(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ssignal(__sig: ::std::os::raw::c_int, __handler: __sighandler_t) -> __sighandler_t;
}
unsafe extern "C" {
    pub fn gsignal(__sig: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn psignal(__sig: ::std::os::raw::c_int, __s: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn psiginfo(__pinfo: *const siginfo_t, __s: *const ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn sigblock(__mask: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigsetmask(__mask: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn siggetmask() -> ::std::os::raw::c_int;
}
pub type sig_t = __sighandler_t;
unsafe extern "C" {
    pub fn sigemptyset(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigfillset(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigaddset(__set: *mut sigset_t, __signo: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigdelset(__set: *mut sigset_t, __signo: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigismember(
        __set: *const sigset_t,
        __signo: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_handler: sigaction__bindgen_ty_1,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
    pub sa_restorer: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigaction__bindgen_ty_1 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut siginfo_t,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigaction__bindgen_ty_1"][::std::mem::size_of::<sigaction__bindgen_ty_1>() - 8usize];
    ["Alignment of sigaction__bindgen_ty_1"]
        [::std::mem::align_of::<sigaction__bindgen_ty_1>() - 8usize];
    ["Offset of field: sigaction__bindgen_ty_1::sa_handler"]
        [::std::mem::offset_of!(sigaction__bindgen_ty_1, sa_handler) - 0usize];
    ["Offset of field: sigaction__bindgen_ty_1::sa_sigaction"]
        [::std::mem::offset_of!(sigaction__bindgen_ty_1, sa_sigaction) - 0usize];
};
impl Default for sigaction__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigaction"][::std::mem::size_of::<sigaction>() - 152usize];
    ["Alignment of sigaction"][::std::mem::align_of::<sigaction>() - 8usize];
    ["Offset of field: sigaction::__sigaction_handler"]
        [::std::mem::offset_of!(sigaction, __sigaction_handler) - 0usize];
    ["Offset of field: sigaction::sa_mask"][::std::mem::offset_of!(sigaction, sa_mask) - 8usize];
    ["Offset of field: sigaction::sa_flags"]
        [::std::mem::offset_of!(sigaction, sa_flags) - 136usize];
    ["Offset of field: sigaction::sa_restorer"]
        [::std::mem::offset_of!(sigaction, sa_restorer) - 144usize];
};
impl Default for sigaction {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn sigprocmask(
        __how: ::std::os::raw::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigsuspend(__set: *const sigset_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigaction(
        __sig: ::std::os::raw::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigpending(__set: *mut sigset_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigwait(
        __set: *const sigset_t,
        __sig: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigwaitinfo(__set: *const sigset_t, __info: *mut siginfo_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigtimedwait(
        __set: *const sigset_t,
        __info: *mut siginfo_t,
        __timeout: *const timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sigqueue(
        __pid: __pid_t,
        __sig: ::std::os::raw::c_int,
        __val: sigval,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _fpx_sw_bytes {
    pub magic1: __uint32_t,
    pub extended_size: __uint32_t,
    pub xstate_bv: __uint64_t,
    pub xstate_size: __uint32_t,
    pub __glibc_reserved1: [__uint32_t; 7usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _fpx_sw_bytes"][::std::mem::size_of::<_fpx_sw_bytes>() - 48usize];
    ["Alignment of _fpx_sw_bytes"][::std::mem::align_of::<_fpx_sw_bytes>() - 8usize];
    ["Offset of field: _fpx_sw_bytes::magic1"]
        [::std::mem::offset_of!(_fpx_sw_bytes, magic1) - 0usize];
    ["Offset of field: _fpx_sw_bytes::extended_size"]
        [::std::mem::offset_of!(_fpx_sw_bytes, extended_size) - 4usize];
    ["Offset of field: _fpx_sw_bytes::xstate_bv"]
        [::std::mem::offset_of!(_fpx_sw_bytes, xstate_bv) - 8usize];
    ["Offset of field: _fpx_sw_bytes::xstate_size"]
        [::std::mem::offset_of!(_fpx_sw_bytes, xstate_size) - 16usize];
    ["Offset of field: _fpx_sw_bytes::__glibc_reserved1"]
        [::std::mem::offset_of!(_fpx_sw_bytes, __glibc_reserved1) - 20usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _fpreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _fpreg"][::std::mem::size_of::<_fpreg>() - 10usize];
    ["Alignment of _fpreg"][::std::mem::align_of::<_fpreg>() - 2usize];
    ["Offset of field: _fpreg::significand"][::std::mem::offset_of!(_fpreg, significand) - 0usize];
    ["Offset of field: _fpreg::exponent"][::std::mem::offset_of!(_fpreg, exponent) - 8usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _fpxreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
    pub __glibc_reserved1: [::std::os::raw::c_ushort; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _fpxreg"][::std::mem::size_of::<_fpxreg>() - 16usize];
    ["Alignment of _fpxreg"][::std::mem::align_of::<_fpxreg>() - 2usize];
    ["Offset of field: _fpxreg::significand"]
        [::std::mem::offset_of!(_fpxreg, significand) - 0usize];
    ["Offset of field: _fpxreg::exponent"][::std::mem::offset_of!(_fpxreg, exponent) - 8usize];
    ["Offset of field: _fpxreg::__glibc_reserved1"]
        [::std::mem::offset_of!(_fpxreg, __glibc_reserved1) - 10usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _xmmreg {
    pub element: [__uint32_t; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _xmmreg"][::std::mem::size_of::<_xmmreg>() - 16usize];
    ["Alignment of _xmmreg"][::std::mem::align_of::<_xmmreg>() - 4usize];
    ["Offset of field: _xmmreg::element"][::std::mem::offset_of!(_xmmreg, element) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_fpxreg; 8usize],
    pub _xmm: [_xmmreg; 16usize],
    pub __glibc_reserved1: [__uint32_t; 24usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _fpstate"][::std::mem::size_of::<_fpstate>() - 512usize];
    ["Alignment of _fpstate"][::std::mem::align_of::<_fpstate>() - 8usize];
    ["Offset of field: _fpstate::cwd"][::std::mem::offset_of!(_fpstate, cwd) - 0usize];
    ["Offset of field: _fpstate::swd"][::std::mem::offset_of!(_fpstate, swd) - 2usize];
    ["Offset of field: _fpstate::ftw"][::std::mem::offset_of!(_fpstate, ftw) - 4usize];
    ["Offset of field: _fpstate::fop"][::std::mem::offset_of!(_fpstate, fop) - 6usize];
    ["Offset of field: _fpstate::rip"][::std::mem::offset_of!(_fpstate, rip) - 8usize];
    ["Offset of field: _fpstate::rdp"][::std::mem::offset_of!(_fpstate, rdp) - 16usize];
    ["Offset of field: _fpstate::mxcsr"][::std::mem::offset_of!(_fpstate, mxcsr) - 24usize];
    ["Offset of field: _fpstate::mxcr_mask"][::std::mem::offset_of!(_fpstate, mxcr_mask) - 28usize];
    ["Offset of field: _fpstate::_st"][::std::mem::offset_of!(_fpstate, _st) - 32usize];
    ["Offset of field: _fpstate::_xmm"][::std::mem::offset_of!(_fpstate, _xmm) - 160usize];
    ["Offset of field: _fpstate::__glibc_reserved1"]
        [::std::mem::offset_of!(_fpstate, __glibc_reserved1) - 416usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigcontext {
    pub r8: __uint64_t,
    pub r9: __uint64_t,
    pub r10: __uint64_t,
    pub r11: __uint64_t,
    pub r12: __uint64_t,
    pub r13: __uint64_t,
    pub r14: __uint64_t,
    pub r15: __uint64_t,
    pub rdi: __uint64_t,
    pub rsi: __uint64_t,
    pub rbp: __uint64_t,
    pub rbx: __uint64_t,
    pub rdx: __uint64_t,
    pub rax: __uint64_t,
    pub rcx: __uint64_t,
    pub rsp: __uint64_t,
    pub rip: __uint64_t,
    pub eflags: __uint64_t,
    pub cs: ::std::os::raw::c_ushort,
    pub gs: ::std::os::raw::c_ushort,
    pub fs: ::std::os::raw::c_ushort,
    pub __pad0: ::std::os::raw::c_ushort,
    pub err: __uint64_t,
    pub trapno: __uint64_t,
    pub oldmask: __uint64_t,
    pub cr2: __uint64_t,
    pub __bindgen_anon_1: sigcontext__bindgen_ty_1,
    pub __reserved1: [__uint64_t; 8usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigcontext__bindgen_ty_1 {
    pub fpstate: *mut _fpstate,
    pub __fpstate_word: __uint64_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigcontext__bindgen_ty_1"]
        [::std::mem::size_of::<sigcontext__bindgen_ty_1>() - 8usize];
    ["Alignment of sigcontext__bindgen_ty_1"]
        [::std::mem::align_of::<sigcontext__bindgen_ty_1>() - 8usize];
    ["Offset of field: sigcontext__bindgen_ty_1::fpstate"]
        [::std::mem::offset_of!(sigcontext__bindgen_ty_1, fpstate) - 0usize];
    ["Offset of field: sigcontext__bindgen_ty_1::__fpstate_word"]
        [::std::mem::offset_of!(sigcontext__bindgen_ty_1, __fpstate_word) - 0usize];
};
impl Default for sigcontext__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigcontext"][::std::mem::size_of::<sigcontext>() - 256usize];
    ["Alignment of sigcontext"][::std::mem::align_of::<sigcontext>() - 8usize];
    ["Offset of field: sigcontext::r8"][::std::mem::offset_of!(sigcontext, r8) - 0usize];
    ["Offset of field: sigcontext::r9"][::std::mem::offset_of!(sigcontext, r9) - 8usize];
    ["Offset of field: sigcontext::r10"][::std::mem::offset_of!(sigcontext, r10) - 16usize];
    ["Offset of field: sigcontext::r11"][::std::mem::offset_of!(sigcontext, r11) - 24usize];
    ["Offset of field: sigcontext::r12"][::std::mem::offset_of!(sigcontext, r12) - 32usize];
    ["Offset of field: sigcontext::r13"][::std::mem::offset_of!(sigcontext, r13) - 40usize];
    ["Offset of field: sigcontext::r14"][::std::mem::offset_of!(sigcontext, r14) - 48usize];
    ["Offset of field: sigcontext::r15"][::std::mem::offset_of!(sigcontext, r15) - 56usize];
    ["Offset of field: sigcontext::rdi"][::std::mem::offset_of!(sigcontext, rdi) - 64usize];
    ["Offset of field: sigcontext::rsi"][::std::mem::offset_of!(sigcontext, rsi) - 72usize];
    ["Offset of field: sigcontext::rbp"][::std::mem::offset_of!(sigcontext, rbp) - 80usize];
    ["Offset of field: sigcontext::rbx"][::std::mem::offset_of!(sigcontext, rbx) - 88usize];
    ["Offset of field: sigcontext::rdx"][::std::mem::offset_of!(sigcontext, rdx) - 96usize];
    ["Offset of field: sigcontext::rax"][::std::mem::offset_of!(sigcontext, rax) - 104usize];
    ["Offset of field: sigcontext::rcx"][::std::mem::offset_of!(sigcontext, rcx) - 112usize];
    ["Offset of field: sigcontext::rsp"][::std::mem::offset_of!(sigcontext, rsp) - 120usize];
    ["Offset of field: sigcontext::rip"][::std::mem::offset_of!(sigcontext, rip) - 128usize];
    ["Offset of field: sigcontext::eflags"][::std::mem::offset_of!(sigcontext, eflags) - 136usize];
    ["Offset of field: sigcontext::cs"][::std::mem::offset_of!(sigcontext, cs) - 144usize];
    ["Offset of field: sigcontext::gs"][::std::mem::offset_of!(sigcontext, gs) - 146usize];
    ["Offset of field: sigcontext::fs"][::std::mem::offset_of!(sigcontext, fs) - 148usize];
    ["Offset of field: sigcontext::__pad0"][::std::mem::offset_of!(sigcontext, __pad0) - 150usize];
    ["Offset of field: sigcontext::err"][::std::mem::offset_of!(sigcontext, err) - 152usize];
    ["Offset of field: sigcontext::trapno"][::std::mem::offset_of!(sigcontext, trapno) - 160usize];
    ["Offset of field: sigcontext::oldmask"]
        [::std::mem::offset_of!(sigcontext, oldmask) - 168usize];
    ["Offset of field: sigcontext::cr2"][::std::mem::offset_of!(sigcontext, cr2) - 176usize];
    ["Offset of field: sigcontext::__reserved1"]
        [::std::mem::offset_of!(sigcontext, __reserved1) - 192usize];
};
impl Default for sigcontext {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _xsave_hdr {
    pub xstate_bv: __uint64_t,
    pub __glibc_reserved1: [__uint64_t; 2usize],
    pub __glibc_reserved2: [__uint64_t; 5usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _xsave_hdr"][::std::mem::size_of::<_xsave_hdr>() - 64usize];
    ["Alignment of _xsave_hdr"][::std::mem::align_of::<_xsave_hdr>() - 8usize];
    ["Offset of field: _xsave_hdr::xstate_bv"]
        [::std::mem::offset_of!(_xsave_hdr, xstate_bv) - 0usize];
    ["Offset of field: _xsave_hdr::__glibc_reserved1"]
        [::std::mem::offset_of!(_xsave_hdr, __glibc_reserved1) - 8usize];
    ["Offset of field: _xsave_hdr::__glibc_reserved2"]
        [::std::mem::offset_of!(_xsave_hdr, __glibc_reserved2) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _ymmh_state {
    pub ymmh_space: [__uint32_t; 64usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _ymmh_state"][::std::mem::size_of::<_ymmh_state>() - 256usize];
    ["Alignment of _ymmh_state"][::std::mem::align_of::<_ymmh_state>() - 4usize];
    ["Offset of field: _ymmh_state::ymmh_space"]
        [::std::mem::offset_of!(_ymmh_state, ymmh_space) - 0usize];
};
impl Default for _ymmh_state {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _xstate {
    pub fpstate: _fpstate,
    pub xstate_hdr: _xsave_hdr,
    pub ymmh: _ymmh_state,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _xstate"][::std::mem::size_of::<_xstate>() - 832usize];
    ["Alignment of _xstate"][::std::mem::align_of::<_xstate>() - 8usize];
    ["Offset of field: _xstate::fpstate"][::std::mem::offset_of!(_xstate, fpstate) - 0usize];
    ["Offset of field: _xstate::xstate_hdr"]
        [::std::mem::offset_of!(_xstate, xstate_hdr) - 512usize];
    ["Offset of field: _xstate::ymmh"][::std::mem::offset_of!(_xstate, ymmh) - 576usize];
};
impl Default for _xstate {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn sigreturn(__scp: *mut sigcontext) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stack_t {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_flags: ::std::os::raw::c_int,
    pub ss_size: usize,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of stack_t"][::std::mem::size_of::<stack_t>() - 24usize];
    ["Alignment of stack_t"][::std::mem::align_of::<stack_t>() - 8usize];
    ["Offset of field: stack_t::ss_sp"][::std::mem::offset_of!(stack_t, ss_sp) - 0usize];
    ["Offset of field: stack_t::ss_flags"][::std::mem::offset_of!(stack_t, ss_flags) - 8usize];
    ["Offset of field: stack_t::ss_size"][::std::mem::offset_of!(stack_t, ss_size) - 16usize];
};
impl Default for stack_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type greg_t = ::std::os::raw::c_longlong;
pub type gregset_t = [greg_t; 23usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _libc_fpxreg {
    pub significand: [::std::os::raw::c_ushort; 4usize],
    pub exponent: ::std::os::raw::c_ushort,
    pub __glibc_reserved1: [::std::os::raw::c_ushort; 3usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _libc_fpxreg"][::std::mem::size_of::<_libc_fpxreg>() - 16usize];
    ["Alignment of _libc_fpxreg"][::std::mem::align_of::<_libc_fpxreg>() - 2usize];
    ["Offset of field: _libc_fpxreg::significand"]
        [::std::mem::offset_of!(_libc_fpxreg, significand) - 0usize];
    ["Offset of field: _libc_fpxreg::exponent"]
        [::std::mem::offset_of!(_libc_fpxreg, exponent) - 8usize];
    ["Offset of field: _libc_fpxreg::__glibc_reserved1"]
        [::std::mem::offset_of!(_libc_fpxreg, __glibc_reserved1) - 10usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _libc_xmmreg"][::std::mem::size_of::<_libc_xmmreg>() - 16usize];
    ["Alignment of _libc_xmmreg"][::std::mem::align_of::<_libc_xmmreg>() - 4usize];
    ["Offset of field: _libc_xmmreg::element"]
        [::std::mem::offset_of!(_libc_xmmreg, element) - 0usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8usize],
    pub _xmm: [_libc_xmmreg; 16usize],
    pub __glibc_reserved1: [__uint32_t; 24usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _libc_fpstate"][::std::mem::size_of::<_libc_fpstate>() - 512usize];
    ["Alignment of _libc_fpstate"][::std::mem::align_of::<_libc_fpstate>() - 8usize];
    ["Offset of field: _libc_fpstate::cwd"][::std::mem::offset_of!(_libc_fpstate, cwd) - 0usize];
    ["Offset of field: _libc_fpstate::swd"][::std::mem::offset_of!(_libc_fpstate, swd) - 2usize];
    ["Offset of field: _libc_fpstate::ftw"][::std::mem::offset_of!(_libc_fpstate, ftw) - 4usize];
    ["Offset of field: _libc_fpstate::fop"][::std::mem::offset_of!(_libc_fpstate, fop) - 6usize];
    ["Offset of field: _libc_fpstate::rip"][::std::mem::offset_of!(_libc_fpstate, rip) - 8usize];
    ["Offset of field: _libc_fpstate::rdp"][::std::mem::offset_of!(_libc_fpstate, rdp) - 16usize];
    ["Offset of field: _libc_fpstate::mxcsr"]
        [::std::mem::offset_of!(_libc_fpstate, mxcsr) - 24usize];
    ["Offset of field: _libc_fpstate::mxcr_mask"]
        [::std::mem::offset_of!(_libc_fpstate, mxcr_mask) - 28usize];
    ["Offset of field: _libc_fpstate::_st"][::std::mem::offset_of!(_libc_fpstate, _st) - 32usize];
    ["Offset of field: _libc_fpstate::_xmm"]
        [::std::mem::offset_of!(_libc_fpstate, _xmm) - 160usize];
    ["Offset of field: _libc_fpstate::__glibc_reserved1"]
        [::std::mem::offset_of!(_libc_fpstate, __glibc_reserved1) - 416usize];
};
pub type fpregset_t = *mut _libc_fpstate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [::std::os::raw::c_ulonglong; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of mcontext_t"][::std::mem::size_of::<mcontext_t>() - 256usize];
    ["Alignment of mcontext_t"][::std::mem::align_of::<mcontext_t>() - 8usize];
    ["Offset of field: mcontext_t::gregs"][::std::mem::offset_of!(mcontext_t, gregs) - 0usize];
    ["Offset of field: mcontext_t::fpregs"][::std::mem::offset_of!(mcontext_t, fpregs) - 184usize];
    ["Offset of field: mcontext_t::__reserved1"]
        [::std::mem::offset_of!(mcontext_t, __reserved1) - 192usize];
};
impl Default for mcontext_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ucontext_t {
    pub uc_flags: ::std::os::raw::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [::std::os::raw::c_ulonglong; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ucontext_t"][::std::mem::size_of::<ucontext_t>() - 968usize];
    ["Alignment of ucontext_t"][::std::mem::align_of::<ucontext_t>() - 8usize];
    ["Offset of field: ucontext_t::uc_flags"]
        [::std::mem::offset_of!(ucontext_t, uc_flags) - 0usize];
    ["Offset of field: ucontext_t::uc_link"][::std::mem::offset_of!(ucontext_t, uc_link) - 8usize];
    ["Offset of field: ucontext_t::uc_stack"]
        [::std::mem::offset_of!(ucontext_t, uc_stack) - 16usize];
    ["Offset of field: ucontext_t::uc_mcontext"]
        [::std::mem::offset_of!(ucontext_t, uc_mcontext) - 40usize];
    ["Offset of field: ucontext_t::uc_sigmask"]
        [::std::mem::offset_of!(ucontext_t, uc_sigmask) - 296usize];
    ["Offset of field: ucontext_t::__fpregs_mem"]
        [::std::mem::offset_of!(ucontext_t, __fpregs_mem) - 424usize];
    ["Offset of field: ucontext_t::__ssp"][::std::mem::offset_of!(ucontext_t, __ssp) - 936usize];
};
impl Default for ucontext_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn siginterrupt(
        __sig: ::std::os::raw::c_int,
        __interrupt: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const SS_ONSTACK: _bindgen_ty_9 = 1;
pub const SS_DISABLE: _bindgen_ty_9 = 2;
pub type _bindgen_ty_9 = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn sigaltstack(__ss: *const stack_t, __oss: *mut stack_t) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_onstack: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sigstack"][::std::mem::size_of::<sigstack>() - 16usize];
    ["Alignment of sigstack"][::std::mem::align_of::<sigstack>() - 8usize];
    ["Offset of field: sigstack::ss_sp"][::std::mem::offset_of!(sigstack, ss_sp) - 0usize];
    ["Offset of field: sigstack::ss_onstack"]
        [::std::mem::offset_of!(sigstack, ss_onstack) - 8usize];
};
impl Default for sigstack {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn sigstack(__ss: *mut sigstack, __oss: *mut sigstack) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_sigmask(
        __how: ::std::os::raw::c_int,
        __newmask: *const __sigset_t,
        __oldmask: *mut __sigset_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_kill(
        __threadid: pthread_t,
        __signo: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __libc_current_sigrtmin() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __libc_current_sigrtmax() -> ::std::os::raw::c_int;
}
pub type useconds_t = __useconds_t;
pub type socklen_t = __socklen_t;
unsafe extern "C" {
    pub fn access(
        __name: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn faccessat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __type: ::std::os::raw::c_int,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lseek(
        __fd: ::std::os::raw::c_int,
        __offset: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> __off_t;
}
unsafe extern "C" {
    pub fn close(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn closefrom(__lowfd: ::std::os::raw::c_int);
}
unsafe extern "C" {
    pub fn read(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
    ) -> isize;
}
unsafe extern "C" {
    pub fn write(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> isize;
}
unsafe extern "C" {
    pub fn pread(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_void,
        __nbytes: usize,
        __offset: __off_t,
    ) -> isize;
}
unsafe extern "C" {
    pub fn pwrite(
        __fd: ::std::os::raw::c_int,
        __buf: *const ::std::os::raw::c_void,
        __n: usize,
        __offset: __off_t,
    ) -> isize;
}
unsafe extern "C" {
    pub fn pipe(__pipedes: *mut ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn alarm(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn sleep(__seconds: ::std::os::raw::c_uint) -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn ualarm(__value: __useconds_t, __interval: __useconds_t) -> __useconds_t;
}
unsafe extern "C" {
    pub fn usleep(__useconds: __useconds_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pause() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn chown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fchown(
        __fd: ::std::os::raw::c_int,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn lchown(
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fchownat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __owner: __uid_t,
        __group: __gid_t,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn chdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fchdir(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getcwd(__buf: *mut ::std::os::raw::c_char, __size: usize)
        -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn getwd(__buf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn dup(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn dup2(__fd: ::std::os::raw::c_int, __fd2: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut __environ: *mut *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn execve(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fexecve(
        __fd: ::std::os::raw::c_int,
        __argv: *const *mut ::std::os::raw::c_char,
        __envp: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn execv(
        __path: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn execle(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn execl(
        __path: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn execvp(
        __file: *const ::std::os::raw::c_char,
        __argv: *const *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn execlp(
        __file: *const ::std::os::raw::c_char,
        __arg: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn nice(__inc: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn _exit(__status: ::std::os::raw::c_int) -> !;
}
pub const _PC_LINK_MAX: _bindgen_ty_10 = 0;
pub const _PC_MAX_CANON: _bindgen_ty_10 = 1;
pub const _PC_MAX_INPUT: _bindgen_ty_10 = 2;
pub const _PC_NAME_MAX: _bindgen_ty_10 = 3;
pub const _PC_PATH_MAX: _bindgen_ty_10 = 4;
pub const _PC_PIPE_BUF: _bindgen_ty_10 = 5;
pub const _PC_CHOWN_RESTRICTED: _bindgen_ty_10 = 6;
pub const _PC_NO_TRUNC: _bindgen_ty_10 = 7;
pub const _PC_VDISABLE: _bindgen_ty_10 = 8;
pub const _PC_SYNC_IO: _bindgen_ty_10 = 9;
pub const _PC_ASYNC_IO: _bindgen_ty_10 = 10;
pub const _PC_PRIO_IO: _bindgen_ty_10 = 11;
pub const _PC_SOCK_MAXBUF: _bindgen_ty_10 = 12;
pub const _PC_FILESIZEBITS: _bindgen_ty_10 = 13;
pub const _PC_REC_INCR_XFER_SIZE: _bindgen_ty_10 = 14;
pub const _PC_REC_MAX_XFER_SIZE: _bindgen_ty_10 = 15;
pub const _PC_REC_MIN_XFER_SIZE: _bindgen_ty_10 = 16;
pub const _PC_REC_XFER_ALIGN: _bindgen_ty_10 = 17;
pub const _PC_ALLOC_SIZE_MIN: _bindgen_ty_10 = 18;
pub const _PC_SYMLINK_MAX: _bindgen_ty_10 = 19;
pub const _PC_2_SYMLINKS: _bindgen_ty_10 = 20;
pub type _bindgen_ty_10 = ::std::os::raw::c_uint;
pub const _SC_ARG_MAX: _bindgen_ty_11 = 0;
pub const _SC_CHILD_MAX: _bindgen_ty_11 = 1;
pub const _SC_CLK_TCK: _bindgen_ty_11 = 2;
pub const _SC_NGROUPS_MAX: _bindgen_ty_11 = 3;
pub const _SC_OPEN_MAX: _bindgen_ty_11 = 4;
pub const _SC_STREAM_MAX: _bindgen_ty_11 = 5;
pub const _SC_TZNAME_MAX: _bindgen_ty_11 = 6;
pub const _SC_JOB_CONTROL: _bindgen_ty_11 = 7;
pub const _SC_SAVED_IDS: _bindgen_ty_11 = 8;
pub const _SC_REALTIME_SIGNALS: _bindgen_ty_11 = 9;
pub const _SC_PRIORITY_SCHEDULING: _bindgen_ty_11 = 10;
pub const _SC_TIMERS: _bindgen_ty_11 = 11;
pub const _SC_ASYNCHRONOUS_IO: _bindgen_ty_11 = 12;
pub const _SC_PRIORITIZED_IO: _bindgen_ty_11 = 13;
pub const _SC_SYNCHRONIZED_IO: _bindgen_ty_11 = 14;
pub const _SC_FSYNC: _bindgen_ty_11 = 15;
pub const _SC_MAPPED_FILES: _bindgen_ty_11 = 16;
pub const _SC_MEMLOCK: _bindgen_ty_11 = 17;
pub const _SC_MEMLOCK_RANGE: _bindgen_ty_11 = 18;
pub const _SC_MEMORY_PROTECTION: _bindgen_ty_11 = 19;
pub const _SC_MESSAGE_PASSING: _bindgen_ty_11 = 20;
pub const _SC_SEMAPHORES: _bindgen_ty_11 = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: _bindgen_ty_11 = 22;
pub const _SC_AIO_LISTIO_MAX: _bindgen_ty_11 = 23;
pub const _SC_AIO_MAX: _bindgen_ty_11 = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: _bindgen_ty_11 = 25;
pub const _SC_DELAYTIMER_MAX: _bindgen_ty_11 = 26;
pub const _SC_MQ_OPEN_MAX: _bindgen_ty_11 = 27;
pub const _SC_MQ_PRIO_MAX: _bindgen_ty_11 = 28;
pub const _SC_VERSION: _bindgen_ty_11 = 29;
pub const _SC_PAGESIZE: _bindgen_ty_11 = 30;
pub const _SC_RTSIG_MAX: _bindgen_ty_11 = 31;
pub const _SC_SEM_NSEMS_MAX: _bindgen_ty_11 = 32;
pub const _SC_SEM_VALUE_MAX: _bindgen_ty_11 = 33;
pub const _SC_SIGQUEUE_MAX: _bindgen_ty_11 = 34;
pub const _SC_TIMER_MAX: _bindgen_ty_11 = 35;
pub const _SC_BC_BASE_MAX: _bindgen_ty_11 = 36;
pub const _SC_BC_DIM_MAX: _bindgen_ty_11 = 37;
pub const _SC_BC_SCALE_MAX: _bindgen_ty_11 = 38;
pub const _SC_BC_STRING_MAX: _bindgen_ty_11 = 39;
pub const _SC_COLL_WEIGHTS_MAX: _bindgen_ty_11 = 40;
pub const _SC_EQUIV_CLASS_MAX: _bindgen_ty_11 = 41;
pub const _SC_EXPR_NEST_MAX: _bindgen_ty_11 = 42;
pub const _SC_LINE_MAX: _bindgen_ty_11 = 43;
pub const _SC_RE_DUP_MAX: _bindgen_ty_11 = 44;
pub const _SC_CHARCLASS_NAME_MAX: _bindgen_ty_11 = 45;
pub const _SC_2_VERSION: _bindgen_ty_11 = 46;
pub const _SC_2_C_BIND: _bindgen_ty_11 = 47;
pub const _SC_2_C_DEV: _bindgen_ty_11 = 48;
pub const _SC_2_FORT_DEV: _bindgen_ty_11 = 49;
pub const _SC_2_FORT_RUN: _bindgen_ty_11 = 50;
pub const _SC_2_SW_DEV: _bindgen_ty_11 = 51;
pub const _SC_2_LOCALEDEF: _bindgen_ty_11 = 52;
pub const _SC_PII: _bindgen_ty_11 = 53;
pub const _SC_PII_XTI: _bindgen_ty_11 = 54;
pub const _SC_PII_SOCKET: _bindgen_ty_11 = 55;
pub const _SC_PII_INTERNET: _bindgen_ty_11 = 56;
pub const _SC_PII_OSI: _bindgen_ty_11 = 57;
pub const _SC_POLL: _bindgen_ty_11 = 58;
pub const _SC_SELECT: _bindgen_ty_11 = 59;
pub const _SC_UIO_MAXIOV: _bindgen_ty_11 = 60;
pub const _SC_IOV_MAX: _bindgen_ty_11 = 60;
pub const _SC_PII_INTERNET_STREAM: _bindgen_ty_11 = 61;
pub const _SC_PII_INTERNET_DGRAM: _bindgen_ty_11 = 62;
pub const _SC_PII_OSI_COTS: _bindgen_ty_11 = 63;
pub const _SC_PII_OSI_CLTS: _bindgen_ty_11 = 64;
pub const _SC_PII_OSI_M: _bindgen_ty_11 = 65;
pub const _SC_T_IOV_MAX: _bindgen_ty_11 = 66;
pub const _SC_THREADS: _bindgen_ty_11 = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: _bindgen_ty_11 = 68;
pub const _SC_GETGR_R_SIZE_MAX: _bindgen_ty_11 = 69;
pub const _SC_GETPW_R_SIZE_MAX: _bindgen_ty_11 = 70;
pub const _SC_LOGIN_NAME_MAX: _bindgen_ty_11 = 71;
pub const _SC_TTY_NAME_MAX: _bindgen_ty_11 = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: _bindgen_ty_11 = 73;
pub const _SC_THREAD_KEYS_MAX: _bindgen_ty_11 = 74;
pub const _SC_THREAD_STACK_MIN: _bindgen_ty_11 = 75;
pub const _SC_THREAD_THREADS_MAX: _bindgen_ty_11 = 76;
pub const _SC_THREAD_ATTR_STACKADDR: _bindgen_ty_11 = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: _bindgen_ty_11 = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: _bindgen_ty_11 = 79;
pub const _SC_THREAD_PRIO_INHERIT: _bindgen_ty_11 = 80;
pub const _SC_THREAD_PRIO_PROTECT: _bindgen_ty_11 = 81;
pub const _SC_THREAD_PROCESS_SHARED: _bindgen_ty_11 = 82;
pub const _SC_NPROCESSORS_CONF: _bindgen_ty_11 = 83;
pub const _SC_NPROCESSORS_ONLN: _bindgen_ty_11 = 84;
pub const _SC_PHYS_PAGES: _bindgen_ty_11 = 85;
pub const _SC_AVPHYS_PAGES: _bindgen_ty_11 = 86;
pub const _SC_ATEXIT_MAX: _bindgen_ty_11 = 87;
pub const _SC_PASS_MAX: _bindgen_ty_11 = 88;
pub const _SC_XOPEN_VERSION: _bindgen_ty_11 = 89;
pub const _SC_XOPEN_XCU_VERSION: _bindgen_ty_11 = 90;
pub const _SC_XOPEN_UNIX: _bindgen_ty_11 = 91;
pub const _SC_XOPEN_CRYPT: _bindgen_ty_11 = 92;
pub const _SC_XOPEN_ENH_I18N: _bindgen_ty_11 = 93;
pub const _SC_XOPEN_SHM: _bindgen_ty_11 = 94;
pub const _SC_2_CHAR_TERM: _bindgen_ty_11 = 95;
pub const _SC_2_C_VERSION: _bindgen_ty_11 = 96;
pub const _SC_2_UPE: _bindgen_ty_11 = 97;
pub const _SC_XOPEN_XPG2: _bindgen_ty_11 = 98;
pub const _SC_XOPEN_XPG3: _bindgen_ty_11 = 99;
pub const _SC_XOPEN_XPG4: _bindgen_ty_11 = 100;
pub const _SC_CHAR_BIT: _bindgen_ty_11 = 101;
pub const _SC_CHAR_MAX: _bindgen_ty_11 = 102;
pub const _SC_CHAR_MIN: _bindgen_ty_11 = 103;
pub const _SC_INT_MAX: _bindgen_ty_11 = 104;
pub const _SC_INT_MIN: _bindgen_ty_11 = 105;
pub const _SC_LONG_BIT: _bindgen_ty_11 = 106;
pub const _SC_WORD_BIT: _bindgen_ty_11 = 107;
pub const _SC_MB_LEN_MAX: _bindgen_ty_11 = 108;
pub const _SC_NZERO: _bindgen_ty_11 = 109;
pub const _SC_SSIZE_MAX: _bindgen_ty_11 = 110;
pub const _SC_SCHAR_MAX: _bindgen_ty_11 = 111;
pub const _SC_SCHAR_MIN: _bindgen_ty_11 = 112;
pub const _SC_SHRT_MAX: _bindgen_ty_11 = 113;
pub const _SC_SHRT_MIN: _bindgen_ty_11 = 114;
pub const _SC_UCHAR_MAX: _bindgen_ty_11 = 115;
pub const _SC_UINT_MAX: _bindgen_ty_11 = 116;
pub const _SC_ULONG_MAX: _bindgen_ty_11 = 117;
pub const _SC_USHRT_MAX: _bindgen_ty_11 = 118;
pub const _SC_NL_ARGMAX: _bindgen_ty_11 = 119;
pub const _SC_NL_LANGMAX: _bindgen_ty_11 = 120;
pub const _SC_NL_MSGMAX: _bindgen_ty_11 = 121;
pub const _SC_NL_NMAX: _bindgen_ty_11 = 122;
pub const _SC_NL_SETMAX: _bindgen_ty_11 = 123;
pub const _SC_NL_TEXTMAX: _bindgen_ty_11 = 124;
pub const _SC_XBS5_ILP32_OFF32: _bindgen_ty_11 = 125;
pub const _SC_XBS5_ILP32_OFFBIG: _bindgen_ty_11 = 126;
pub const _SC_XBS5_LP64_OFF64: _bindgen_ty_11 = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: _bindgen_ty_11 = 128;
pub const _SC_XOPEN_LEGACY: _bindgen_ty_11 = 129;
pub const _SC_XOPEN_REALTIME: _bindgen_ty_11 = 130;
pub const _SC_XOPEN_REALTIME_THREADS: _bindgen_ty_11 = 131;
pub const _SC_ADVISORY_INFO: _bindgen_ty_11 = 132;
pub const _SC_BARRIERS: _bindgen_ty_11 = 133;
pub const _SC_BASE: _bindgen_ty_11 = 134;
pub const _SC_C_LANG_SUPPORT: _bindgen_ty_11 = 135;
pub const _SC_C_LANG_SUPPORT_R: _bindgen_ty_11 = 136;
pub const _SC_CLOCK_SELECTION: _bindgen_ty_11 = 137;
pub const _SC_CPUTIME: _bindgen_ty_11 = 138;
pub const _SC_THREAD_CPUTIME: _bindgen_ty_11 = 139;
pub const _SC_DEVICE_IO: _bindgen_ty_11 = 140;
pub const _SC_DEVICE_SPECIFIC: _bindgen_ty_11 = 141;
pub const _SC_DEVICE_SPECIFIC_R: _bindgen_ty_11 = 142;
pub const _SC_FD_MGMT: _bindgen_ty_11 = 143;
pub const _SC_FIFO: _bindgen_ty_11 = 144;
pub const _SC_PIPE: _bindgen_ty_11 = 145;
pub const _SC_FILE_ATTRIBUTES: _bindgen_ty_11 = 146;
pub const _SC_FILE_LOCKING: _bindgen_ty_11 = 147;
pub const _SC_FILE_SYSTEM: _bindgen_ty_11 = 148;
pub const _SC_MONOTONIC_CLOCK: _bindgen_ty_11 = 149;
pub const _SC_MULTI_PROCESS: _bindgen_ty_11 = 150;
pub const _SC_SINGLE_PROCESS: _bindgen_ty_11 = 151;
pub const _SC_NETWORKING: _bindgen_ty_11 = 152;
pub const _SC_READER_WRITER_LOCKS: _bindgen_ty_11 = 153;
pub const _SC_SPIN_LOCKS: _bindgen_ty_11 = 154;
pub const _SC_REGEXP: _bindgen_ty_11 = 155;
pub const _SC_REGEX_VERSION: _bindgen_ty_11 = 156;
pub const _SC_SHELL: _bindgen_ty_11 = 157;
pub const _SC_SIGNALS: _bindgen_ty_11 = 158;
pub const _SC_SPAWN: _bindgen_ty_11 = 159;
pub const _SC_SPORADIC_SERVER: _bindgen_ty_11 = 160;
pub const _SC_THREAD_SPORADIC_SERVER: _bindgen_ty_11 = 161;
pub const _SC_SYSTEM_DATABASE: _bindgen_ty_11 = 162;
pub const _SC_SYSTEM_DATABASE_R: _bindgen_ty_11 = 163;
pub const _SC_TIMEOUTS: _bindgen_ty_11 = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: _bindgen_ty_11 = 165;
pub const _SC_USER_GROUPS: _bindgen_ty_11 = 166;
pub const _SC_USER_GROUPS_R: _bindgen_ty_11 = 167;
pub const _SC_2_PBS: _bindgen_ty_11 = 168;
pub const _SC_2_PBS_ACCOUNTING: _bindgen_ty_11 = 169;
pub const _SC_2_PBS_LOCATE: _bindgen_ty_11 = 170;
pub const _SC_2_PBS_MESSAGE: _bindgen_ty_11 = 171;
pub const _SC_2_PBS_TRACK: _bindgen_ty_11 = 172;
pub const _SC_SYMLOOP_MAX: _bindgen_ty_11 = 173;
pub const _SC_STREAMS: _bindgen_ty_11 = 174;
pub const _SC_2_PBS_CHECKPOINT: _bindgen_ty_11 = 175;
pub const _SC_V6_ILP32_OFF32: _bindgen_ty_11 = 176;
pub const _SC_V6_ILP32_OFFBIG: _bindgen_ty_11 = 177;
pub const _SC_V6_LP64_OFF64: _bindgen_ty_11 = 178;
pub const _SC_V6_LPBIG_OFFBIG: _bindgen_ty_11 = 179;
pub const _SC_HOST_NAME_MAX: _bindgen_ty_11 = 180;
pub const _SC_TRACE: _bindgen_ty_11 = 181;
pub const _SC_TRACE_EVENT_FILTER: _bindgen_ty_11 = 182;
pub const _SC_TRACE_INHERIT: _bindgen_ty_11 = 183;
pub const _SC_TRACE_LOG: _bindgen_ty_11 = 184;
pub const _SC_LEVEL1_ICACHE_SIZE: _bindgen_ty_11 = 185;
pub const _SC_LEVEL1_ICACHE_ASSOC: _bindgen_ty_11 = 186;
pub const _SC_LEVEL1_ICACHE_LINESIZE: _bindgen_ty_11 = 187;
pub const _SC_LEVEL1_DCACHE_SIZE: _bindgen_ty_11 = 188;
pub const _SC_LEVEL1_DCACHE_ASSOC: _bindgen_ty_11 = 189;
pub const _SC_LEVEL1_DCACHE_LINESIZE: _bindgen_ty_11 = 190;
pub const _SC_LEVEL2_CACHE_SIZE: _bindgen_ty_11 = 191;
pub const _SC_LEVEL2_CACHE_ASSOC: _bindgen_ty_11 = 192;
pub const _SC_LEVEL2_CACHE_LINESIZE: _bindgen_ty_11 = 193;
pub const _SC_LEVEL3_CACHE_SIZE: _bindgen_ty_11 = 194;
pub const _SC_LEVEL3_CACHE_ASSOC: _bindgen_ty_11 = 195;
pub const _SC_LEVEL3_CACHE_LINESIZE: _bindgen_ty_11 = 196;
pub const _SC_LEVEL4_CACHE_SIZE: _bindgen_ty_11 = 197;
pub const _SC_LEVEL4_CACHE_ASSOC: _bindgen_ty_11 = 198;
pub const _SC_LEVEL4_CACHE_LINESIZE: _bindgen_ty_11 = 199;
pub const _SC_IPV6: _bindgen_ty_11 = 235;
pub const _SC_RAW_SOCKETS: _bindgen_ty_11 = 236;
pub const _SC_V7_ILP32_OFF32: _bindgen_ty_11 = 237;
pub const _SC_V7_ILP32_OFFBIG: _bindgen_ty_11 = 238;
pub const _SC_V7_LP64_OFF64: _bindgen_ty_11 = 239;
pub const _SC_V7_LPBIG_OFFBIG: _bindgen_ty_11 = 240;
pub const _SC_SS_REPL_MAX: _bindgen_ty_11 = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: _bindgen_ty_11 = 242;
pub const _SC_TRACE_NAME_MAX: _bindgen_ty_11 = 243;
pub const _SC_TRACE_SYS_MAX: _bindgen_ty_11 = 244;
pub const _SC_TRACE_USER_EVENT_MAX: _bindgen_ty_11 = 245;
pub const _SC_XOPEN_STREAMS: _bindgen_ty_11 = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: _bindgen_ty_11 = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: _bindgen_ty_11 = 248;
pub const _SC_MINSIGSTKSZ: _bindgen_ty_11 = 249;
pub const _SC_SIGSTKSZ: _bindgen_ty_11 = 250;
pub type _bindgen_ty_11 = ::std::os::raw::c_uint;
pub const _CS_PATH: _bindgen_ty_12 = 0;
pub const _CS_V6_WIDTH_RESTRICTED_ENVS: _bindgen_ty_12 = 1;
pub const _CS_GNU_LIBC_VERSION: _bindgen_ty_12 = 2;
pub const _CS_GNU_LIBPTHREAD_VERSION: _bindgen_ty_12 = 3;
pub const _CS_V5_WIDTH_RESTRICTED_ENVS: _bindgen_ty_12 = 4;
pub const _CS_V7_WIDTH_RESTRICTED_ENVS: _bindgen_ty_12 = 5;
pub const _CS_LFS_CFLAGS: _bindgen_ty_12 = 1000;
pub const _CS_LFS_LDFLAGS: _bindgen_ty_12 = 1001;
pub const _CS_LFS_LIBS: _bindgen_ty_12 = 1002;
pub const _CS_LFS_LINTFLAGS: _bindgen_ty_12 = 1003;
pub const _CS_LFS64_CFLAGS: _bindgen_ty_12 = 1004;
pub const _CS_LFS64_LDFLAGS: _bindgen_ty_12 = 1005;
pub const _CS_LFS64_LIBS: _bindgen_ty_12 = 1006;
pub const _CS_LFS64_LINTFLAGS: _bindgen_ty_12 = 1007;
pub const _CS_XBS5_ILP32_OFF32_CFLAGS: _bindgen_ty_12 = 1100;
pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: _bindgen_ty_12 = 1101;
pub const _CS_XBS5_ILP32_OFF32_LIBS: _bindgen_ty_12 = 1102;
pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: _bindgen_ty_12 = 1103;
pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: _bindgen_ty_12 = 1104;
pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1105;
pub const _CS_XBS5_ILP32_OFFBIG_LIBS: _bindgen_ty_12 = 1106;
pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1107;
pub const _CS_XBS5_LP64_OFF64_CFLAGS: _bindgen_ty_12 = 1108;
pub const _CS_XBS5_LP64_OFF64_LDFLAGS: _bindgen_ty_12 = 1109;
pub const _CS_XBS5_LP64_OFF64_LIBS: _bindgen_ty_12 = 1110;
pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: _bindgen_ty_12 = 1111;
pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_12 = 1112;
pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1113;
pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: _bindgen_ty_12 = 1114;
pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1115;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: _bindgen_ty_12 = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: _bindgen_ty_12 = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: _bindgen_ty_12 = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: _bindgen_ty_12 = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: _bindgen_ty_12 = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: _bindgen_ty_12 = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: _bindgen_ty_12 = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: _bindgen_ty_12 = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: _bindgen_ty_12 = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: _bindgen_ty_12 = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_12 = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: _bindgen_ty_12 = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: _bindgen_ty_12 = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: _bindgen_ty_12 = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: _bindgen_ty_12 = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: _bindgen_ty_12 = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: _bindgen_ty_12 = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: _bindgen_ty_12 = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: _bindgen_ty_12 = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: _bindgen_ty_12 = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: _bindgen_ty_12 = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: _bindgen_ty_12 = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: _bindgen_ty_12 = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: _bindgen_ty_12 = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: _bindgen_ty_12 = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: _bindgen_ty_12 = 1147;
pub const _CS_V6_ENV: _bindgen_ty_12 = 1148;
pub const _CS_V7_ENV: _bindgen_ty_12 = 1149;
pub type _bindgen_ty_12 = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn pathconf(
        __path: *const ::std::os::raw::c_char,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn fpathconf(
        __fd: ::std::os::raw::c_int,
        __name: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn sysconf(__name: ::std::os::raw::c_int) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn confstr(
        __name: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> usize;
}
unsafe extern "C" {
    pub fn getpid() -> __pid_t;
}
unsafe extern "C" {
    pub fn getppid() -> __pid_t;
}
unsafe extern "C" {
    pub fn getpgrp() -> __pid_t;
}
unsafe extern "C" {
    pub fn __getpgid(__pid: __pid_t) -> __pid_t;
}
unsafe extern "C" {
    pub fn getpgid(__pid: __pid_t) -> __pid_t;
}
unsafe extern "C" {
    pub fn setpgid(__pid: __pid_t, __pgid: __pid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setpgrp() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setsid() -> __pid_t;
}
unsafe extern "C" {
    pub fn getsid(__pid: __pid_t) -> __pid_t;
}
unsafe extern "C" {
    pub fn getuid() -> __uid_t;
}
unsafe extern "C" {
    pub fn geteuid() -> __uid_t;
}
unsafe extern "C" {
    pub fn getgid() -> __gid_t;
}
unsafe extern "C" {
    pub fn getegid() -> __gid_t;
}
unsafe extern "C" {
    pub fn getgroups(__size: ::std::os::raw::c_int, __list: *mut __gid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setreuid(__ruid: __uid_t, __euid: __uid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn seteuid(__uid: __uid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setgid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setregid(__rgid: __gid_t, __egid: __gid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setegid(__gid: __gid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fork() -> __pid_t;
}
unsafe extern "C" {
    pub fn vfork() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ttyname(__fd: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn ttyname_r(
        __fd: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn isatty(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ttyslot() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn link(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn linkat(
        __fromfd: ::std::os::raw::c_int,
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn symlink(
        __from: *const ::std::os::raw::c_char,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn readlink(
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
unsafe extern "C" {
    pub fn symlinkat(
        __from: *const ::std::os::raw::c_char,
        __tofd: ::std::os::raw::c_int,
        __to: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn readlinkat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> isize;
}
unsafe extern "C" {
    pub fn unlink(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn unlinkat(
        __fd: ::std::os::raw::c_int,
        __name: *const ::std::os::raw::c_char,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn rmdir(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn tcgetpgrp(__fd: ::std::os::raw::c_int) -> __pid_t;
}
unsafe extern "C" {
    pub fn tcsetpgrp(__fd: ::std::os::raw::c_int, __pgrp_id: __pid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getlogin() -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn getlogin_r(
        __name: *mut ::std::os::raw::c_char,
        __name_len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setlogin(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut optarg: *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static mut optind: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut opterr: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut optopt: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getopt(
        ___argc: ::std::os::raw::c_int,
        ___argv: *const *mut ::std::os::raw::c_char,
        __shortopts: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gethostname(__name: *mut ::std::os::raw::c_char, __len: usize) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sethostname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sethostid(__id: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getdomainname(
        __name: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn setdomainname(
        __name: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn vhangup() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn revoke(__file: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn profil(
        __sample_buffer: *mut ::std::os::raw::c_ushort,
        __size: usize,
        __offset: usize,
        __scale: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn acct(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getusershell() -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn endusershell();
}
unsafe extern "C" {
    pub fn setusershell();
}
unsafe extern "C" {
    pub fn daemon(
        __nochdir: ::std::os::raw::c_int,
        __noclose: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn chroot(__path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getpass(__prompt: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn fsync(__fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn gethostid() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn sync();
}
unsafe extern "C" {
    pub fn getpagesize() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn getdtablesize() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn truncate(
        __file: *const ::std::os::raw::c_char,
        __length: __off_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn ftruncate(__fd: ::std::os::raw::c_int, __length: __off_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn brk(__addr: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sbrk(__delta: isize) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn syscall(__sysno: ::std::os::raw::c_long, ...) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn lockf(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        __len: __off_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn fdatasync(__fildes: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn crypt(
        __key: *const ::std::os::raw::c_char,
        __salt: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn getentropy(
        __buffer: *mut ::std::os::raw::c_void,
        __length: usize,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct flock {
    pub l_type: ::std::os::raw::c_short,
    pub l_whence: ::std::os::raw::c_short,
    pub l_start: __off_t,
    pub l_len: __off_t,
    pub l_pid: __pid_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of flock"][::std::mem::size_of::<flock>() - 32usize];
    ["Alignment of flock"][::std::mem::align_of::<flock>() - 8usize];
    ["Offset of field: flock::l_type"][::std::mem::offset_of!(flock, l_type) - 0usize];
    ["Offset of field: flock::l_whence"][::std::mem::offset_of!(flock, l_whence) - 2usize];
    ["Offset of field: flock::l_start"][::std::mem::offset_of!(flock, l_start) - 8usize];
    ["Offset of field: flock::l_len"][::std::mem::offset_of!(flock, l_len) - 16usize];
    ["Offset of field: flock::l_pid"][::std::mem::offset_of!(flock, l_pid) - 24usize];
};
unsafe extern "C" {
    pub fn fcntl(
        __fd: ::std::os::raw::c_int,
        __cmd: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn open(
        __file: *const ::std::os::raw::c_char,
        __oflag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn openat(
        __fd: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __oflag: ::std::os::raw::c_int,
        ...
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn creat(__file: *const ::std::os::raw::c_char, __mode: mode_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn posix_fadvise(
        __fd: ::std::os::raw::c_int,
        __offset: off_t,
        __len: off_t,
        __advise: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn posix_fallocate(
        __fd: ::std::os::raw::c_int,
        __offset: off_t,
        __len: off_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn backtrace(
        __array: *mut *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn backtrace_symbols(
        __array: *const *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_int,
    ) -> *mut *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn backtrace_symbols_fd(
        __array: *const *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_int,
        __fd: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of tm"][::std::mem::size_of::<tm>() - 56usize];
    ["Alignment of tm"][::std::mem::align_of::<tm>() - 8usize];
    ["Offset of field: tm::tm_sec"][::std::mem::offset_of!(tm, tm_sec) - 0usize];
    ["Offset of field: tm::tm_min"][::std::mem::offset_of!(tm, tm_min) - 4usize];
    ["Offset of field: tm::tm_hour"][::std::mem::offset_of!(tm, tm_hour) - 8usize];
    ["Offset of field: tm::tm_mday"][::std::mem::offset_of!(tm, tm_mday) - 12usize];
    ["Offset of field: tm::tm_mon"][::std::mem::offset_of!(tm, tm_mon) - 16usize];
    ["Offset of field: tm::tm_year"][::std::mem::offset_of!(tm, tm_year) - 20usize];
    ["Offset of field: tm::tm_wday"][::std::mem::offset_of!(tm, tm_wday) - 24usize];
    ["Offset of field: tm::tm_yday"][::std::mem::offset_of!(tm, tm_yday) - 28usize];
    ["Offset of field: tm::tm_isdst"][::std::mem::offset_of!(tm, tm_isdst) - 32usize];
    ["Offset of field: tm::tm_gmtoff"][::std::mem::offset_of!(tm, tm_gmtoff) - 40usize];
    ["Offset of field: tm::tm_zone"][::std::mem::offset_of!(tm, tm_zone) - 48usize];
};
impl Default for tm {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of itimerspec"][::std::mem::size_of::<itimerspec>() - 32usize];
    ["Alignment of itimerspec"][::std::mem::align_of::<itimerspec>() - 8usize];
    ["Offset of field: itimerspec::it_interval"]
        [::std::mem::offset_of!(itimerspec, it_interval) - 0usize];
    ["Offset of field: itimerspec::it_value"]
        [::std::mem::offset_of!(itimerspec, it_value) - 16usize];
};
unsafe extern "C" {
    pub fn clock() -> clock_t;
}
unsafe extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
unsafe extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
unsafe extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
unsafe extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
unsafe extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
unsafe extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
unsafe extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
unsafe extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
unsafe extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
unsafe extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub static mut __tzname: [*mut ::std::os::raw::c_char; 2usize];
}
unsafe extern "C" {
    pub static mut __daylight: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut __timezone: ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub static mut tzname: [*mut ::std::os::raw::c_char; 2usize];
}
unsafe extern "C" {
    pub fn tzset();
}
unsafe extern "C" {
    pub static mut daylight: ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub static mut timezone: ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
unsafe extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
unsafe extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct sched_param {
    pub sched_priority: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of sched_param"][::std::mem::size_of::<sched_param>() - 4usize];
    ["Alignment of sched_param"][::std::mem::align_of::<sched_param>() - 4usize];
    ["Offset of field: sched_param::sched_priority"]
        [::std::mem::offset_of!(sched_param, sched_priority) - 0usize];
};
pub type __cpu_mask = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cpu_set_t"][::std::mem::size_of::<cpu_set_t>() - 128usize];
    ["Alignment of cpu_set_t"][::std::mem::align_of::<cpu_set_t>() - 8usize];
    ["Offset of field: cpu_set_t::__bits"][::std::mem::offset_of!(cpu_set_t, __bits) - 0usize];
};
unsafe extern "C" {
    pub fn __sched_cpucount(__setsize: usize, __setp: *const cpu_set_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __sched_cpualloc(__count: usize) -> *mut cpu_set_t;
}
unsafe extern "C" {
    pub fn __sched_cpufree(__set: *mut cpu_set_t);
}
unsafe extern "C" {
    pub fn sched_setparam(__pid: __pid_t, __param: *const sched_param) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_getparam(__pid: __pid_t, __param: *mut sched_param) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_setscheduler(
        __pid: __pid_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_getscheduler(__pid: __pid_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_yield() -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_get_priority_max(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_get_priority_min(__algorithm: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn sched_rr_get_interval(__pid: __pid_t, __t: *mut timespec) -> ::std::os::raw::c_int;
}
pub type __jmp_buf = [::std::os::raw::c_long; 8usize];
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
    pub __saved_mask: __sigset_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __jmp_buf_tag"][::std::mem::size_of::<__jmp_buf_tag>() - 200usize];
    ["Alignment of __jmp_buf_tag"][::std::mem::align_of::<__jmp_buf_tag>() - 8usize];
    ["Offset of field: __jmp_buf_tag::__jmpbuf"]
        [::std::mem::offset_of!(__jmp_buf_tag, __jmpbuf) - 0usize];
    ["Offset of field: __jmp_buf_tag::__mask_was_saved"]
        [::std::mem::offset_of!(__jmp_buf_tag, __mask_was_saved) - 64usize];
    ["Offset of field: __jmp_buf_tag::__saved_mask"]
        [::std::mem::offset_of!(__jmp_buf_tag, __saved_mask) - 72usize];
};
pub const PTHREAD_CREATE_JOINABLE: _bindgen_ty_13 = 0;
pub const PTHREAD_CREATE_DETACHED: _bindgen_ty_13 = 1;
pub type _bindgen_ty_13 = ::std::os::raw::c_uint;
pub const PTHREAD_MUTEX_TIMED_NP: _bindgen_ty_14 = 0;
pub const PTHREAD_MUTEX_RECURSIVE_NP: _bindgen_ty_14 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: _bindgen_ty_14 = 2;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: _bindgen_ty_14 = 3;
pub const PTHREAD_MUTEX_NORMAL: _bindgen_ty_14 = 0;
pub const PTHREAD_MUTEX_RECURSIVE: _bindgen_ty_14 = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: _bindgen_ty_14 = 2;
pub const PTHREAD_MUTEX_DEFAULT: _bindgen_ty_14 = 0;
pub type _bindgen_ty_14 = ::std::os::raw::c_uint;
pub const PTHREAD_MUTEX_STALLED: _bindgen_ty_15 = 0;
pub const PTHREAD_MUTEX_STALLED_NP: _bindgen_ty_15 = 0;
pub const PTHREAD_MUTEX_ROBUST: _bindgen_ty_15 = 1;
pub const PTHREAD_MUTEX_ROBUST_NP: _bindgen_ty_15 = 1;
pub type _bindgen_ty_15 = ::std::os::raw::c_uint;
pub const PTHREAD_PRIO_NONE: _bindgen_ty_16 = 0;
pub const PTHREAD_PRIO_INHERIT: _bindgen_ty_16 = 1;
pub const PTHREAD_PRIO_PROTECT: _bindgen_ty_16 = 2;
pub type _bindgen_ty_16 = ::std::os::raw::c_uint;
pub const PTHREAD_RWLOCK_PREFER_READER_NP: _bindgen_ty_17 = 0;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NP: _bindgen_ty_17 = 1;
pub const PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: _bindgen_ty_17 = 2;
pub const PTHREAD_RWLOCK_DEFAULT_NP: _bindgen_ty_17 = 0;
pub type _bindgen_ty_17 = ::std::os::raw::c_uint;
pub const PTHREAD_INHERIT_SCHED: _bindgen_ty_18 = 0;
pub const PTHREAD_EXPLICIT_SCHED: _bindgen_ty_18 = 1;
pub type _bindgen_ty_18 = ::std::os::raw::c_uint;
pub const PTHREAD_SCOPE_SYSTEM: _bindgen_ty_19 = 0;
pub const PTHREAD_SCOPE_PROCESS: _bindgen_ty_19 = 1;
pub type _bindgen_ty_19 = ::std::os::raw::c_uint;
pub const PTHREAD_PROCESS_PRIVATE: _bindgen_ty_20 = 0;
pub const PTHREAD_PROCESS_SHARED: _bindgen_ty_20 = 1;
pub type _bindgen_ty_20 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _pthread_cleanup_buffer {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __canceltype: ::std::os::raw::c_int,
    pub __prev: *mut _pthread_cleanup_buffer,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _pthread_cleanup_buffer"][::std::mem::size_of::<_pthread_cleanup_buffer>() - 32usize];
    ["Alignment of _pthread_cleanup_buffer"]
        [::std::mem::align_of::<_pthread_cleanup_buffer>() - 8usize];
    ["Offset of field: _pthread_cleanup_buffer::__routine"]
        [::std::mem::offset_of!(_pthread_cleanup_buffer, __routine) - 0usize];
    ["Offset of field: _pthread_cleanup_buffer::__arg"]
        [::std::mem::offset_of!(_pthread_cleanup_buffer, __arg) - 8usize];
    ["Offset of field: _pthread_cleanup_buffer::__canceltype"]
        [::std::mem::offset_of!(_pthread_cleanup_buffer, __canceltype) - 16usize];
    ["Offset of field: _pthread_cleanup_buffer::__prev"]
        [::std::mem::offset_of!(_pthread_cleanup_buffer, __prev) - 24usize];
};
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const PTHREAD_CANCEL_ENABLE: _bindgen_ty_21 = 0;
pub const PTHREAD_CANCEL_DISABLE: _bindgen_ty_21 = 1;
pub type _bindgen_ty_21 = ::std::os::raw::c_uint;
pub const PTHREAD_CANCEL_DEFERRED: _bindgen_ty_22 = 0;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: _bindgen_ty_22 = 1;
pub type _bindgen_ty_22 = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> *mut ::std::os::raw::c_void,
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_exit(__retval: *mut ::std::os::raw::c_void) -> !;
}
unsafe extern "C" {
    pub fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_detach(__th: pthread_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_self() -> pthread_t;
}
unsafe extern "C" {
    pub fn pthread_equal(__thread1: pthread_t, __thread2: pthread_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getdetachstate(
        __attr: *const pthread_attr_t,
        __detachstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setdetachstate(
        __attr: *mut pthread_attr_t,
        __detachstate: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getguardsize(
        __attr: *const pthread_attr_t,
        __guardsize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setguardsize(
        __attr: *mut pthread_attr_t,
        __guardsize: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getschedparam(
        __attr: *const pthread_attr_t,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setschedparam(
        __attr: *mut pthread_attr_t,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getschedpolicy(
        __attr: *const pthread_attr_t,
        __policy: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setschedpolicy(
        __attr: *mut pthread_attr_t,
        __policy: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getinheritsched(
        __attr: *const pthread_attr_t,
        __inherit: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setinheritsched(
        __attr: *mut pthread_attr_t,
        __inherit: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getscope(
        __attr: *const pthread_attr_t,
        __scope: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setscope(
        __attr: *mut pthread_attr_t,
        __scope: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getstackaddr(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setstackaddr(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getstacksize(
        __attr: *const pthread_attr_t,
        __stacksize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setstacksize(
        __attr: *mut pthread_attr_t,
        __stacksize: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_getstack(
        __attr: *const pthread_attr_t,
        __stackaddr: *mut *mut ::std::os::raw::c_void,
        __stacksize: *mut usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_attr_setstack(
        __attr: *mut pthread_attr_t,
        __stackaddr: *mut ::std::os::raw::c_void,
        __stacksize: usize,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_setschedparam(
        __target_thread: pthread_t,
        __policy: ::std::os::raw::c_int,
        __param: *const sched_param,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_getschedparam(
        __target_thread: pthread_t,
        __policy: *mut ::std::os::raw::c_int,
        __param: *mut sched_param,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_setschedprio(
        __target_thread: pthread_t,
        __prio: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_once(
        __once_control: *mut pthread_once_t,
        __init_routine: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_setcancelstate(
        __state: ::std::os::raw::c_int,
        __oldstate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_setcanceltype(
        __type: ::std::os::raw::c_int,
        __oldtype: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cancel(__th: pthread_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_testcancel();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: __jmp_buf,
    pub __mask_was_saved: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __cancel_jmp_buf_tag"][::std::mem::size_of::<__cancel_jmp_buf_tag>() - 72usize];
    ["Alignment of __cancel_jmp_buf_tag"][::std::mem::align_of::<__cancel_jmp_buf_tag>() - 8usize];
    ["Offset of field: __cancel_jmp_buf_tag::__cancel_jmp_buf"]
        [::std::mem::offset_of!(__cancel_jmp_buf_tag, __cancel_jmp_buf) - 0usize];
    ["Offset of field: __cancel_jmp_buf_tag::__mask_was_saved"]
        [::std::mem::offset_of!(__cancel_jmp_buf_tag, __mask_was_saved) - 64usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1usize],
    pub __pad: [*mut ::std::os::raw::c_void; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_unwind_buf_t"][::std::mem::size_of::<__pthread_unwind_buf_t>() - 104usize];
    ["Alignment of __pthread_unwind_buf_t"]
        [::std::mem::align_of::<__pthread_unwind_buf_t>() - 8usize];
    ["Offset of field: __pthread_unwind_buf_t::__cancel_jmp_buf"]
        [::std::mem::offset_of!(__pthread_unwind_buf_t, __cancel_jmp_buf) - 0usize];
    ["Offset of field: __pthread_unwind_buf_t::__pad"]
        [::std::mem::offset_of!(__pthread_unwind_buf_t, __pad) - 72usize];
};
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __cancel_arg: *mut ::std::os::raw::c_void,
    pub __do_it: ::std::os::raw::c_int,
    pub __cancel_type: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __pthread_cleanup_frame"][::std::mem::size_of::<__pthread_cleanup_frame>() - 24usize];
    ["Alignment of __pthread_cleanup_frame"]
        [::std::mem::align_of::<__pthread_cleanup_frame>() - 8usize];
    ["Offset of field: __pthread_cleanup_frame::__cancel_routine"]
        [::std::mem::offset_of!(__pthread_cleanup_frame, __cancel_routine) - 0usize];
    ["Offset of field: __pthread_cleanup_frame::__cancel_arg"]
        [::std::mem::offset_of!(__pthread_cleanup_frame, __cancel_arg) - 8usize];
    ["Offset of field: __pthread_cleanup_frame::__do_it"]
        [::std::mem::offset_of!(__pthread_cleanup_frame, __do_it) - 16usize];
    ["Offset of field: __pthread_cleanup_frame::__cancel_type"]
        [::std::mem::offset_of!(__pthread_cleanup_frame, __cancel_type) - 20usize];
};
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn __pthread_register_cancel(__buf: *mut __pthread_unwind_buf_t);
}
unsafe extern "C" {
    pub fn __pthread_unregister_cancel(__buf: *mut __pthread_unwind_buf_t);
}
unsafe extern "C" {
    pub fn __pthread_unwind_next(__buf: *mut __pthread_unwind_buf_t) -> !;
}
unsafe extern "C" {
    pub fn __sigsetjmp(
        __env: *mut __jmp_buf_tag,
        __savemask: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_timedlock(
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_getprioceiling(
        __mutex: *const pthread_mutex_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_setprioceiling(
        __mutex: *mut pthread_mutex_t,
        __prioceiling: ::std::os::raw::c_int,
        __old_ceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutex_consistent(__mutex: *mut pthread_mutex_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_init(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_destroy(__attr: *mut pthread_mutexattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_getpshared(
        __attr: *const pthread_mutexattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_setpshared(
        __attr: *mut pthread_mutexattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_gettype(
        __attr: *const pthread_mutexattr_t,
        __kind: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_settype(
        __attr: *mut pthread_mutexattr_t,
        __kind: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_getprotocol(
        __attr: *const pthread_mutexattr_t,
        __protocol: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_setprotocol(
        __attr: *mut pthread_mutexattr_t,
        __protocol: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_getprioceiling(
        __attr: *const pthread_mutexattr_t,
        __prioceiling: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_setprioceiling(
        __attr: *mut pthread_mutexattr_t,
        __prioceiling: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_getrobust(
        __attr: *const pthread_mutexattr_t,
        __robustness: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_mutexattr_setrobust(
        __attr: *mut pthread_mutexattr_t,
        __robustness: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_init(
        __rwlock: *mut pthread_rwlock_t,
        __attr: *const pthread_rwlockattr_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_destroy(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_rdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_tryrdlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_timedrdlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_wrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_trywrlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_timedwrlock(
        __rwlock: *mut pthread_rwlock_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlock_unlock(__rwlock: *mut pthread_rwlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_init(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_destroy(__attr: *mut pthread_rwlockattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_getpshared(
        __attr: *const pthread_rwlockattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_setpshared(
        __attr: *mut pthread_rwlockattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_getkind_np(
        __attr: *const pthread_rwlockattr_t,
        __pref: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_rwlockattr_setkind_np(
        __attr: *mut pthread_rwlockattr_t,
        __pref: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_init(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_destroy(__attr: *mut pthread_condattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_getpshared(
        __attr: *const pthread_condattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_setpshared(
        __attr: *mut pthread_condattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_getclock(
        __attr: *const pthread_condattr_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_condattr_setclock(
        __attr: *mut pthread_condattr_t,
        __clock_id: __clockid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_spin_init(
        __lock: *mut pthread_spinlock_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_spin_destroy(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_spin_lock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_spin_trylock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_spin_unlock(__lock: *mut pthread_spinlock_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrier_init(
        __barrier: *mut pthread_barrier_t,
        __attr: *const pthread_barrierattr_t,
        __count: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrier_destroy(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrier_wait(__barrier: *mut pthread_barrier_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrierattr_init(__attr: *mut pthread_barrierattr_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrierattr_destroy(__attr: *mut pthread_barrierattr_t)
        -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrierattr_getpshared(
        __attr: *const pthread_barrierattr_t,
        __pshared: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_barrierattr_setpshared(
        __attr: *mut pthread_barrierattr_t,
        __pshared: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_key_create(
        __key: *mut pthread_key_t,
        __destr_function: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void),
        >,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_key_delete(__key: pthread_key_t) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_getspecific(__key: pthread_key_t) -> *mut ::std::os::raw::c_void;
}
unsafe extern "C" {
    pub fn pthread_setspecific(
        __key: pthread_key_t,
        __pointer: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_getcpuclockid(
        __thread_id: pthread_t,
        __clock_id: *mut __clockid_t,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn pthread_atfork(
        __prepare: ::std::option::Option<unsafe extern "C" fn()>,
        __parent: ::std::option::Option<unsafe extern "C" fn()>,
        __child: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __assert_fail(
        __assertion: *const ::std::os::raw::c_char,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_uint,
        __function: *const ::std::os::raw::c_char,
    ) -> !;
}
unsafe extern "C" {
    pub fn __assert_perror_fail(
        __errnum: ::std::os::raw::c_int,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_uint,
        __function: *const ::std::os::raw::c_char,
    ) -> !;
}
unsafe extern "C" {
    pub fn __assert(
        __assertion: *const ::std::os::raw::c_char,
        __file: *const ::std::os::raw::c_char,
        __line: ::std::os::raw::c_int,
    ) -> !;
}
pub type u1i = u8;
pub type u2i = u16;
pub type u4i = u32;
pub type u8i = ::std::os::raw::c_ulonglong;
pub type b1i = i8;
pub type b2i = i16;
pub type b4i = i32;
pub type b8i = ::std::os::raw::c_longlong;
pub type f4i = f32;
pub type f8i = u128;
pub const _DEBUG_LOG_: ::std::os::raw::c_int = 0;
pub const _DEBUG_VAR_: u8i = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _proc_deamon_struct {
    pub _proc_deamon_params: *mut _proc_deamon_struct,
    pub _proc_deamon_array: *mut _proc_deamon_struct,
    pub n_cpu: ::std::os::raw::c_int,
    pub t_idx: ::std::os::raw::c_int,
    pub running: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_int,
    pub once: ::std::os::raw::c_int,
    pub mutex_lock: *mut pthread_mutex_t,
    pub rw_lock: *mut pthread_rwlock_t,
    pub _COND_LOCK: pthread_mutex_t,
    pub _COND: pthread_cond_t,
    pub memtotal: u8i,
    pub memavail: u8i,
    pub ncpu: ::std::os::raw::c_int,
    pub max_rss: u8i,
    pub max_vsz: u8i,
    pub rss_limit: u8i,
    pub utime: f64,
    pub stime: f64,
    pub rtime: f64,
    pub rtime_limit: f64,
    pub interval: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _proc_deamon_struct"][::std::mem::size_of::<_proc_deamon_struct>() - 232usize];
    ["Alignment of _proc_deamon_struct"][::std::mem::align_of::<_proc_deamon_struct>() - 8usize];
    ["Offset of field: _proc_deamon_struct::_proc_deamon_params"]
        [::std::mem::offset_of!(_proc_deamon_struct, _proc_deamon_params) - 0usize];
    ["Offset of field: _proc_deamon_struct::_proc_deamon_array"]
        [::std::mem::offset_of!(_proc_deamon_struct, _proc_deamon_array) - 8usize];
    ["Offset of field: _proc_deamon_struct::n_cpu"]
        [::std::mem::offset_of!(_proc_deamon_struct, n_cpu) - 16usize];
    ["Offset of field: _proc_deamon_struct::t_idx"]
        [::std::mem::offset_of!(_proc_deamon_struct, t_idx) - 20usize];
    ["Offset of field: _proc_deamon_struct::running"]
        [::std::mem::offset_of!(_proc_deamon_struct, running) - 24usize];
    ["Offset of field: _proc_deamon_struct::state"]
        [::std::mem::offset_of!(_proc_deamon_struct, state) - 28usize];
    ["Offset of field: _proc_deamon_struct::once"]
        [::std::mem::offset_of!(_proc_deamon_struct, once) - 32usize];
    ["Offset of field: _proc_deamon_struct::mutex_lock"]
        [::std::mem::offset_of!(_proc_deamon_struct, mutex_lock) - 40usize];
    ["Offset of field: _proc_deamon_struct::rw_lock"]
        [::std::mem::offset_of!(_proc_deamon_struct, rw_lock) - 48usize];
    ["Offset of field: _proc_deamon_struct::_COND_LOCK"]
        [::std::mem::offset_of!(_proc_deamon_struct, _COND_LOCK) - 56usize];
    ["Offset of field: _proc_deamon_struct::_COND"]
        [::std::mem::offset_of!(_proc_deamon_struct, _COND) - 96usize];
    ["Offset of field: _proc_deamon_struct::memtotal"]
        [::std::mem::offset_of!(_proc_deamon_struct, memtotal) - 144usize];
    ["Offset of field: _proc_deamon_struct::memavail"]
        [::std::mem::offset_of!(_proc_deamon_struct, memavail) - 152usize];
    ["Offset of field: _proc_deamon_struct::ncpu"]
        [::std::mem::offset_of!(_proc_deamon_struct, ncpu) - 160usize];
    ["Offset of field: _proc_deamon_struct::max_rss"]
        [::std::mem::offset_of!(_proc_deamon_struct, max_rss) - 168usize];
    ["Offset of field: _proc_deamon_struct::max_vsz"]
        [::std::mem::offset_of!(_proc_deamon_struct, max_vsz) - 176usize];
    ["Offset of field: _proc_deamon_struct::rss_limit"]
        [::std::mem::offset_of!(_proc_deamon_struct, rss_limit) - 184usize];
    ["Offset of field: _proc_deamon_struct::utime"]
        [::std::mem::offset_of!(_proc_deamon_struct, utime) - 192usize];
    ["Offset of field: _proc_deamon_struct::stime"]
        [::std::mem::offset_of!(_proc_deamon_struct, stime) - 200usize];
    ["Offset of field: _proc_deamon_struct::rtime"]
        [::std::mem::offset_of!(_proc_deamon_struct, rtime) - 208usize];
    ["Offset of field: _proc_deamon_struct::rtime_limit"]
        [::std::mem::offset_of!(_proc_deamon_struct, rtime_limit) - 216usize];
    ["Offset of field: _proc_deamon_struct::interval"]
        [::std::mem::offset_of!(_proc_deamon_struct, interval) - 224usize];
};
impl Default for _proc_deamon_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static mut _sig_proc_deamon: *mut _proc_deamon_struct;
}
pub type mem_array_count = ::std::option::Option<
    unsafe extern "C" fn(obj: *mut ::std::os::raw::c_void, idx: ::std::os::raw::c_int) -> usize,
>;
pub type mem_load_post =
    ::std::option::Option<unsafe extern "C" fn(obj: *mut ::std::os::raw::c_void, aux_data: usize)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obj_desc_t {
    pub tag: *const ::std::os::raw::c_char,
    pub size: usize,
    pub n_child: ::std::os::raw::c_int,
    pub mem_type: [u8; 64usize],
    pub addr: [off_t; 64usize],
    pub desc: [*const obj_desc_t; 64usize],
    pub cnt: mem_array_count,
    pub post: mem_load_post,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of obj_desc_t"][::std::mem::size_of::<obj_desc_t>() - 1128usize];
    ["Alignment of obj_desc_t"][::std::mem::align_of::<obj_desc_t>() - 8usize];
    ["Offset of field: obj_desc_t::tag"][::std::mem::offset_of!(obj_desc_t, tag) - 0usize];
    ["Offset of field: obj_desc_t::size"][::std::mem::offset_of!(obj_desc_t, size) - 8usize];
    ["Offset of field: obj_desc_t::n_child"][::std::mem::offset_of!(obj_desc_t, n_child) - 16usize];
    ["Offset of field: obj_desc_t::mem_type"]
        [::std::mem::offset_of!(obj_desc_t, mem_type) - 20usize];
    ["Offset of field: obj_desc_t::addr"][::std::mem::offset_of!(obj_desc_t, addr) - 88usize];
    ["Offset of field: obj_desc_t::desc"][::std::mem::offset_of!(obj_desc_t, desc) - 600usize];
    ["Offset of field: obj_desc_t::cnt"][::std::mem::offset_of!(obj_desc_t, cnt) - 1112usize];
    ["Offset of field: obj_desc_t::post"][::std::mem::offset_of!(obj_desc_t, post) - 1120usize];
};
impl Default for obj_desc_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static OBJ_DESC_DATA: obj_desc_t;
}
unsafe extern "C" {
    pub static OBJ_DESC_CHAR_ARRAY: obj_desc_t;
}
unsafe extern "C" {
    pub static mut mem_share_locks: *mut ::std::os::raw::c_char;
}
pub const mem_share_lock_size: ::std::os::raw::c_int = 0;
pub type sighandler_t = ::std::option::Option<unsafe extern "C" fn(sig: ::std::os::raw::c_int)>;
unsafe extern "C" {
    pub static mut sig_term: sighandler_t;
}
unsafe extern "C" {
    pub static mut sig_int: sighandler_t;
}
unsafe extern "C" {
    pub static mut sig_hup: sighandler_t;
}
unsafe extern "C" {
    pub static mut sig_kill: sighandler_t;
}
pub const cleanup_mem_share_in_progress: sig_atomic_t = 0;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u8list {
    pub buffer: *mut u1i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u8list"][::std::mem::size_of::<u8list>() - 32usize];
    ["Alignment of u8list"][::std::mem::align_of::<u8list>() - 8usize];
    ["Offset of field: u8list::buffer"][::std::mem::offset_of!(u8list, buffer) - 0usize];
    ["Offset of field: u8list::size"][::std::mem::offset_of!(u8list, size) - 8usize];
    ["Offset of field: u8list::cap"][::std::mem::offset_of!(u8list, cap) - 16usize];
};
impl Default for u8list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u8list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u8list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u1v {
    pub buffer: *mut u1i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u1v"][::std::mem::size_of::<u1v>() - 32usize];
    ["Alignment of u1v"][::std::mem::align_of::<u1v>() - 8usize];
    ["Offset of field: u1v::buffer"][::std::mem::offset_of!(u1v, buffer) - 0usize];
    ["Offset of field: u1v::size"][::std::mem::offset_of!(u1v, size) - 8usize];
    ["Offset of field: u1v::cap"][::std::mem::offset_of!(u1v, cap) - 16usize];
};
impl Default for u1v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u1v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u1v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u16list {
    pub buffer: *mut u2i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u16list"][::std::mem::size_of::<u16list>() - 32usize];
    ["Alignment of u16list"][::std::mem::align_of::<u16list>() - 8usize];
    ["Offset of field: u16list::buffer"][::std::mem::offset_of!(u16list, buffer) - 0usize];
    ["Offset of field: u16list::size"][::std::mem::offset_of!(u16list, size) - 8usize];
    ["Offset of field: u16list::cap"][::std::mem::offset_of!(u16list, cap) - 16usize];
};
impl Default for u16list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u16list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u16list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u2v {
    pub buffer: *mut u2i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u2v"][::std::mem::size_of::<u2v>() - 32usize];
    ["Alignment of u2v"][::std::mem::align_of::<u2v>() - 8usize];
    ["Offset of field: u2v::buffer"][::std::mem::offset_of!(u2v, buffer) - 0usize];
    ["Offset of field: u2v::size"][::std::mem::offset_of!(u2v, size) - 8usize];
    ["Offset of field: u2v::cap"][::std::mem::offset_of!(u2v, cap) - 16usize];
};
impl Default for u2v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u2v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u2v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u32list {
    pub buffer: *mut u4i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u32list"][::std::mem::size_of::<u32list>() - 32usize];
    ["Alignment of u32list"][::std::mem::align_of::<u32list>() - 8usize];
    ["Offset of field: u32list::buffer"][::std::mem::offset_of!(u32list, buffer) - 0usize];
    ["Offset of field: u32list::size"][::std::mem::offset_of!(u32list, size) - 8usize];
    ["Offset of field: u32list::cap"][::std::mem::offset_of!(u32list, cap) - 16usize];
};
impl Default for u32list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u32list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u32list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u4v {
    pub buffer: *mut u4i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u4v"][::std::mem::size_of::<u4v>() - 32usize];
    ["Alignment of u4v"][::std::mem::align_of::<u4v>() - 8usize];
    ["Offset of field: u4v::buffer"][::std::mem::offset_of!(u4v, buffer) - 0usize];
    ["Offset of field: u4v::size"][::std::mem::offset_of!(u4v, size) - 8usize];
    ["Offset of field: u4v::cap"][::std::mem::offset_of!(u4v, cap) - 16usize];
};
impl Default for u4v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u4v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u4v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u64list {
    pub buffer: *mut u8i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u64list"][::std::mem::size_of::<u64list>() - 32usize];
    ["Alignment of u64list"][::std::mem::align_of::<u64list>() - 8usize];
    ["Offset of field: u64list::buffer"][::std::mem::offset_of!(u64list, buffer) - 0usize];
    ["Offset of field: u64list::size"][::std::mem::offset_of!(u64list, size) - 8usize];
    ["Offset of field: u64list::cap"][::std::mem::offset_of!(u64list, cap) - 16usize];
};
impl Default for u64list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u64list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u64list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u8v {
    pub buffer: *mut u8i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u8v"][::std::mem::size_of::<u8v>() - 32usize];
    ["Alignment of u8v"][::std::mem::align_of::<u8v>() - 8usize];
    ["Offset of field: u8v::buffer"][::std::mem::offset_of!(u8v, buffer) - 0usize];
    ["Offset of field: u8v::size"][::std::mem::offset_of!(u8v, size) - 8usize];
    ["Offset of field: u8v::cap"][::std::mem::offset_of!(u8v, cap) - 16usize];
};
impl Default for u8v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl u8v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static u8v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b8list {
    pub buffer: *mut b1i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b8list"][::std::mem::size_of::<b8list>() - 32usize];
    ["Alignment of b8list"][::std::mem::align_of::<b8list>() - 8usize];
    ["Offset of field: b8list::buffer"][::std::mem::offset_of!(b8list, buffer) - 0usize];
    ["Offset of field: b8list::size"][::std::mem::offset_of!(b8list, size) - 8usize];
    ["Offset of field: b8list::cap"][::std::mem::offset_of!(b8list, cap) - 16usize];
};
impl Default for b8list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b8list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b8list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b1v {
    pub buffer: *mut b1i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b1v"][::std::mem::size_of::<b1v>() - 32usize];
    ["Alignment of b1v"][::std::mem::align_of::<b1v>() - 8usize];
    ["Offset of field: b1v::buffer"][::std::mem::offset_of!(b1v, buffer) - 0usize];
    ["Offset of field: b1v::size"][::std::mem::offset_of!(b1v, size) - 8usize];
    ["Offset of field: b1v::cap"][::std::mem::offset_of!(b1v, cap) - 16usize];
};
impl Default for b1v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b1v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b1v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b16list {
    pub buffer: *mut b2i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b16list"][::std::mem::size_of::<b16list>() - 32usize];
    ["Alignment of b16list"][::std::mem::align_of::<b16list>() - 8usize];
    ["Offset of field: b16list::buffer"][::std::mem::offset_of!(b16list, buffer) - 0usize];
    ["Offset of field: b16list::size"][::std::mem::offset_of!(b16list, size) - 8usize];
    ["Offset of field: b16list::cap"][::std::mem::offset_of!(b16list, cap) - 16usize];
};
impl Default for b16list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b16list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b16list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b2v {
    pub buffer: *mut b2i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b2v"][::std::mem::size_of::<b2v>() - 32usize];
    ["Alignment of b2v"][::std::mem::align_of::<b2v>() - 8usize];
    ["Offset of field: b2v::buffer"][::std::mem::offset_of!(b2v, buffer) - 0usize];
    ["Offset of field: b2v::size"][::std::mem::offset_of!(b2v, size) - 8usize];
    ["Offset of field: b2v::cap"][::std::mem::offset_of!(b2v, cap) - 16usize];
};
impl Default for b2v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b2v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b2v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b32list {
    pub buffer: *mut b4i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b32list"][::std::mem::size_of::<b32list>() - 32usize];
    ["Alignment of b32list"][::std::mem::align_of::<b32list>() - 8usize];
    ["Offset of field: b32list::buffer"][::std::mem::offset_of!(b32list, buffer) - 0usize];
    ["Offset of field: b32list::size"][::std::mem::offset_of!(b32list, size) - 8usize];
    ["Offset of field: b32list::cap"][::std::mem::offset_of!(b32list, cap) - 16usize];
};
impl Default for b32list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b32list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b32list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b4v {
    pub buffer: *mut b4i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b4v"][::std::mem::size_of::<b4v>() - 32usize];
    ["Alignment of b4v"][::std::mem::align_of::<b4v>() - 8usize];
    ["Offset of field: b4v::buffer"][::std::mem::offset_of!(b4v, buffer) - 0usize];
    ["Offset of field: b4v::size"][::std::mem::offset_of!(b4v, size) - 8usize];
    ["Offset of field: b4v::cap"][::std::mem::offset_of!(b4v, cap) - 16usize];
};
impl Default for b4v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b4v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b4v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b64list {
    pub buffer: *mut b8i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b64list"][::std::mem::size_of::<b64list>() - 32usize];
    ["Alignment of b64list"][::std::mem::align_of::<b64list>() - 8usize];
    ["Offset of field: b64list::buffer"][::std::mem::offset_of!(b64list, buffer) - 0usize];
    ["Offset of field: b64list::size"][::std::mem::offset_of!(b64list, size) - 8usize];
    ["Offset of field: b64list::cap"][::std::mem::offset_of!(b64list, cap) - 16usize];
};
impl Default for b64list {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b64list {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b64list_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct b8v {
    pub buffer: *mut b8i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of b8v"][::std::mem::size_of::<b8v>() - 32usize];
    ["Alignment of b8v"][::std::mem::align_of::<b8v>() - 8usize];
    ["Offset of field: b8v::buffer"][::std::mem::offset_of!(b8v, buffer) - 0usize];
    ["Offset of field: b8v::size"][::std::mem::offset_of!(b8v, size) - 8usize];
    ["Offset of field: b8v::cap"][::std::mem::offset_of!(b8v, cap) - 16usize];
};
impl Default for b8v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl b8v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static b8v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f4v {
    pub buffer: *mut f4i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of f4v"][::std::mem::size_of::<f4v>() - 32usize];
    ["Alignment of f4v"][::std::mem::align_of::<f4v>() - 8usize];
    ["Offset of field: f4v::buffer"][::std::mem::offset_of!(f4v, buffer) - 0usize];
    ["Offset of field: f4v::size"][::std::mem::offset_of!(f4v, size) - 8usize];
    ["Offset of field: f4v::cap"][::std::mem::offset_of!(f4v, cap) - 16usize];
};
impl Default for f4v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl f4v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static f4v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct f8v {
    pub buffer: *mut f8i,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of f8v"][::std::mem::size_of::<f8v>() - 32usize];
    ["Alignment of f8v"][::std::mem::align_of::<f8v>() - 8usize];
    ["Offset of field: f8v::buffer"][::std::mem::offset_of!(f8v, buffer) - 0usize];
    ["Offset of field: f8v::size"][::std::mem::offset_of!(f8v, size) - 8usize];
    ["Offset of field: f8v::cap"][::std::mem::offset_of!(f8v, cap) - 16usize];
};
impl Default for f8v {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl f8v {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static f8v_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vplist {
    pub buffer: *mut *mut ::std::os::raw::c_void,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of vplist"][::std::mem::size_of::<vplist>() - 32usize];
    ["Alignment of vplist"][::std::mem::align_of::<vplist>() - 8usize];
    ["Offset of field: vplist::buffer"][::std::mem::offset_of!(vplist, buffer) - 0usize];
    ["Offset of field: vplist::size"][::std::mem::offset_of!(vplist, size) - 8usize];
    ["Offset of field: vplist::cap"][::std::mem::offset_of!(vplist, cap) - 16usize];
};
impl Default for vplist {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl vplist {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static vplist_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cplist {
    pub buffer: *mut *mut ::std::os::raw::c_char,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cplist"][::std::mem::size_of::<cplist>() - 32usize];
    ["Alignment of cplist"][::std::mem::align_of::<cplist>() - 8usize];
    ["Offset of field: cplist::buffer"][::std::mem::offset_of!(cplist, buffer) - 0usize];
    ["Offset of field: cplist::size"][::std::mem::offset_of!(cplist, size) - 8usize];
    ["Offset of field: cplist::cap"][::std::mem::offset_of!(cplist, cap) - 16usize];
};
impl Default for cplist {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl cplist {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static cplist_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static cplist_deep_obj_desc: obj_desc_t;
}
pub type __m64 = [::std::os::raw::c_longlong; 1usize];
pub type __v1di = [::std::os::raw::c_longlong; 1usize];
pub type __v2si = [::std::os::raw::c_int; 2usize];
pub type __v4hi = [::std::os::raw::c_short; 4usize];
pub type __v8qi = [::std::os::raw::c_char; 8usize];
pub type __v4si = [::std::os::raw::c_int; 4usize];
pub type __v4sf = [f32; 4usize];
pub type __m128 = [f32; 4usize];
pub type __m128_u = [f32; 4usize];
pub type __v4su = [::std::os::raw::c_uint; 4usize];
unsafe extern "C" {
    pub fn _mm_sfence();
}
unsafe extern "C" {
    pub fn _mm_getcsr() -> ::std::os::raw::c_uint;
}
unsafe extern "C" {
    pub fn _mm_setcsr(__i: ::std::os::raw::c_uint);
}
pub type __m128d = [f64; 2usize];
pub type __m128i = [::std::os::raw::c_longlong; 2usize];
pub type __m128d_u = [f64; 2usize];
pub type __m128i_u = [::std::os::raw::c_longlong; 2usize];
pub type __v2df = [f64; 2usize];
pub type __v2di = [::std::os::raw::c_longlong; 2usize];
pub type __v8hi = [::std::os::raw::c_short; 8usize];
pub type __v16qi = [::std::os::raw::c_char; 16usize];
pub type __v2du = [::std::os::raw::c_ulonglong; 2usize];
pub type __v8hu = [::std::os::raw::c_ushort; 8usize];
pub type __v16qu = [::std::os::raw::c_uchar; 16usize];
pub type __v16qs = [::std::os::raw::c_schar; 16usize];
unsafe extern "C" {
    pub fn _mm_clflush(__p: *const ::std::os::raw::c_void);
}
unsafe extern "C" {
    pub fn _mm_lfence();
}
unsafe extern "C" {
    pub fn _mm_mfence();
}
unsafe extern "C" {
    pub fn _mm_pause();
}
pub type xint = __m128i;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct seqalign_result_t {
    pub score: ::std::os::raw::c_int,
    pub qb: ::std::os::raw::c_int,
    pub qe: ::std::os::raw::c_int,
    pub tb: ::std::os::raw::c_int,
    pub te: ::std::os::raw::c_int,
    pub mat: ::std::os::raw::c_int,
    pub mis: ::std::os::raw::c_int,
    pub ins: ::std::os::raw::c_int,
    pub del: ::std::os::raw::c_int,
    pub aln: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of seqalign_result_t"][::std::mem::size_of::<seqalign_result_t>() - 40usize];
    ["Alignment of seqalign_result_t"][::std::mem::align_of::<seqalign_result_t>() - 4usize];
    ["Offset of field: seqalign_result_t::score"]
        [::std::mem::offset_of!(seqalign_result_t, score) - 0usize];
    ["Offset of field: seqalign_result_t::qb"]
        [::std::mem::offset_of!(seqalign_result_t, qb) - 4usize];
    ["Offset of field: seqalign_result_t::qe"]
        [::std::mem::offset_of!(seqalign_result_t, qe) - 8usize];
    ["Offset of field: seqalign_result_t::tb"]
        [::std::mem::offset_of!(seqalign_result_t, tb) - 12usize];
    ["Offset of field: seqalign_result_t::te"]
        [::std::mem::offset_of!(seqalign_result_t, te) - 16usize];
    ["Offset of field: seqalign_result_t::mat"]
        [::std::mem::offset_of!(seqalign_result_t, mat) - 20usize];
    ["Offset of field: seqalign_result_t::mis"]
        [::std::mem::offset_of!(seqalign_result_t, mis) - 24usize];
    ["Offset of field: seqalign_result_t::ins"]
        [::std::mem::offset_of!(seqalign_result_t, ins) - 28usize];
    ["Offset of field: seqalign_result_t::del"]
        [::std::mem::offset_of!(seqalign_result_t, del) - 32usize];
    ["Offset of field: seqalign_result_t::aln"]
        [::std::mem::offset_of!(seqalign_result_t, aln) - 36usize];
};
pub type float_t = f32;
pub type double_t = f64;
unsafe extern "C" {
    pub fn __fpclassify(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __signbit(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isinf(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __finite(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isnan(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __iseqsig(__x: f64, __y: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __issignaling(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn acos(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __acos(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn asin(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __asin(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn atan(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __atan(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn atan2(__y: f64, __x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __atan2(__y: f64, __x: f64) -> f64;
}
unsafe extern "C" {
    pub fn cos(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __cos(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn sin(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __sin(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn tan(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __tan(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn cosh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __cosh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn sinh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __sinh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn tanh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __tanh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn acosh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __acosh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn asinh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __asinh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn atanh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __atanh(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn exp(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __exp(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn __frexp(__x: f64, __exponent: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn __ldexp(__x: f64, __exponent: ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn log(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __log(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn log10(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __log10(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn modf(__x: f64, __iptr: *mut f64) -> f64;
}
unsafe extern "C" {
    pub fn __modf(__x: f64, __iptr: *mut f64) -> f64;
}
unsafe extern "C" {
    pub fn expm1(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __expm1(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn log1p(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __log1p(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn logb(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __logb(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn exp2(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __exp2(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn log2(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __log2(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn pow(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __pow(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn sqrt(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __sqrt(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn hypot(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __hypot(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn cbrt(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __cbrt(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn ceil(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __ceil(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn fabs(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fabs(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn floor(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __floor(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn fmod(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fmod(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn isinf(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn finite(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn drem(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __drem(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn significand(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __significand(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn copysign(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __copysign(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
unsafe extern "C" {
    pub fn __nan(__tagb: *const ::std::os::raw::c_char) -> f64;
}
unsafe extern "C" {
    pub fn isnan(__value: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn j0(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __j0(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn j1(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __j1(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
unsafe extern "C" {
    pub fn __jn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
unsafe extern "C" {
    pub fn y0(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __y0(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn y1(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __y1(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
unsafe extern "C" {
    pub fn __yn(arg1: ::std::os::raw::c_int, arg2: f64) -> f64;
}
unsafe extern "C" {
    pub fn erf(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __erf(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn erfc(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __erfc(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn lgamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __lgamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn tgamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __tgamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn gamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn __gamma(arg1: f64) -> f64;
}
unsafe extern "C" {
    pub fn lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn __lgamma_r(arg1: f64, __signgamp: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn rint(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __rint(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn nextafter(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __nextafter(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn nexttoward(__x: f64, __y: u128) -> f64;
}
unsafe extern "C" {
    pub fn __nexttoward(__x: f64, __y: u128) -> f64;
}
unsafe extern "C" {
    pub fn remainder(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __remainder(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn __scalbn(__x: f64, __n: ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn ilogb(__x: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __ilogb(__x: f64) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
unsafe extern "C" {
    pub fn __scalbln(__x: f64, __n: ::std::os::raw::c_long) -> f64;
}
unsafe extern "C" {
    pub fn nearbyint(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __nearbyint(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn round(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __round(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn trunc(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn __trunc(__x: f64) -> f64;
}
unsafe extern "C" {
    pub fn remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn __remquo(__x: f64, __y: f64, __quo: *mut ::std::os::raw::c_int) -> f64;
}
unsafe extern "C" {
    pub fn lrint(__x: f64) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lrint(__x: f64) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llrint(__x: f64) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn lround(__x: f64) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lround(__x: f64) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llround(__x: f64) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llround(__x: f64) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn fdim(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fdim(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn fmax(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fmax(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn fmin(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fmin(__x: f64, __y: f64) -> f64;
}
unsafe extern "C" {
    pub fn fma(__x: f64, __y: f64, __z: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fma(__x: f64, __y: f64, __z: f64) -> f64;
}
unsafe extern "C" {
    pub fn scalb(__x: f64, __n: f64) -> f64;
}
unsafe extern "C" {
    pub fn __scalb(__x: f64, __n: f64) -> f64;
}
unsafe extern "C" {
    pub fn __fpclassifyf(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __signbitf(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isinff(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __finitef(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isnanf(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __iseqsigf(__x: f32, __y: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __issignalingf(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn acosf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __acosf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn asinf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __asinf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn atanf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __atanf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn atan2f(__y: f32, __x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __atan2f(__y: f32, __x: f32) -> f32;
}
unsafe extern "C" {
    pub fn cosf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __cosf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn sinf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __sinf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn tanf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __tanf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn coshf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __coshf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn sinhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __sinhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn tanhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __tanhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn acoshf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __acoshf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn asinhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __asinhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn atanhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __atanhf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn expf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __expf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn __frexpf(__x: f32, __exponent: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn __ldexpf(__x: f32, __exponent: ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn logf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __logf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn log10f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __log10f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn modff(__x: f32, __iptr: *mut f32) -> f32;
}
unsafe extern "C" {
    pub fn __modff(__x: f32, __iptr: *mut f32) -> f32;
}
unsafe extern "C" {
    pub fn expm1f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __expm1f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn log1pf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __log1pf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn logbf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __logbf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn exp2f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __exp2f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn log2f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __log2f(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn powf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __powf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn sqrtf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __sqrtf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn hypotf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __hypotf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn cbrtf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __cbrtf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn ceilf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __ceilf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn fabsf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fabsf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn floorf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __floorf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn fmodf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fmodf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn isinff(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn finitef(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn dremf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __dremf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn significandf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __significandf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn copysignf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __copysignf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
unsafe extern "C" {
    pub fn __nanf(__tagb: *const ::std::os::raw::c_char) -> f32;
}
unsafe extern "C" {
    pub fn isnanf(__value: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn j0f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __j0f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn j1f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __j1f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
unsafe extern "C" {
    pub fn __jnf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
unsafe extern "C" {
    pub fn y0f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __y0f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn y1f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __y1f(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
unsafe extern "C" {
    pub fn __ynf(arg1: ::std::os::raw::c_int, arg2: f32) -> f32;
}
unsafe extern "C" {
    pub fn erff(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __erff(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn erfcf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __erfcf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn lgammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __lgammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn tgammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __tgammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn gammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn __gammaf(arg1: f32) -> f32;
}
unsafe extern "C" {
    pub fn lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn __lgammaf_r(arg1: f32, __signgamp: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn rintf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __rintf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn nextafterf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __nextafterf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn nexttowardf(__x: f32, __y: u128) -> f32;
}
unsafe extern "C" {
    pub fn __nexttowardf(__x: f32, __y: u128) -> f32;
}
unsafe extern "C" {
    pub fn remainderf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __remainderf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn __scalbnf(__x: f32, __n: ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __ilogbf(__x: f32) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
unsafe extern "C" {
    pub fn __scalblnf(__x: f32, __n: ::std::os::raw::c_long) -> f32;
}
unsafe extern "C" {
    pub fn nearbyintf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __nearbyintf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn roundf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __roundf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn truncf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn __truncf(__x: f32) -> f32;
}
unsafe extern "C" {
    pub fn remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn __remquof(__x: f32, __y: f32, __quo: *mut ::std::os::raw::c_int) -> f32;
}
unsafe extern "C" {
    pub fn lrintf(__x: f32) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lrintf(__x: f32) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llrintf(__x: f32) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn lroundf(__x: f32) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lroundf(__x: f32) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llroundf(__x: f32) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn fdimf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fdimf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn fmaxf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fmaxf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn fminf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fminf(__x: f32, __y: f32) -> f32;
}
unsafe extern "C" {
    pub fn fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fmaf(__x: f32, __y: f32, __z: f32) -> f32;
}
unsafe extern "C" {
    pub fn scalbf(__x: f32, __n: f32) -> f32;
}
unsafe extern "C" {
    pub fn __scalbf(__x: f32, __n: f32) -> f32;
}
unsafe extern "C" {
    pub fn __fpclassifyl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __signbitl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isinfl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __finitel(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __isnanl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __iseqsigl(__x: u128, __y: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __issignalingl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn acosl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __acosl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn asinl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __asinl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn atanl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __atanl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn atan2l(__y: u128, __x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __atan2l(__y: u128, __x: u128) -> u128;
}
unsafe extern "C" {
    pub fn cosl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __cosl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn sinl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __sinl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn tanl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __tanl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn coshl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __coshl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn sinhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __sinhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn tanhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __tanhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn acoshl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __acoshl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn asinhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __asinhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn atanhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __atanhl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn expl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __expl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn __frexpl(__x: u128, __exponent: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn __ldexpl(__x: u128, __exponent: ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn logl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __logl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn log10l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __log10l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn modfl(__x: u128, __iptr: *mut u128) -> u128;
}
unsafe extern "C" {
    pub fn __modfl(__x: u128, __iptr: *mut u128) -> u128;
}
unsafe extern "C" {
    pub fn expm1l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __expm1l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn log1pl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __log1pl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn logbl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __logbl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn exp2l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __exp2l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn log2l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __log2l(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn powl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __powl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn sqrtl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __sqrtl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn hypotl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __hypotl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn cbrtl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __cbrtl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn ceill(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __ceill(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn fabsl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fabsl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn floorl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __floorl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn fmodl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fmodl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn isinfl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn finitel(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn dreml(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __dreml(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn significandl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __significandl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn copysignl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __copysignl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
unsafe extern "C" {
    pub fn __nanl(__tagb: *const ::std::os::raw::c_char) -> u128;
}
unsafe extern "C" {
    pub fn isnanl(__value: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn j0l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __j0l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn j1l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __j1l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
unsafe extern "C" {
    pub fn __jnl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
unsafe extern "C" {
    pub fn y0l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __y0l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn y1l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __y1l(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
unsafe extern "C" {
    pub fn __ynl(arg1: ::std::os::raw::c_int, arg2: u128) -> u128;
}
unsafe extern "C" {
    pub fn erfl(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __erfl(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn erfcl(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __erfcl(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn lgammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __lgammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn tgammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __tgammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn gammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn __gammal(arg1: u128) -> u128;
}
unsafe extern "C" {
    pub fn lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn __lgammal_r(arg1: u128, __signgamp: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn rintl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __rintl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn nextafterl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __nextafterl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn nexttowardl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __nexttowardl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn remainderl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __remainderl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn __scalbnl(__x: u128, __n: ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn __ilogbl(__x: u128) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
unsafe extern "C" {
    pub fn __scalblnl(__x: u128, __n: ::std::os::raw::c_long) -> u128;
}
unsafe extern "C" {
    pub fn nearbyintl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __nearbyintl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn roundl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __roundl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn truncl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn __truncl(__x: u128) -> u128;
}
unsafe extern "C" {
    pub fn remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn __remquol(__x: u128, __y: u128, __quo: *mut ::std::os::raw::c_int) -> u128;
}
unsafe extern "C" {
    pub fn lrintl(__x: u128) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lrintl(__x: u128) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llrintl(__x: u128) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn lroundl(__x: u128) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn __lroundl(__x: u128) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn __llroundl(__x: u128) -> ::std::os::raw::c_longlong;
}
unsafe extern "C" {
    pub fn fdiml(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fdiml(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn fmaxl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fmaxl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn fminl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fminl(__x: u128, __y: u128) -> u128;
}
unsafe extern "C" {
    pub fn fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
unsafe extern "C" {
    pub fn __fmal(__x: u128, __y: u128, __z: u128) -> u128;
}
unsafe extern "C" {
    pub fn scalbl(__x: u128, __n: u128) -> u128;
}
unsafe extern "C" {
    pub fn __scalbl(__x: u128, __n: u128) -> u128;
}
unsafe extern "C" {
    pub static mut signgam: ::std::os::raw::c_int;
}
pub const FP_NAN: _bindgen_ty_23 = 0;
pub const FP_INFINITE: _bindgen_ty_23 = 1;
pub const FP_ZERO: _bindgen_ty_23 = 2;
pub const FP_SUBNORMAL: _bindgen_ty_23 = 3;
pub const FP_NORMAL: _bindgen_ty_23 = 4;
pub type _bindgen_ty_23 = ::std::os::raw::c_uint;
unsafe extern "C" {
    pub static byte_ones_table: [u1i; 256usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitVec {
    pub bits: *mut u8i,
    pub n_bit: u8i,
    pub n_cap: u8i,
    pub sums: *mut u8i,
    pub sum_size: u8i,
    pub n_ones: u8i,
    pub hash: *mut u8i,
    pub hash_size: u8i,
    pub hash_mod: u8i,
    pub iter_idx: b8i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BitVec"][::std::mem::size_of::<BitVec>() - 80usize];
    ["Alignment of BitVec"][::std::mem::align_of::<BitVec>() - 8usize];
    ["Offset of field: BitVec::bits"][::std::mem::offset_of!(BitVec, bits) - 0usize];
    ["Offset of field: BitVec::n_bit"][::std::mem::offset_of!(BitVec, n_bit) - 8usize];
    ["Offset of field: BitVec::n_cap"][::std::mem::offset_of!(BitVec, n_cap) - 16usize];
    ["Offset of field: BitVec::sums"][::std::mem::offset_of!(BitVec, sums) - 24usize];
    ["Offset of field: BitVec::sum_size"][::std::mem::offset_of!(BitVec, sum_size) - 32usize];
    ["Offset of field: BitVec::n_ones"][::std::mem::offset_of!(BitVec, n_ones) - 40usize];
    ["Offset of field: BitVec::hash"][::std::mem::offset_of!(BitVec, hash) - 48usize];
    ["Offset of field: BitVec::hash_size"][::std::mem::offset_of!(BitVec, hash_size) - 56usize];
    ["Offset of field: BitVec::hash_mod"][::std::mem::offset_of!(BitVec, hash_mod) - 64usize];
    ["Offset of field: BitVec::iter_idx"][::std::mem::offset_of!(BitVec, iter_idx) - 72usize];
};
impl Default for BitVec {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static bitvec_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static Mod37BitPosition: [::std::os::raw::c_int; 37usize];
}
unsafe extern "C" {
    pub static sys_prime_list: [u64; 61usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u32hash {
    pub array: *mut u32,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u32hash"][::std::mem::size_of::<u32hash>() - 88usize];
    ["Alignment of u32hash"][::std::mem::align_of::<u32hash>() - 8usize];
    ["Offset of field: u32hash::array"][::std::mem::offset_of!(u32hash, array) - 0usize];
    ["Offset of field: u32hash::ones"][::std::mem::offset_of!(u32hash, ones) - 8usize];
    ["Offset of field: u32hash::dels"][::std::mem::offset_of!(u32hash, dels) - 16usize];
    ["Offset of field: u32hash::e_size"][::std::mem::offset_of!(u32hash, e_size) - 24usize];
    ["Offset of field: u32hash::ocp"][::std::mem::offset_of!(u32hash, ocp) - 32usize];
    ["Offset of field: u32hash::size"][::std::mem::offset_of!(u32hash, size) - 40usize];
    ["Offset of field: u32hash::count"][::std::mem::offset_of!(u32hash, count) - 48usize];
    ["Offset of field: u32hash::max"][::std::mem::offset_of!(u32hash, max) - 56usize];
    ["Offset of field: u32hash::load_factor"]
        [::std::mem::offset_of!(u32hash, load_factor) - 64usize];
    ["Offset of field: u32hash::iter_ptr"][::std::mem::offset_of!(u32hash, iter_ptr) - 72usize];
    ["Offset of field: u32hash::userdata"][::std::mem::offset_of!(u32hash, userdata) - 80usize];
};
impl Default for u32hash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static u32hash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct u64hash {
    pub array: *mut u64,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of u64hash"][::std::mem::size_of::<u64hash>() - 88usize];
    ["Alignment of u64hash"][::std::mem::align_of::<u64hash>() - 8usize];
    ["Offset of field: u64hash::array"][::std::mem::offset_of!(u64hash, array) - 0usize];
    ["Offset of field: u64hash::ones"][::std::mem::offset_of!(u64hash, ones) - 8usize];
    ["Offset of field: u64hash::dels"][::std::mem::offset_of!(u64hash, dels) - 16usize];
    ["Offset of field: u64hash::e_size"][::std::mem::offset_of!(u64hash, e_size) - 24usize];
    ["Offset of field: u64hash::ocp"][::std::mem::offset_of!(u64hash, ocp) - 32usize];
    ["Offset of field: u64hash::size"][::std::mem::offset_of!(u64hash, size) - 40usize];
    ["Offset of field: u64hash::count"][::std::mem::offset_of!(u64hash, count) - 48usize];
    ["Offset of field: u64hash::max"][::std::mem::offset_of!(u64hash, max) - 56usize];
    ["Offset of field: u64hash::load_factor"]
        [::std::mem::offset_of!(u64hash, load_factor) - 64usize];
    ["Offset of field: u64hash::iter_ptr"][::std::mem::offset_of!(u64hash, iter_ptr) - 72usize];
    ["Offset of field: u64hash::userdata"][::std::mem::offset_of!(u64hash, userdata) - 80usize];
};
impl Default for u64hash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static u64hash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct i32hash {
    pub array: *mut ::std::os::raw::c_int,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of i32hash"][::std::mem::size_of::<i32hash>() - 88usize];
    ["Alignment of i32hash"][::std::mem::align_of::<i32hash>() - 8usize];
    ["Offset of field: i32hash::array"][::std::mem::offset_of!(i32hash, array) - 0usize];
    ["Offset of field: i32hash::ones"][::std::mem::offset_of!(i32hash, ones) - 8usize];
    ["Offset of field: i32hash::dels"][::std::mem::offset_of!(i32hash, dels) - 16usize];
    ["Offset of field: i32hash::e_size"][::std::mem::offset_of!(i32hash, e_size) - 24usize];
    ["Offset of field: i32hash::ocp"][::std::mem::offset_of!(i32hash, ocp) - 32usize];
    ["Offset of field: i32hash::size"][::std::mem::offset_of!(i32hash, size) - 40usize];
    ["Offset of field: i32hash::count"][::std::mem::offset_of!(i32hash, count) - 48usize];
    ["Offset of field: i32hash::max"][::std::mem::offset_of!(i32hash, max) - 56usize];
    ["Offset of field: i32hash::load_factor"]
        [::std::mem::offset_of!(i32hash, load_factor) - 64usize];
    ["Offset of field: i32hash::iter_ptr"][::std::mem::offset_of!(i32hash, iter_ptr) - 72usize];
    ["Offset of field: i32hash::userdata"][::std::mem::offset_of!(i32hash, userdata) - 80usize];
};
impl Default for i32hash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static i32hash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct chash {
    pub array: *mut *mut ::std::os::raw::c_char,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of chash"][::std::mem::size_of::<chash>() - 88usize];
    ["Alignment of chash"][::std::mem::align_of::<chash>() - 8usize];
    ["Offset of field: chash::array"][::std::mem::offset_of!(chash, array) - 0usize];
    ["Offset of field: chash::ones"][::std::mem::offset_of!(chash, ones) - 8usize];
    ["Offset of field: chash::dels"][::std::mem::offset_of!(chash, dels) - 16usize];
    ["Offset of field: chash::e_size"][::std::mem::offset_of!(chash, e_size) - 24usize];
    ["Offset of field: chash::ocp"][::std::mem::offset_of!(chash, ocp) - 32usize];
    ["Offset of field: chash::size"][::std::mem::offset_of!(chash, size) - 40usize];
    ["Offset of field: chash::count"][::std::mem::offset_of!(chash, count) - 48usize];
    ["Offset of field: chash::max"][::std::mem::offset_of!(chash, max) - 56usize];
    ["Offset of field: chash::load_factor"][::std::mem::offset_of!(chash, load_factor) - 64usize];
    ["Offset of field: chash::iter_ptr"][::std::mem::offset_of!(chash, iter_ptr) - 72usize];
    ["Offset of field: chash::userdata"][::std::mem::offset_of!(chash, userdata) - 80usize];
};
impl Default for chash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static chash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct uuhash_t {
    pub key: u4i,
    pub val: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of uuhash_t"][::std::mem::size_of::<uuhash_t>() - 8usize];
    ["Alignment of uuhash_t"][::std::mem::align_of::<uuhash_t>() - 4usize];
    ["Offset of field: uuhash_t::key"][::std::mem::offset_of!(uuhash_t, key) - 0usize];
    ["Offset of field: uuhash_t::val"][::std::mem::offset_of!(uuhash_t, val) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uuhash {
    pub array: *mut uuhash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of uuhash"][::std::mem::size_of::<uuhash>() - 88usize];
    ["Alignment of uuhash"][::std::mem::align_of::<uuhash>() - 8usize];
    ["Offset of field: uuhash::array"][::std::mem::offset_of!(uuhash, array) - 0usize];
    ["Offset of field: uuhash::ones"][::std::mem::offset_of!(uuhash, ones) - 8usize];
    ["Offset of field: uuhash::dels"][::std::mem::offset_of!(uuhash, dels) - 16usize];
    ["Offset of field: uuhash::e_size"][::std::mem::offset_of!(uuhash, e_size) - 24usize];
    ["Offset of field: uuhash::ocp"][::std::mem::offset_of!(uuhash, ocp) - 32usize];
    ["Offset of field: uuhash::size"][::std::mem::offset_of!(uuhash, size) - 40usize];
    ["Offset of field: uuhash::count"][::std::mem::offset_of!(uuhash, count) - 48usize];
    ["Offset of field: uuhash::max"][::std::mem::offset_of!(uuhash, max) - 56usize];
    ["Offset of field: uuhash::load_factor"][::std::mem::offset_of!(uuhash, load_factor) - 64usize];
    ["Offset of field: uuhash::iter_ptr"][::std::mem::offset_of!(uuhash, iter_ptr) - 72usize];
    ["Offset of field: uuhash::userdata"][::std::mem::offset_of!(uuhash, userdata) - 80usize];
};
impl Default for uuhash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static uuhash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct uihash_t {
    pub key: u4i,
    pub val: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of uihash_t"][::std::mem::size_of::<uihash_t>() - 8usize];
    ["Alignment of uihash_t"][::std::mem::align_of::<uihash_t>() - 4usize];
    ["Offset of field: uihash_t::key"][::std::mem::offset_of!(uihash_t, key) - 0usize];
    ["Offset of field: uihash_t::val"][::std::mem::offset_of!(uihash_t, val) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uihash {
    pub array: *mut uihash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of uihash"][::std::mem::size_of::<uihash>() - 88usize];
    ["Alignment of uihash"][::std::mem::align_of::<uihash>() - 8usize];
    ["Offset of field: uihash::array"][::std::mem::offset_of!(uihash, array) - 0usize];
    ["Offset of field: uihash::ones"][::std::mem::offset_of!(uihash, ones) - 8usize];
    ["Offset of field: uihash::dels"][::std::mem::offset_of!(uihash, dels) - 16usize];
    ["Offset of field: uihash::e_size"][::std::mem::offset_of!(uihash, e_size) - 24usize];
    ["Offset of field: uihash::ocp"][::std::mem::offset_of!(uihash, ocp) - 32usize];
    ["Offset of field: uihash::size"][::std::mem::offset_of!(uihash, size) - 40usize];
    ["Offset of field: uihash::count"][::std::mem::offset_of!(uihash, count) - 48usize];
    ["Offset of field: uihash::max"][::std::mem::offset_of!(uihash, max) - 56usize];
    ["Offset of field: uihash::load_factor"][::std::mem::offset_of!(uihash, load_factor) - 64usize];
    ["Offset of field: uihash::iter_ptr"][::std::mem::offset_of!(uihash, iter_ptr) - 72usize];
    ["Offset of field: uihash::userdata"][::std::mem::offset_of!(uihash, userdata) - 80usize];
};
impl Default for uihash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static uihash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct UUhash_t {
    pub key: u8i,
    pub val: u8i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UUhash_t"][::std::mem::size_of::<UUhash_t>() - 16usize];
    ["Alignment of UUhash_t"][::std::mem::align_of::<UUhash_t>() - 8usize];
    ["Offset of field: UUhash_t::key"][::std::mem::offset_of!(UUhash_t, key) - 0usize];
    ["Offset of field: UUhash_t::val"][::std::mem::offset_of!(UUhash_t, val) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UUhash {
    pub array: *mut UUhash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UUhash"][::std::mem::size_of::<UUhash>() - 88usize];
    ["Alignment of UUhash"][::std::mem::align_of::<UUhash>() - 8usize];
    ["Offset of field: UUhash::array"][::std::mem::offset_of!(UUhash, array) - 0usize];
    ["Offset of field: UUhash::ones"][::std::mem::offset_of!(UUhash, ones) - 8usize];
    ["Offset of field: UUhash::dels"][::std::mem::offset_of!(UUhash, dels) - 16usize];
    ["Offset of field: UUhash::e_size"][::std::mem::offset_of!(UUhash, e_size) - 24usize];
    ["Offset of field: UUhash::ocp"][::std::mem::offset_of!(UUhash, ocp) - 32usize];
    ["Offset of field: UUhash::size"][::std::mem::offset_of!(UUhash, size) - 40usize];
    ["Offset of field: UUhash::count"][::std::mem::offset_of!(UUhash, count) - 48usize];
    ["Offset of field: UUhash::max"][::std::mem::offset_of!(UUhash, max) - 56usize];
    ["Offset of field: UUhash::load_factor"][::std::mem::offset_of!(UUhash, load_factor) - 64usize];
    ["Offset of field: UUhash::iter_ptr"][::std::mem::offset_of!(UUhash, iter_ptr) - 72usize];
    ["Offset of field: UUhash::userdata"][::std::mem::offset_of!(UUhash, userdata) - 80usize];
};
impl Default for UUhash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static UUhash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cuhash_t {
    pub key: *mut ::std::os::raw::c_char,
    pub val: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cuhash_t"][::std::mem::size_of::<cuhash_t>() - 16usize];
    ["Alignment of cuhash_t"][::std::mem::align_of::<cuhash_t>() - 8usize];
    ["Offset of field: cuhash_t::key"][::std::mem::offset_of!(cuhash_t, key) - 0usize];
    ["Offset of field: cuhash_t::val"][::std::mem::offset_of!(cuhash_t, val) - 8usize];
};
impl Default for cuhash_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cuhash {
    pub array: *mut cuhash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cuhash"][::std::mem::size_of::<cuhash>() - 88usize];
    ["Alignment of cuhash"][::std::mem::align_of::<cuhash>() - 8usize];
    ["Offset of field: cuhash::array"][::std::mem::offset_of!(cuhash, array) - 0usize];
    ["Offset of field: cuhash::ones"][::std::mem::offset_of!(cuhash, ones) - 8usize];
    ["Offset of field: cuhash::dels"][::std::mem::offset_of!(cuhash, dels) - 16usize];
    ["Offset of field: cuhash::e_size"][::std::mem::offset_of!(cuhash, e_size) - 24usize];
    ["Offset of field: cuhash::ocp"][::std::mem::offset_of!(cuhash, ocp) - 32usize];
    ["Offset of field: cuhash::size"][::std::mem::offset_of!(cuhash, size) - 40usize];
    ["Offset of field: cuhash::count"][::std::mem::offset_of!(cuhash, count) - 48usize];
    ["Offset of field: cuhash::max"][::std::mem::offset_of!(cuhash, max) - 56usize];
    ["Offset of field: cuhash::load_factor"][::std::mem::offset_of!(cuhash, load_factor) - 64usize];
    ["Offset of field: cuhash::iter_ptr"][::std::mem::offset_of!(cuhash, iter_ptr) - 72usize];
    ["Offset of field: cuhash::userdata"][::std::mem::offset_of!(cuhash, userdata) - 80usize];
};
impl Default for cuhash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static cuhash_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static cuhash_struct_deep_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static cuhash_deep_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cihash_t {
    pub key: *mut ::std::os::raw::c_char,
    pub val: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cihash_t"][::std::mem::size_of::<cihash_t>() - 16usize];
    ["Alignment of cihash_t"][::std::mem::align_of::<cihash_t>() - 8usize];
    ["Offset of field: cihash_t::key"][::std::mem::offset_of!(cihash_t, key) - 0usize];
    ["Offset of field: cihash_t::val"][::std::mem::offset_of!(cihash_t, val) - 8usize];
};
impl Default for cihash_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cihash {
    pub array: *mut cihash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cihash"][::std::mem::size_of::<cihash>() - 88usize];
    ["Alignment of cihash"][::std::mem::align_of::<cihash>() - 8usize];
    ["Offset of field: cihash::array"][::std::mem::offset_of!(cihash, array) - 0usize];
    ["Offset of field: cihash::ones"][::std::mem::offset_of!(cihash, ones) - 8usize];
    ["Offset of field: cihash::dels"][::std::mem::offset_of!(cihash, dels) - 16usize];
    ["Offset of field: cihash::e_size"][::std::mem::offset_of!(cihash, e_size) - 24usize];
    ["Offset of field: cihash::ocp"][::std::mem::offset_of!(cihash, ocp) - 32usize];
    ["Offset of field: cihash::size"][::std::mem::offset_of!(cihash, size) - 40usize];
    ["Offset of field: cihash::count"][::std::mem::offset_of!(cihash, count) - 48usize];
    ["Offset of field: cihash::max"][::std::mem::offset_of!(cihash, max) - 56usize];
    ["Offset of field: cihash::load_factor"][::std::mem::offset_of!(cihash, load_factor) - 64usize];
    ["Offset of field: cihash::iter_ptr"][::std::mem::offset_of!(cihash, iter_ptr) - 72usize];
    ["Offset of field: cihash::userdata"][::std::mem::offset_of!(cihash, userdata) - 80usize];
};
impl Default for cihash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static cihash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clhash_t {
    pub key: *mut ::std::os::raw::c_char,
    pub val: ::std::os::raw::c_ulonglong,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of clhash_t"][::std::mem::size_of::<clhash_t>() - 16usize];
    ["Alignment of clhash_t"][::std::mem::align_of::<clhash_t>() - 8usize];
    ["Offset of field: clhash_t::key"][::std::mem::offset_of!(clhash_t, key) - 0usize];
    ["Offset of field: clhash_t::val"][::std::mem::offset_of!(clhash_t, val) - 8usize];
};
impl Default for clhash_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clhash {
    pub array: *mut clhash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of clhash"][::std::mem::size_of::<clhash>() - 88usize];
    ["Alignment of clhash"][::std::mem::align_of::<clhash>() - 8usize];
    ["Offset of field: clhash::array"][::std::mem::offset_of!(clhash, array) - 0usize];
    ["Offset of field: clhash::ones"][::std::mem::offset_of!(clhash, ones) - 8usize];
    ["Offset of field: clhash::dels"][::std::mem::offset_of!(clhash, dels) - 16usize];
    ["Offset of field: clhash::e_size"][::std::mem::offset_of!(clhash, e_size) - 24usize];
    ["Offset of field: clhash::ocp"][::std::mem::offset_of!(clhash, ocp) - 32usize];
    ["Offset of field: clhash::size"][::std::mem::offset_of!(clhash, size) - 40usize];
    ["Offset of field: clhash::count"][::std::mem::offset_of!(clhash, count) - 48usize];
    ["Offset of field: clhash::max"][::std::mem::offset_of!(clhash, max) - 56usize];
    ["Offset of field: clhash::load_factor"][::std::mem::offset_of!(clhash, load_factor) - 64usize];
    ["Offset of field: clhash::iter_ptr"][::std::mem::offset_of!(clhash, iter_ptr) - 72usize];
    ["Offset of field: clhash::userdata"][::std::mem::offset_of!(clhash, userdata) - 80usize];
};
impl Default for clhash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static clhash_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cchash_t {
    pub key: *mut ::std::os::raw::c_char,
    pub val: *mut ::std::os::raw::c_char,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cchash_t"][::std::mem::size_of::<cchash_t>() - 16usize];
    ["Alignment of cchash_t"][::std::mem::align_of::<cchash_t>() - 8usize];
    ["Offset of field: cchash_t::key"][::std::mem::offset_of!(cchash_t, key) - 0usize];
    ["Offset of field: cchash_t::val"][::std::mem::offset_of!(cchash_t, val) - 8usize];
};
impl Default for cchash_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cchash {
    pub array: *mut cchash_t,
    pub ones: *mut BitVec,
    pub dels: *mut BitVec,
    pub e_size: usize,
    pub ocp: usize,
    pub size: usize,
    pub count: usize,
    pub max: usize,
    pub load_factor: f32,
    pub iter_ptr: usize,
    pub userdata: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of cchash"][::std::mem::size_of::<cchash>() - 88usize];
    ["Alignment of cchash"][::std::mem::align_of::<cchash>() - 8usize];
    ["Offset of field: cchash::array"][::std::mem::offset_of!(cchash, array) - 0usize];
    ["Offset of field: cchash::ones"][::std::mem::offset_of!(cchash, ones) - 8usize];
    ["Offset of field: cchash::dels"][::std::mem::offset_of!(cchash, dels) - 16usize];
    ["Offset of field: cchash::e_size"][::std::mem::offset_of!(cchash, e_size) - 24usize];
    ["Offset of field: cchash::ocp"][::std::mem::offset_of!(cchash, ocp) - 32usize];
    ["Offset of field: cchash::size"][::std::mem::offset_of!(cchash, size) - 40usize];
    ["Offset of field: cchash::count"][::std::mem::offset_of!(cchash, count) - 48usize];
    ["Offset of field: cchash::max"][::std::mem::offset_of!(cchash, max) - 56usize];
    ["Offset of field: cchash::load_factor"][::std::mem::offset_of!(cchash, load_factor) - 64usize];
    ["Offset of field: cchash::iter_ptr"][::std::mem::offset_of!(cchash, iter_ptr) - 72usize];
    ["Offset of field: cchash::userdata"][::std::mem::offset_of!(cchash, userdata) - 80usize];
};
impl Default for cchash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static cchash_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static base_bit_table: [u1i; 256usize];
}
unsafe extern "C" {
    pub static base_bit4_table: [u1i; 256usize];
}
unsafe extern "C" {
    pub static bit4_bit_table: [u1i; 16usize];
}
unsafe extern "C" {
    pub static bit_base_table: [::std::os::raw::c_char; 13usize];
}
unsafe extern "C" {
    pub static bit4_base_table: [::std::os::raw::c_char; 17usize];
}
unsafe extern "C" {
    pub static spare_2bits_table: [u4i; 256usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseBank {
    pub bits: *mut u8i,
    pub size: u8i,
    pub cap: u8i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BaseBank"][::std::mem::size_of::<BaseBank>() - 24usize];
    ["Alignment of BaseBank"][::std::mem::align_of::<BaseBank>() - 8usize];
    ["Offset of field: BaseBank::bits"][::std::mem::offset_of!(BaseBank, bits) - 0usize];
    ["Offset of field: BaseBank::size"][::std::mem::offset_of!(BaseBank, size) - 8usize];
    ["Offset of field: BaseBank::cap"][::std::mem::offset_of!(BaseBank, cap) - 16usize];
};
impl Default for BaseBank {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static basebank_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _mradix_struct {
    pub _mradix_params: *mut _mradix_struct,
    pub _mradix_array: *mut _mradix_struct,
    pub n_cpu: ::std::os::raw::c_int,
    pub t_idx: ::std::os::raw::c_int,
    pub running: ::std::os::raw::c_int,
    pub state: ::std::os::raw::c_int,
    pub once: ::std::os::raw::c_int,
    pub mutex_lock: *mut pthread_mutex_t,
    pub rw_lock: *mut pthread_rwlock_t,
    pub _COND_LOCK: pthread_mutex_t,
    pub _COND: pthread_cond_t,
    pub bb: *mut BaseBank,
    pub counts: [*mut u4i; 2usize],
    pub offs: *mut u4i,
    pub lcps: *mut u1v,
    pub size: u4i,
    pub klen: u4i,
    pub task: ::std::os::raw::c_int,
    pub log: *mut FILE,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _mradix_struct"][::std::mem::size_of::<_mradix_struct>() - 208usize];
    ["Alignment of _mradix_struct"][::std::mem::align_of::<_mradix_struct>() - 8usize];
    ["Offset of field: _mradix_struct::_mradix_params"]
        [::std::mem::offset_of!(_mradix_struct, _mradix_params) - 0usize];
    ["Offset of field: _mradix_struct::_mradix_array"]
        [::std::mem::offset_of!(_mradix_struct, _mradix_array) - 8usize];
    ["Offset of field: _mradix_struct::n_cpu"]
        [::std::mem::offset_of!(_mradix_struct, n_cpu) - 16usize];
    ["Offset of field: _mradix_struct::t_idx"]
        [::std::mem::offset_of!(_mradix_struct, t_idx) - 20usize];
    ["Offset of field: _mradix_struct::running"]
        [::std::mem::offset_of!(_mradix_struct, running) - 24usize];
    ["Offset of field: _mradix_struct::state"]
        [::std::mem::offset_of!(_mradix_struct, state) - 28usize];
    ["Offset of field: _mradix_struct::once"]
        [::std::mem::offset_of!(_mradix_struct, once) - 32usize];
    ["Offset of field: _mradix_struct::mutex_lock"]
        [::std::mem::offset_of!(_mradix_struct, mutex_lock) - 40usize];
    ["Offset of field: _mradix_struct::rw_lock"]
        [::std::mem::offset_of!(_mradix_struct, rw_lock) - 48usize];
    ["Offset of field: _mradix_struct::_COND_LOCK"]
        [::std::mem::offset_of!(_mradix_struct, _COND_LOCK) - 56usize];
    ["Offset of field: _mradix_struct::_COND"]
        [::std::mem::offset_of!(_mradix_struct, _COND) - 96usize];
    ["Offset of field: _mradix_struct::bb"][::std::mem::offset_of!(_mradix_struct, bb) - 144usize];
    ["Offset of field: _mradix_struct::counts"]
        [::std::mem::offset_of!(_mradix_struct, counts) - 152usize];
    ["Offset of field: _mradix_struct::offs"]
        [::std::mem::offset_of!(_mradix_struct, offs) - 168usize];
    ["Offset of field: _mradix_struct::lcps"]
        [::std::mem::offset_of!(_mradix_struct, lcps) - 176usize];
    ["Offset of field: _mradix_struct::size"]
        [::std::mem::offset_of!(_mradix_struct, size) - 184usize];
    ["Offset of field: _mradix_struct::klen"]
        [::std::mem::offset_of!(_mradix_struct, klen) - 188usize];
    ["Offset of field: _mradix_struct::task"]
        [::std::mem::offset_of!(_mradix_struct, task) - 192usize];
    ["Offset of field: _mradix_struct::log"]
        [::std::mem::offset_of!(_mradix_struct, log) - 200usize];
};
impl Default for _mradix_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SeqBank {
    pub nseq: u4i,
    pub rdseqs: *mut BaseBank,
    pub rdtags: *mut cplist,
    pub rdoffs: *mut u8v,
    pub rdlens: *mut u4v,
    pub rdhash: *mut cuhash,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SeqBank"][::std::mem::size_of::<SeqBank>() - 48usize];
    ["Alignment of SeqBank"][::std::mem::align_of::<SeqBank>() - 8usize];
    ["Offset of field: SeqBank::nseq"][::std::mem::offset_of!(SeqBank, nseq) - 0usize];
    ["Offset of field: SeqBank::rdseqs"][::std::mem::offset_of!(SeqBank, rdseqs) - 8usize];
    ["Offset of field: SeqBank::rdtags"][::std::mem::offset_of!(SeqBank, rdtags) - 16usize];
    ["Offset of field: SeqBank::rdoffs"][::std::mem::offset_of!(SeqBank, rdoffs) - 24usize];
    ["Offset of field: SeqBank::rdlens"][::std::mem::offset_of!(SeqBank, rdlens) - 32usize];
    ["Offset of field: SeqBank::rdhash"][::std::mem::offset_of!(SeqBank, rdhash) - 40usize];
};
impl Default for SeqBank {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub static seqbank_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct seqbank_reg_t {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub off: u4i,
    pub len: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of seqbank_reg_t"][::std::mem::size_of::<seqbank_reg_t>() - 12usize];
    ["Alignment of seqbank_reg_t"][::std::mem::align_of::<seqbank_reg_t>() - 4usize];
    ["Offset of field: seqbank_reg_t::off"][::std::mem::offset_of!(seqbank_reg_t, off) - 4usize];
    ["Offset of field: seqbank_reg_t::len"][::std::mem::offset_of!(seqbank_reg_t, len) - 8usize];
};
impl seqbank_reg_t {
    #[inline]
    pub fn rid(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 30u8) as u32) }
    }
    #[inline]
    pub fn set_rid(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 30u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rid_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                30u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_rid_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                30u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn dir(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(30usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_dir(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(30usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn dir_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                30usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_dir_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                30usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn status(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_status(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn status_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                31usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_status_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                31usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(rid: u4i, dir: u4i, status: u4i) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 30u8, {
            let rid: u32 = unsafe { ::std::mem::transmute(rid) };
            rid as u64
        });
        __bindgen_bitfield_unit.set(30usize, 1u8, {
            let dir: u32 = unsafe { ::std::mem::transmute(dir) };
            dir as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let status: u32 = unsafe { ::std::mem::transmute(status) };
            status as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct seqbankregv {
    pub buffer: *mut seqbank_reg_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of seqbankregv"][::std::mem::size_of::<seqbankregv>() - 32usize];
    ["Alignment of seqbankregv"][::std::mem::align_of::<seqbankregv>() - 8usize];
    ["Offset of field: seqbankregv::buffer"][::std::mem::offset_of!(seqbankregv, buffer) - 0usize];
    ["Offset of field: seqbankregv::size"][::std::mem::offset_of!(seqbankregv, size) - 8usize];
    ["Offset of field: seqbankregv::cap"][::std::mem::offset_of!(seqbankregv, cap) - 16usize];
};
impl Default for seqbankregv {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl seqbankregv {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static seqbankregv_obj_desc: obj_desc_t;
}
pub type string_size_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct String {
    pub string: *mut ::std::os::raw::c_char,
    pub size: string_size_t,
    pub capacity: string_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of String"][::std::mem::size_of::<String>() - 16usize];
    ["Alignment of String"][::std::mem::align_of::<String>() - 8usize];
    ["Offset of field: String::string"][::std::mem::offset_of!(String, string) - 0usize];
    ["Offset of field: String::size"][::std::mem::offset_of!(String, size) - 8usize];
    ["Offset of field: String::capacity"][::std::mem::offset_of!(String, capacity) - 12usize];
};
impl Default for String {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VString {
    pub string: *mut ::std::os::raw::c_char,
    pub size: string_size_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VString"][::std::mem::size_of::<VString>() - 16usize];
    ["Alignment of VString"][::std::mem::align_of::<VString>() - 8usize];
    ["Offset of field: VString::string"][::std::mem::offset_of!(VString, string) - 0usize];
    ["Offset of field: VString::size"][::std::mem::offset_of!(VString, size) - 8usize];
};
impl Default for VString {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VStrv {
    pub buffer: *mut VString,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VStrv"][::std::mem::size_of::<VStrv>() - 32usize];
    ["Alignment of VStrv"][::std::mem::align_of::<VStrv>() - 8usize];
    ["Offset of field: VStrv::buffer"][::std::mem::offset_of!(VStrv, buffer) - 0usize];
    ["Offset of field: VStrv::size"][::std::mem::offset_of!(VStrv, size) - 8usize];
    ["Offset of field: VStrv::cap"][::std::mem::offset_of!(VStrv, cap) - 16usize];
};
impl Default for VStrv {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl VStrv {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static VStrv_obj_desc: obj_desc_t;
}
unsafe extern "C" {
    pub static string_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoanode_t {
    pub rid: u2i,
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub vst: u2i,
    pub nin: u2i,
    pub nou: u2i,
    pub nct: u2i,
    pub cov: u2i,
    pub pos: ::std::os::raw::c_int,
    pub mpos: ::std::os::raw::c_int,
    pub cpos: ::std::os::raw::c_int,
    pub rpos: ::std::os::raw::c_int,
    pub edge: u4i,
    pub erev: u4i,
    pub next: u4i,
    pub prev: u4i,
    pub header: u4i,
    pub mmidx: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoanode_t"][::std::mem::size_of::<bspoanode_t>() - 56usize];
    ["Alignment of bspoanode_t"][::std::mem::align_of::<bspoanode_t>() - 4usize];
    ["Offset of field: bspoanode_t::rid"][::std::mem::offset_of!(bspoanode_t, rid) - 0usize];
    ["Offset of field: bspoanode_t::vst"][::std::mem::offset_of!(bspoanode_t, vst) - 4usize];
    ["Offset of field: bspoanode_t::nin"][::std::mem::offset_of!(bspoanode_t, nin) - 6usize];
    ["Offset of field: bspoanode_t::nou"][::std::mem::offset_of!(bspoanode_t, nou) - 8usize];
    ["Offset of field: bspoanode_t::nct"][::std::mem::offset_of!(bspoanode_t, nct) - 10usize];
    ["Offset of field: bspoanode_t::cov"][::std::mem::offset_of!(bspoanode_t, cov) - 12usize];
    ["Offset of field: bspoanode_t::pos"][::std::mem::offset_of!(bspoanode_t, pos) - 16usize];
    ["Offset of field: bspoanode_t::mpos"][::std::mem::offset_of!(bspoanode_t, mpos) - 20usize];
    ["Offset of field: bspoanode_t::cpos"][::std::mem::offset_of!(bspoanode_t, cpos) - 24usize];
    ["Offset of field: bspoanode_t::rpos"][::std::mem::offset_of!(bspoanode_t, rpos) - 28usize];
    ["Offset of field: bspoanode_t::edge"][::std::mem::offset_of!(bspoanode_t, edge) - 32usize];
    ["Offset of field: bspoanode_t::erev"][::std::mem::offset_of!(bspoanode_t, erev) - 36usize];
    ["Offset of field: bspoanode_t::next"][::std::mem::offset_of!(bspoanode_t, next) - 40usize];
    ["Offset of field: bspoanode_t::prev"][::std::mem::offset_of!(bspoanode_t, prev) - 44usize];
    ["Offset of field: bspoanode_t::header"][::std::mem::offset_of!(bspoanode_t, header) - 48usize];
    ["Offset of field: bspoanode_t::mmidx"][::std::mem::offset_of!(bspoanode_t, mmidx) - 52usize];
};
impl bspoanode_t {
    #[inline]
    pub fn base(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u8) }
    }
    #[inline]
    pub fn set_base(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn base_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                7u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_base_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                7u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn ref_(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_ref(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn ref__raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_ref_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aux(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 2u8) as u8) }
    }
    #[inline]
    pub fn set_aux(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aux_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                8usize,
                2u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_aux_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                8usize,
                2u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn colorful(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_colorful(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn colorful_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                10usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_colorful_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                10usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn inuse(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_inuse(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn inuse_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                11usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_inuse_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                11usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bonus(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(12usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_bonus(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bonus_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                12usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_bonus_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                12usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bless(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_bless(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bless_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_bless_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rdc(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_rdc(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rdc_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                14usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_rdc_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                14usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn rdd(&self) -> u1i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u8) }
    }
    #[inline]
    pub fn set_rdd(&mut self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn rdd_raw(this: *const Self) -> u1i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 2usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                15usize,
                1u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_rdd_raw(this: *mut Self, val: u1i) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 2usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                15usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        base: u1i,
        ref_: u1i,
        aux: u1i,
        colorful: u1i,
        inuse: u1i,
        bonus: u1i,
        bless: u1i,
        rdc: u1i,
        rdd: u1i,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let base: u8 = unsafe { ::std::mem::transmute(base) };
            base as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let ref_: u8 = unsafe { ::std::mem::transmute(ref_) };
            ref_ as u64
        });
        __bindgen_bitfield_unit.set(8usize, 2u8, {
            let aux: u8 = unsafe { ::std::mem::transmute(aux) };
            aux as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let colorful: u8 = unsafe { ::std::mem::transmute(colorful) };
            colorful as u64
        });
        __bindgen_bitfield_unit.set(11usize, 1u8, {
            let inuse: u8 = unsafe { ::std::mem::transmute(inuse) };
            inuse as u64
        });
        __bindgen_bitfield_unit.set(12usize, 1u8, {
            let bonus: u8 = unsafe { ::std::mem::transmute(bonus) };
            bonus as u64
        });
        __bindgen_bitfield_unit.set(13usize, 1u8, {
            let bless: u8 = unsafe { ::std::mem::transmute(bless) };
            bless as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let rdc: u8 = unsafe { ::std::mem::transmute(rdc) };
            rdc as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let rdd: u8 = unsafe { ::std::mem::transmute(rdd) };
            rdd as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bspoanodev {
    pub buffer: *mut bspoanode_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoanodev"][::std::mem::size_of::<bspoanodev>() - 32usize];
    ["Alignment of bspoanodev"][::std::mem::align_of::<bspoanodev>() - 8usize];
    ["Offset of field: bspoanodev::buffer"][::std::mem::offset_of!(bspoanodev, buffer) - 0usize];
    ["Offset of field: bspoanodev::size"][::std::mem::offset_of!(bspoanodev, size) - 8usize];
    ["Offset of field: bspoanodev::cap"][::std::mem::offset_of!(bspoanodev, cap) - 16usize];
};
impl Default for bspoanodev {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl bspoanodev {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static bspoanodev_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoaedge_t {
    pub node: u4i,
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub next: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoaedge_t"][::std::mem::size_of::<bspoaedge_t>() - 12usize];
    ["Alignment of bspoaedge_t"][::std::mem::align_of::<bspoaedge_t>() - 4usize];
    ["Offset of field: bspoaedge_t::node"][::std::mem::offset_of!(bspoaedge_t, node) - 0usize];
    ["Offset of field: bspoaedge_t::next"][::std::mem::offset_of!(bspoaedge_t, next) - 8usize];
};
impl bspoaedge_t {
    #[inline]
    pub fn cov(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_cov(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn cov_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                31u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_cov_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                31u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn vst(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(31usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_vst(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(31usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn vst_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                31usize,
                1u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_vst_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                31usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(cov: u4i, vst: u4i) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 31u8, {
            let cov: u32 = unsafe { ::std::mem::transmute(cov) };
            cov as u64
        });
        __bindgen_bitfield_unit.set(31usize, 1u8, {
            let vst: u32 = unsafe { ::std::mem::transmute(vst) };
            vst as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bspoaedgev {
    pub buffer: *mut bspoaedge_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoaedgev"][::std::mem::size_of::<bspoaedgev>() - 32usize];
    ["Alignment of bspoaedgev"][::std::mem::align_of::<bspoaedgev>() - 8usize];
    ["Offset of field: bspoaedgev::buffer"][::std::mem::offset_of!(bspoaedgev, buffer) - 0usize];
    ["Offset of field: bspoaedgev::size"][::std::mem::offset_of!(bspoaedgev, size) - 8usize];
    ["Offset of field: bspoaedgev::cap"][::std::mem::offset_of!(bspoaedgev, cap) - 16usize];
};
impl Default for bspoaedgev {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl bspoaedgev {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static bspoaedgev_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoaedge_chg_t {
    pub node1: u4i,
    pub node2: u4i,
    pub cov: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoaedge_chg_t"][::std::mem::size_of::<bspoaedge_chg_t>() - 12usize];
    ["Alignment of bspoaedge_chg_t"][::std::mem::align_of::<bspoaedge_chg_t>() - 4usize];
    ["Offset of field: bspoaedge_chg_t::node1"]
        [::std::mem::offset_of!(bspoaedge_chg_t, node1) - 0usize];
    ["Offset of field: bspoaedge_chg_t::node2"]
        [::std::mem::offset_of!(bspoaedge_chg_t, node2) - 4usize];
    ["Offset of field: bspoaedge_chg_t::cov"]
        [::std::mem::offset_of!(bspoaedge_chg_t, cov) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bspoaedgechgv {
    pub buffer: *mut bspoaedge_chg_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoaedgechgv"][::std::mem::size_of::<bspoaedgechgv>() - 32usize];
    ["Alignment of bspoaedgechgv"][::std::mem::align_of::<bspoaedgechgv>() - 8usize];
    ["Offset of field: bspoaedgechgv::buffer"]
        [::std::mem::offset_of!(bspoaedgechgv, buffer) - 0usize];
    ["Offset of field: bspoaedgechgv::size"][::std::mem::offset_of!(bspoaedgechgv, size) - 8usize];
    ["Offset of field: bspoaedgechgv::cap"][::std::mem::offset_of!(bspoaedgechgv, cap) - 16usize];
};
impl Default for bspoaedgechgv {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl bspoaedgechgv {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static bspoaedgechgv_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BSPOAPar {
    pub refmode: ::std::os::raw::c_int,
    pub shuffle: ::std::os::raw::c_int,
    pub alnmode: ::std::os::raw::c_int,
    pub realn: ::std::os::raw::c_int,
    pub seqcore: u4i,
    pub nrec: ::std::os::raw::c_int,
    pub ksz: ::std::os::raw::c_int,
    pub bwtrigger: ::std::os::raw::c_int,
    pub bandwidth: ::std::os::raw::c_int,
    pub M: ::std::os::raw::c_int,
    pub X: ::std::os::raw::c_int,
    pub O: ::std::os::raw::c_int,
    pub E: ::std::os::raw::c_int,
    pub Q: ::std::os::raw::c_int,
    pub P: ::std::os::raw::c_int,
    pub T: ::std::os::raw::c_int,
    pub refbonus: ::std::os::raw::c_int,
    pub editbw: ::std::os::raw::c_int,
    pub althi: ::std::os::raw::c_int,
    pub qlthi: ::std::os::raw::c_int,
    pub psub: f32,
    pub pins: f32,
    pub pdel: f32,
    pub piex: f32,
    pub pdex: f32,
    pub hins: f32,
    pub hdel: f32,
    pub min_varcnt: ::std::os::raw::c_int,
    pub min_covfrq: f32,
    pub min_snvqlt: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BSPOAPar"][::std::mem::size_of::<BSPOAPar>() - 120usize];
    ["Alignment of BSPOAPar"][::std::mem::align_of::<BSPOAPar>() - 4usize];
    ["Offset of field: BSPOAPar::refmode"][::std::mem::offset_of!(BSPOAPar, refmode) - 0usize];
    ["Offset of field: BSPOAPar::shuffle"][::std::mem::offset_of!(BSPOAPar, shuffle) - 4usize];
    ["Offset of field: BSPOAPar::alnmode"][::std::mem::offset_of!(BSPOAPar, alnmode) - 8usize];
    ["Offset of field: BSPOAPar::realn"][::std::mem::offset_of!(BSPOAPar, realn) - 12usize];
    ["Offset of field: BSPOAPar::seqcore"][::std::mem::offset_of!(BSPOAPar, seqcore) - 16usize];
    ["Offset of field: BSPOAPar::nrec"][::std::mem::offset_of!(BSPOAPar, nrec) - 20usize];
    ["Offset of field: BSPOAPar::ksz"][::std::mem::offset_of!(BSPOAPar, ksz) - 24usize];
    ["Offset of field: BSPOAPar::bwtrigger"][::std::mem::offset_of!(BSPOAPar, bwtrigger) - 28usize];
    ["Offset of field: BSPOAPar::bandwidth"][::std::mem::offset_of!(BSPOAPar, bandwidth) - 32usize];
    ["Offset of field: BSPOAPar::M"][::std::mem::offset_of!(BSPOAPar, M) - 36usize];
    ["Offset of field: BSPOAPar::X"][::std::mem::offset_of!(BSPOAPar, X) - 40usize];
    ["Offset of field: BSPOAPar::O"][::std::mem::offset_of!(BSPOAPar, O) - 44usize];
    ["Offset of field: BSPOAPar::E"][::std::mem::offset_of!(BSPOAPar, E) - 48usize];
    ["Offset of field: BSPOAPar::Q"][::std::mem::offset_of!(BSPOAPar, Q) - 52usize];
    ["Offset of field: BSPOAPar::P"][::std::mem::offset_of!(BSPOAPar, P) - 56usize];
    ["Offset of field: BSPOAPar::T"][::std::mem::offset_of!(BSPOAPar, T) - 60usize];
    ["Offset of field: BSPOAPar::refbonus"][::std::mem::offset_of!(BSPOAPar, refbonus) - 64usize];
    ["Offset of field: BSPOAPar::editbw"][::std::mem::offset_of!(BSPOAPar, editbw) - 68usize];
    ["Offset of field: BSPOAPar::althi"][::std::mem::offset_of!(BSPOAPar, althi) - 72usize];
    ["Offset of field: BSPOAPar::qlthi"][::std::mem::offset_of!(BSPOAPar, qlthi) - 76usize];
    ["Offset of field: BSPOAPar::psub"][::std::mem::offset_of!(BSPOAPar, psub) - 80usize];
    ["Offset of field: BSPOAPar::pins"][::std::mem::offset_of!(BSPOAPar, pins) - 84usize];
    ["Offset of field: BSPOAPar::pdel"][::std::mem::offset_of!(BSPOAPar, pdel) - 88usize];
    ["Offset of field: BSPOAPar::piex"][::std::mem::offset_of!(BSPOAPar, piex) - 92usize];
    ["Offset of field: BSPOAPar::pdex"][::std::mem::offset_of!(BSPOAPar, pdex) - 96usize];
    ["Offset of field: BSPOAPar::hins"][::std::mem::offset_of!(BSPOAPar, hins) - 100usize];
    ["Offset of field: BSPOAPar::hdel"][::std::mem::offset_of!(BSPOAPar, hdel) - 104usize];
    ["Offset of field: BSPOAPar::min_varcnt"]
        [::std::mem::offset_of!(BSPOAPar, min_varcnt) - 108usize];
    ["Offset of field: BSPOAPar::min_covfrq"]
        [::std::mem::offset_of!(BSPOAPar, min_covfrq) - 112usize];
    ["Offset of field: BSPOAPar::min_snvqlt"]
        [::std::mem::offset_of!(BSPOAPar, min_snvqlt) - 116usize];
};
unsafe extern "C" {
    pub static DEFAULT_BSPOA_PAR: BSPOAPar;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoacns_t {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub max: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoacns_t"][::std::mem::size_of::<bspoacns_t>() - 8usize];
    ["Alignment of bspoacns_t"][::std::mem::align_of::<bspoacns_t>() - 4usize];
    ["Offset of field: bspoacns_t::max"][::std::mem::offset_of!(bspoacns_t, max) - 4usize];
};
impl bspoacns_t {
    #[inline]
    pub fn coff(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 29u8) as u32) }
    }
    #[inline]
    pub fn set_coff(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 29u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn coff_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                29u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_coff_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                29u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bt(&self) -> u4i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(29usize, 3u8) as u32) }
    }
    #[inline]
    pub fn set_bt(&mut self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(29usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bt_raw(this: *const Self) -> u4i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 4usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                29usize,
                3u8,
            ) as u32)
        }
    }
    #[inline]
    pub unsafe fn set_bt_raw(this: *mut Self, val: u4i) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 4usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                29usize,
                3u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(coff: u4i, bt: u4i) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 29u8, {
            let coff: u32 = unsafe { ::std::mem::transmute(coff) };
            coff as u64
        });
        __bindgen_bitfield_unit.set(29usize, 3u8, {
            let bt: u32 = unsafe { ::std::mem::transmute(bt) };
            bt as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoalsp_t {
    pub rid: u2i,
    pub rbeg: u4i,
    pub mbeg: u4i,
    pub rlen: u2i,
    pub mlen: u2i,
    pub scr: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoalsp_t"][::std::mem::size_of::<bspoalsp_t>() - 20usize];
    ["Alignment of bspoalsp_t"][::std::mem::align_of::<bspoalsp_t>() - 4usize];
    ["Offset of field: bspoalsp_t::rid"][::std::mem::offset_of!(bspoalsp_t, rid) - 0usize];
    ["Offset of field: bspoalsp_t::rbeg"][::std::mem::offset_of!(bspoalsp_t, rbeg) - 4usize];
    ["Offset of field: bspoalsp_t::mbeg"][::std::mem::offset_of!(bspoalsp_t, mbeg) - 8usize];
    ["Offset of field: bspoalsp_t::rlen"][::std::mem::offset_of!(bspoalsp_t, rlen) - 12usize];
    ["Offset of field: bspoalsp_t::mlen"][::std::mem::offset_of!(bspoalsp_t, mlen) - 14usize];
    ["Offset of field: bspoalsp_t::scr"][::std::mem::offset_of!(bspoalsp_t, scr) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bspoalspv {
    pub buffer: *mut bspoalsp_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoalspv"][::std::mem::size_of::<bspoalspv>() - 32usize];
    ["Alignment of bspoalspv"][::std::mem::align_of::<bspoalspv>() - 8usize];
    ["Offset of field: bspoalspv::buffer"][::std::mem::offset_of!(bspoalspv, buffer) - 0usize];
    ["Offset of field: bspoalspv::size"][::std::mem::offset_of!(bspoalspv, size) - 8usize];
    ["Offset of field: bspoalspv::cap"][::std::mem::offset_of!(bspoalspv, cap) - 16usize];
};
impl Default for bspoalspv {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl bspoalspv {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static bspoalspv_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bspoavar_t {
    pub cpos: u4i,
    pub mpos: u4i,
    pub refn: u2i,
    pub altn: u2i,
    pub covn: u2i,
    pub refb: u1i,
    pub altb: u1i,
    pub qual: u4i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoavar_t"][::std::mem::size_of::<bspoavar_t>() - 20usize];
    ["Alignment of bspoavar_t"][::std::mem::align_of::<bspoavar_t>() - 4usize];
    ["Offset of field: bspoavar_t::cpos"][::std::mem::offset_of!(bspoavar_t, cpos) - 0usize];
    ["Offset of field: bspoavar_t::mpos"][::std::mem::offset_of!(bspoavar_t, mpos) - 4usize];
    ["Offset of field: bspoavar_t::refn"][::std::mem::offset_of!(bspoavar_t, refn) - 8usize];
    ["Offset of field: bspoavar_t::altn"][::std::mem::offset_of!(bspoavar_t, altn) - 10usize];
    ["Offset of field: bspoavar_t::covn"][::std::mem::offset_of!(bspoavar_t, covn) - 12usize];
    ["Offset of field: bspoavar_t::refb"][::std::mem::offset_of!(bspoavar_t, refb) - 14usize];
    ["Offset of field: bspoavar_t::altb"][::std::mem::offset_of!(bspoavar_t, altb) - 15usize];
    ["Offset of field: bspoavar_t::qual"][::std::mem::offset_of!(bspoavar_t, qual) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bspoavarv {
    pub buffer: *mut bspoavar_t,
    pub size: u8i,
    pub cap: u8i,
    pub _bitfield_align_1: [u64; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of bspoavarv"][::std::mem::size_of::<bspoavarv>() - 32usize];
    ["Alignment of bspoavarv"][::std::mem::align_of::<bspoavarv>() - 8usize];
    ["Offset of field: bspoavarv::buffer"][::std::mem::offset_of!(bspoavarv, buffer) - 0usize];
    ["Offset of field: bspoavarv::size"][::std::mem::offset_of!(bspoavarv, size) - 8usize];
    ["Offset of field: bspoavarv::cap"][::std::mem::offset_of!(bspoavarv, cap) - 16usize];
};
impl Default for bspoavarv {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl bspoavarv {
    #[inline]
    pub fn mem_zero(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u64) }
    }
    #[inline]
    pub fn set_mem_zero(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn mem_zero_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                1u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_mem_zero_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                1u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn n_head(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_n_head(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn n_head_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                1usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_n_head_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                1usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn aligned(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 6u8) as u64) }
    }
    #[inline]
    pub fn set_aligned(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 6u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn aligned_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                7usize,
                6u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_aligned_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                7usize,
                6u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn off(&self) -> u8i {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(13usize, 51u8) as u64) }
    }
    #[inline]
    pub fn set_off(&mut self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(13usize, 51u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn off_raw(this: *const Self) -> u8i {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 8usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                13usize,
                51u8,
            ) as u64)
        }
    }
    #[inline]
    pub unsafe fn set_off_raw(this: *mut Self, val: u8i) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 8usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                13usize,
                51u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        mem_zero: u8i,
        n_head: u8i,
        aligned: u8i,
        off: u8i,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let mem_zero: u64 = unsafe { ::std::mem::transmute(mem_zero) };
            mem_zero as u64
        });
        __bindgen_bitfield_unit.set(1usize, 6u8, {
            let n_head: u64 = unsafe { ::std::mem::transmute(n_head) };
            n_head as u64
        });
        __bindgen_bitfield_unit.set(7usize, 6u8, {
            let aligned: u64 = unsafe { ::std::mem::transmute(aligned) };
            aligned as u64
        });
        __bindgen_bitfield_unit.set(13usize, 51u8, {
            let off: u64 = unsafe { ::std::mem::transmute(off) };
            off as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    pub static bspoavarv_obj_desc: obj_desc_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BSPOA {
    pub mtag: *mut String,
    pub seqs: *mut SeqBank,
    pub ords: *mut u4v,
    pub ndoffs: *mut u4v,
    pub keep_seqs: ::std::os::raw::c_int,
    pub cigars: *mut u4v,
    pub cgbs: *mut u4v,
    pub cges: *mut u4v,
    pub HEAD: u4i,
    pub TAIL: u4i,
    pub nodes: *mut bspoanodev,
    pub edges: *mut bspoaedgev,
    pub ecycs: *mut u4v,
    pub hash: *mut uuhash,
    pub par: *mut BSPOAPar,
    pub piecewise: ::std::os::raw::c_int,
    pub bandwidth: u4i,
    pub qlen: u4i,
    pub slen: u4i,
    pub qb: u4i,
    pub qe: u4i,
    pub nmsa: u4i,
    pub nrds: u4i,
    pub qseq: *mut u1v,
    pub matrix: [[b1i; 16usize]; 4usize],
    pub qprof: [*mut b1v; 4usize],
    pub memp: *mut b1v,
    pub mmblk: u8i,
    pub mmcnt: u8i,
    pub maxscr: ::std::os::raw::c_int,
    pub maxidx: ::std::os::raw::c_int,
    pub maxoff: ::std::os::raw::c_int,
    pub sels: *mut u4v,
    pub rdregs: [*mut b4v; 2usize],
    pub rdbits: *mut BitVec,
    pub states: *mut BitVec,
    pub heap: *mut u8v,
    pub todels: *mut u8v,
    pub stack: *mut u4v,
    pub echgs: *mut bspoaedgechgv,
    pub backbone: u4i,
    pub msacols: *mut u1v,
    pub msaidxs: *mut u4v,
    pub dpvals: [f64; 8usize],
    pub dporis: [f64; 8usize],
    pub dptable: *mut u1i,
    pub cns: *mut u1v,
    pub qlt: *mut u1v,
    pub alt: *mut u1v,
    pub lsp: *mut bspoalspv,
    pub var: *mut bspoavarv,
    pub strs: *mut String,
    pub ncall: u8i,
    pub obj_uid: u8i,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of BSPOA"][::std::mem::size_of::<BSPOA>() - 584usize];
    ["Alignment of BSPOA"][::std::mem::align_of::<BSPOA>() - 8usize];
    ["Offset of field: BSPOA::mtag"][::std::mem::offset_of!(BSPOA, mtag) - 0usize];
    ["Offset of field: BSPOA::seqs"][::std::mem::offset_of!(BSPOA, seqs) - 8usize];
    ["Offset of field: BSPOA::ords"][::std::mem::offset_of!(BSPOA, ords) - 16usize];
    ["Offset of field: BSPOA::ndoffs"][::std::mem::offset_of!(BSPOA, ndoffs) - 24usize];
    ["Offset of field: BSPOA::keep_seqs"][::std::mem::offset_of!(BSPOA, keep_seqs) - 32usize];
    ["Offset of field: BSPOA::cigars"][::std::mem::offset_of!(BSPOA, cigars) - 40usize];
    ["Offset of field: BSPOA::cgbs"][::std::mem::offset_of!(BSPOA, cgbs) - 48usize];
    ["Offset of field: BSPOA::cges"][::std::mem::offset_of!(BSPOA, cges) - 56usize];
    ["Offset of field: BSPOA::HEAD"][::std::mem::offset_of!(BSPOA, HEAD) - 64usize];
    ["Offset of field: BSPOA::TAIL"][::std::mem::offset_of!(BSPOA, TAIL) - 68usize];
    ["Offset of field: BSPOA::nodes"][::std::mem::offset_of!(BSPOA, nodes) - 72usize];
    ["Offset of field: BSPOA::edges"][::std::mem::offset_of!(BSPOA, edges) - 80usize];
    ["Offset of field: BSPOA::ecycs"][::std::mem::offset_of!(BSPOA, ecycs) - 88usize];
    ["Offset of field: BSPOA::hash"][::std::mem::offset_of!(BSPOA, hash) - 96usize];
    ["Offset of field: BSPOA::par"][::std::mem::offset_of!(BSPOA, par) - 104usize];
    ["Offset of field: BSPOA::piecewise"][::std::mem::offset_of!(BSPOA, piecewise) - 112usize];
    ["Offset of field: BSPOA::bandwidth"][::std::mem::offset_of!(BSPOA, bandwidth) - 116usize];
    ["Offset of field: BSPOA::qlen"][::std::mem::offset_of!(BSPOA, qlen) - 120usize];
    ["Offset of field: BSPOA::slen"][::std::mem::offset_of!(BSPOA, slen) - 124usize];
    ["Offset of field: BSPOA::qb"][::std::mem::offset_of!(BSPOA, qb) - 128usize];
    ["Offset of field: BSPOA::qe"][::std::mem::offset_of!(BSPOA, qe) - 132usize];
    ["Offset of field: BSPOA::nmsa"][::std::mem::offset_of!(BSPOA, nmsa) - 136usize];
    ["Offset of field: BSPOA::nrds"][::std::mem::offset_of!(BSPOA, nrds) - 140usize];
    ["Offset of field: BSPOA::qseq"][::std::mem::offset_of!(BSPOA, qseq) - 144usize];
    ["Offset of field: BSPOA::matrix"][::std::mem::offset_of!(BSPOA, matrix) - 152usize];
    ["Offset of field: BSPOA::qprof"][::std::mem::offset_of!(BSPOA, qprof) - 216usize];
    ["Offset of field: BSPOA::memp"][::std::mem::offset_of!(BSPOA, memp) - 248usize];
    ["Offset of field: BSPOA::mmblk"][::std::mem::offset_of!(BSPOA, mmblk) - 256usize];
    ["Offset of field: BSPOA::mmcnt"][::std::mem::offset_of!(BSPOA, mmcnt) - 264usize];
    ["Offset of field: BSPOA::maxscr"][::std::mem::offset_of!(BSPOA, maxscr) - 272usize];
    ["Offset of field: BSPOA::maxidx"][::std::mem::offset_of!(BSPOA, maxidx) - 276usize];
    ["Offset of field: BSPOA::maxoff"][::std::mem::offset_of!(BSPOA, maxoff) - 280usize];
    ["Offset of field: BSPOA::sels"][::std::mem::offset_of!(BSPOA, sels) - 288usize];
    ["Offset of field: BSPOA::rdregs"][::std::mem::offset_of!(BSPOA, rdregs) - 296usize];
    ["Offset of field: BSPOA::rdbits"][::std::mem::offset_of!(BSPOA, rdbits) - 312usize];
    ["Offset of field: BSPOA::states"][::std::mem::offset_of!(BSPOA, states) - 320usize];
    ["Offset of field: BSPOA::heap"][::std::mem::offset_of!(BSPOA, heap) - 328usize];
    ["Offset of field: BSPOA::todels"][::std::mem::offset_of!(BSPOA, todels) - 336usize];
    ["Offset of field: BSPOA::stack"][::std::mem::offset_of!(BSPOA, stack) - 344usize];
    ["Offset of field: BSPOA::echgs"][::std::mem::offset_of!(BSPOA, echgs) - 352usize];
    ["Offset of field: BSPOA::backbone"][::std::mem::offset_of!(BSPOA, backbone) - 360usize];
    ["Offset of field: BSPOA::msacols"][::std::mem::offset_of!(BSPOA, msacols) - 368usize];
    ["Offset of field: BSPOA::msaidxs"][::std::mem::offset_of!(BSPOA, msaidxs) - 376usize];
    ["Offset of field: BSPOA::dpvals"][::std::mem::offset_of!(BSPOA, dpvals) - 384usize];
    ["Offset of field: BSPOA::dporis"][::std::mem::offset_of!(BSPOA, dporis) - 448usize];
    ["Offset of field: BSPOA::dptable"][::std::mem::offset_of!(BSPOA, dptable) - 512usize];
    ["Offset of field: BSPOA::cns"][::std::mem::offset_of!(BSPOA, cns) - 520usize];
    ["Offset of field: BSPOA::qlt"][::std::mem::offset_of!(BSPOA, qlt) - 528usize];
    ["Offset of field: BSPOA::alt"][::std::mem::offset_of!(BSPOA, alt) - 536usize];
    ["Offset of field: BSPOA::lsp"][::std::mem::offset_of!(BSPOA, lsp) - 544usize];
    ["Offset of field: BSPOA::var"][::std::mem::offset_of!(BSPOA, var) - 552usize];
    ["Offset of field: BSPOA::strs"][::std::mem::offset_of!(BSPOA, strs) - 560usize];
    ["Offset of field: BSPOA::ncall"][::std::mem::offset_of!(BSPOA, ncall) - 568usize];
    ["Offset of field: BSPOA::obj_uid"][::std::mem::offset_of!(BSPOA, obj_uid) - 576usize];
};
impl Default for BSPOA {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub const _log_caches_n: u4i = 1;
unsafe extern "C" {
    pub static mut _log_caches: [f64; 1001usize];
}
pub const print_probs: ::std::os::raw::c_int = 0;
unsafe extern "C" {
    pub static mut MM_EPI8_ALL0: xint;
}
unsafe extern "C" {
    pub static mut MM_EPI8_ALL1: xint;
}
unsafe extern "C" {
    pub static mut MM_EPI8_ALL2: xint;
}
unsafe extern "C" {
    pub static mut MM_EPI8_ALL3: xint;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct c_file_t {
    pub path: *mut ::std::os::raw::c_char,
    pub fp: *mut FILE,
    pub closeable: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of c_file_t"][::std::mem::size_of::<c_file_t>() - 24usize];
    ["Alignment of c_file_t"][::std::mem::align_of::<c_file_t>() - 8usize];
    ["Offset of field: c_file_t::path"][::std::mem::offset_of!(c_file_t, path) - 0usize];
    ["Offset of field: c_file_t::fp"][::std::mem::offset_of!(c_file_t, fp) - 8usize];
    ["Offset of field: c_file_t::closeable"][::std::mem::offset_of!(c_file_t, closeable) - 16usize];
};
impl Default for c_file_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
unsafe extern "C" {
    pub fn c_file_open(path: *const ::std::os::raw::c_char) -> *mut c_file_t;
}
unsafe extern "C" {
    pub fn c_file_close(file: *mut c_file_t);
}
unsafe extern "C" {
    pub fn bs_epi8_seqalign_set_score_matrix(m: *mut i8, match_: i8, mismatch: i8);
}
unsafe extern "C" {
    pub fn bs_bs_epi8_seqalign_pairwise(
        qseq: *mut u8,
        qlen: u32,
        tseq: *mut u8,
        tlen: u32,
        mempool: *mut b1v,
        cigars: *mut u4v,
        mode: ::std::os::raw::c_int,
        bandwidth: u32,
        m: *const i8,
        gap_open: i8,
        gap_ext: i8,
        gap_open2: i8,
        gap_ext2: i8,
        verbose: ::std::os::raw::c_int,
    ) -> seqalign_result_t;
}
unsafe extern "C" {
    pub fn bs_s_seqedit_pairwise(
        qseq: *mut u8,
        qlen: u32,
        tseq: *mut u8,
        tlen: u32,
        mode: ::std::os::raw::c_int,
        bandwidth: u32,
        mempool: *mut b1v,
        cigars: *mut u4v,
        verbose: ::std::os::raw::c_int,
    ) -> seqalign_result_t;
}
unsafe extern "C" {
    pub fn bs_ks_seqedit_pairwise(
        ksize: u8,
        qseq: *mut u8,
        qlen: u32,
        tseq: *mut u8,
        tlen: u32,
        mempool: *mut b1v,
        cigars: *mut u4v,
        verbose: ::std::os::raw::c_int,
    ) -> seqalign_result_t;
}
unsafe extern "C" {
    pub fn bs_seqalign_cigar2alnstr(
        qseq: *mut u1i,
        tseq: *mut u1i,
        rs: *mut seqalign_result_t,
        cigars: *mut u4v,
        alnstr: *mut *mut ::std::os::raw::c_char,
        alnlen: ::std::os::raw::c_int,
    );
}
unsafe extern "C" {
    pub fn mempool_init(
        size: usize,
        value: ::std::os::raw::c_int,
        nhead: ::std::os::raw::c_int,
    ) -> *mut b1v;
}
unsafe extern "C" {
    pub fn mempool_free(mempool: *mut b1v);
}
unsafe extern "C" {
    pub fn mempool_clear(mempool: *mut b1v);
}
unsafe extern "C" {
    pub fn cigars_init(size: usize) -> *mut u4v;
}
unsafe extern "C" {
    pub fn cigars_free(cigars: *mut u4v);
}
unsafe extern "C" {
    pub fn cigars_clear(cigars: *mut u4v);
}
unsafe extern "C" {
    pub fn bs_u1v_init(size: usize) -> *mut u1v;
}
unsafe extern "C" {
    pub fn bs_u1v_clear(v: *mut u1v);
}
unsafe extern "C" {
    pub fn bs_u1v_free(v: *mut u1v);
}
unsafe extern "C" {
    pub fn bs_u1v_clear_and_encap(v: *mut u1v, size: usize);
}
unsafe extern "C" {
    pub fn create_bits_from_seq(bits: *mut u1v, seq: *const u8, len: u4i);
}
unsafe extern "C" {
    pub fn bspoa_init(poa_params: BSPOAPar) -> *mut BSPOA;
}
unsafe extern "C" {
    pub fn bspoa_free(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_clear(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_add_sequence(poa: *mut BSPOA, seq: *const u8, len: u4i);
}
unsafe extern "C" {
    pub fn bspoa_cns(poa: *mut BSPOA) -> f64;
}
unsafe extern "C" {
    pub fn bspoa_simple_cns(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_tidy_msa(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_call_snvs(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_print_snvs(poa: *mut BSPOA, label: *mut ::std::os::raw::c_char, fp: *mut c_file_t);
}
unsafe extern "C" {
    pub fn bspoa_print_msa(
        poa: *mut BSPOA,
        label: *mut ::std::os::raw::c_char,
        mbeg: u4i,
        mend: u4i,
        linewidth: u4i,
        colorful: ::std::os::raw::c_int,
        fp: *mut c_file_t,
    );
}
unsafe extern "C" {
    pub fn bspoa_begin(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_end(poa: *mut BSPOA);
}
unsafe extern "C" {
    pub fn bspoa_dump_binary_msa(
        poa: *mut BSPOA,
        metadata: *mut ::std::os::raw::c_char,
        metalen: u4i,
        filename: *const ::std::os::raw::c_char,
    );
}
unsafe extern "C" {
    pub fn bspoa_load_binary_msa(
        poa: *mut BSPOA,
        filename: *const ::std::os::raw::c_char,
        metadata: *mut String,
    ) -> ::std::os::raw::c_int;
}
unsafe extern "C" {
    pub fn bspoa_get_cns(poa: *mut BSPOA, retlen: *mut u4i) -> *mut u1i;
}
unsafe extern "C" {
    pub fn bspoa_get_qlt(poa: *mut BSPOA, retlen: *mut u4i) -> *mut u1i;
}
unsafe extern "C" {
    pub fn bspoa_get_alt(poa: *mut BSPOA, retlen: *mut u4i) -> *mut u1i;
}
unsafe extern "C" {
    pub fn string_free(s: *mut String);
}
unsafe extern "C" {
    pub fn string_init(size: usize) -> *mut String;
}
unsafe extern "C" {
    pub fn bsalign_version() -> *const ::std::os::raw::c_char;
}
unsafe extern "C" {
    pub fn bspoa_get_rid_alignment(
        g: *mut BSPOA,
        rid: ::std::os::raw::c_int,
        len: *mut usize,
    ) -> *mut ::std::os::raw::c_uchar;
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
    ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
    ["Offset of field: __va_list_tag::gp_offset"]
        [::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
    ["Offset of field: __va_list_tag::fp_offset"]
        [::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
    ["Offset of field: __va_list_tag::overflow_arg_area"]
        [::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
    ["Offset of field: __va_list_tag::reg_save_area"]
        [::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};
impl Default for __va_list_tag {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
