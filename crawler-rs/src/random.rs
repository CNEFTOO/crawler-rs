use rand::Rng;

const LETTER_BYTES: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const LETTER_IDX_BITS: u32 = 6; // 每个字符索引需要的位数
const LETTER_IDX_MASK: u64 = (1 << LETTER_IDX_BITS) - 1; // 用于掩码字符索引
const LETTER_IDX_MAX: usize = 63 / LETTER_IDX_BITS as usize; // 每次生成的随机数能提供的字符数量

fn rand_seq(n: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(n);

    // 生成随机种子
    let mut cache = rng.gen::<u64>();
    let mut remain = LETTER_IDX_MAX;

    for _ in 0..n {
        if remain == 0 {
            cache = rng.gen::<u64>();
            remain = LETTER_IDX_MAX;
        }

        // 取低 6 位
        let idx = (cache & LETTER_IDX_MASK) as usize;
        if idx < LETTER_BYTES.len() {
            result.push(LETTER_BYTES[idx] as char);
        }

        cache >>= LETTER_IDX_BITS;
        remain -= 1;
    }

    result
}