pub mod application;

fn main() {
	// アプリケーションのインスタンス
	let app = application::application::Application {};
	// タイムスタンプ
	let current_timestamp = application::mylib::util::Util::get_current_timestamp();
	println!("[TRACE] {}", current_timestamp);
	// サーバーを起動
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
