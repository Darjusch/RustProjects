use super::server::Handler;
use super::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
  public_path: String
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
        "/admin/info" => Response::new(StatusCode::Ok, Some("<h1>Admin Info</h1>".to_string())),
        "/admin/config" => Response::new(StatusCode::Ok, Some("<h1>Admin Config</h1>".to_string())),
        "/admin/console" => Response::new(StatusCode::Ok, Some("<h1>Admin Console</h1>".to_string())),
        _ => Response::new(StatusCode::NotFound, None),
      }
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}