pub struct Util;

impl Util {
	/// システムのタイムスタンプを文字列で返します。
	pub fn get_current_timestamp() -> String {
		let date = chrono::Local::now();
		return format!("{}", date.format("%Y-%m-%d %H:%M:%S%.3f"));
	}
}
