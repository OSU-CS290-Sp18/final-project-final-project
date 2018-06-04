macro_rules! consts {
    ($ty:ty, [$($name:ident = $value:expr)+]) => {
        $(
            pub(super) const $name: $ty = $value;
        )+
    };
}

consts! { u8, [
    DEFAULT_FLOAT_BITS = 32

    MAX_INT_LENGTH = 64

    CHR_LIST    = 59
    CHR_DICT    = 60
    CHR_INT     = 61
    CHR_INT1    = 62
    CHR_INT2    = 63
    CHR_INT4    = 64
    CHR_INT8    = 65
    CHR_FLOAT32 = 66
    CHR_FLOAT64 = 44
    CHR_TRUE    = 67
    CHR_FALSE   = 68
    CHR_NONE    = 69
    CHR_TERM    = 127

    DICT_FIXED_START = 102
    DICT_FIXED_COUNT = 25

    INT_POS_FIXED_START = 0
    INT_POS_FIXED_COUNT = 44

    INT_NEG_FIXED_START = 70
    INT_NEG_FIXED_COUNT = 32

    STR_FIXED_START = 128
    STR_FIXED_COUNT = 64

    LIST_FIXED_START = STR_FIXED_START + STR_FIXED_COUNT
    LIST_FIXED_COUNT = 64
] }
