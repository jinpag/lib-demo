use rand::Rng;

pub fn gen_ran() -> u8 {
	let mut rng = rand::thread_rng(); // 乱数生成器を生成する
	let n: u8 = rng.gen();
	n
}
