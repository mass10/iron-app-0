extern crate iron;

use iron::prelude::*;
use iron::status::Status;

fn on_echo_handler(request: &mut Request) -> IronResult<Response> {
	let query_string = request.url.query().unwrap_or("");
	return Ok(Response::with((Status::Ok, query_string)));
}

fn on_hello_handler(request: &mut Request) -> IronResult<Response> {
	return Ok(Response::with((Status::Ok, "Hello world!")));
}

use router::Router;

fn main() {
	let mut router = Router::new();
	router.get("/", on_hello_handler, "on_hello_handler");
	router.get("/:query", on_echo_handler, "on_echo_handler");

	let _ = Iron::new(router).http("localhost:3000");
}
