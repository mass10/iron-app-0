mod application;

fn main() {
	let app = application::Application {};
	let result = app.run();
	if result.is_err() {
		println!("[ERROR] {}", result.err().unwrap());
		return;
	}
}
