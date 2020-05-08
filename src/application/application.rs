extern crate chrono;
extern crate iron;
extern crate serde_json;

use iron::headers::ContentType;
use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde::ser::SerializeStruct;
use serde_json::json;

/// とあるオブジェクトの定義
#[derive(Debug)]
struct MyUserObject1 {
	/// 氏名
	name: String,
	/// 住所
	address: String,
}

impl serde::ser::Serialize for MyUserObject1 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let mut s = serializer.serialize_struct("MyUserObject1", 2)?;
		s.serialize_field("name", &self.name)?;
		s.serialize_field("address", &self.address)?;
		s.end()
	}
}

#[allow(dead_code)]
fn unknown_to_json(unknown: &MyUserObject1) -> String {
	let value = serde_json::to_value(&unknown).unwrap();
	super::mylib::util::Util::get_current_timestamp();
	return format!("{}", value);
}

pub struct Application;

/// アプリケーションクラス
impl Application {
	/// echo
	fn on_echo_handler(request: &mut Request) -> IronResult<Response> {
		let query_string = request.url.query().unwrap_or("");
		return Ok(Response::with((Status::Ok, query_string)));
	}

	/// /root
	fn on_root_handler(request: &mut Request) -> IronResult<Response> {
		request.url.query();
		return Ok(Response::with((Status::Ok, "(root)")));
	}

	/// /hello
	fn on_hello_handler(request: &mut Request) -> IronResult<Response> {
		request.url.query();
		return Ok(Response::with((Status::Ok, "Hello world!")));
	}

	fn on_recv_json_handler(request: &mut Request) -> IronResult<Response> {
		// println!("[TRACE] BODY: {:?}", request.body.);
		request.url.query();
		let user = MyUserObject1 {
			name: "直江 兼続".to_string(),
			address: "山形県米沢市".to_string(),
		};
		let value = serde_json::to_value(user).unwrap();
		println!("[TRACE] オブジェクトの文字列表現: {}", value);
		let unknown = json!({
			"name": "直江 兼続".to_string(),
			"address": "山形県米沢市".to_string(),
		});
		let payload = format!("{}", unknown);
		println!("[TRACE] RESPONSE: [OK], PAYLOAD: {:?}", payload);
		return Ok(Response::with((ContentType::json().0, Status::Ok, payload)));
	}

	/// サーバーを起動します。
	pub fn run(self: &Application) -> std::result::Result<(), Box<dyn std::error::Error>> {
		let mut router = Router::new();
		router.get("/", Application::on_root_handler, "on_root_handler");
		router.get("/hello", Application::on_hello_handler, "on_hello_handler");
		router.get("/:query", Application::on_echo_handler, "on_echo_handler");
		router.post("/api1", Application::on_recv_json_handler, "on_recv_json_handler");
		let _ = Iron::new(router).http("127.0.0.1:3000");
		return Ok(());
	}
}
