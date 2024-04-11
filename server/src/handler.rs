use http::{request::HttpRequest, response::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait Handler {
    fn handle(req: &HttpRequest) -> HttpResponse;
    fn load_file(file_name: &str) -> Option<String> {
        let default = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default);

        let path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(path);
        contents.ok()
    }
}

pub struct PageNotFoundHandler;
pub struct WebServiceHandler;
pub struct StaticPageHandler;

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    id: i32,
    status: String,
    date: String,
}

impl Handler for PageNotFoundHandler {
    fn handle(_req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default);

        let path = format!("{}/orders.json", data_path);
        let json = fs::read_to_string(path);
        let orders: Vec<OrderStatus> = serde_json::from_str(json.unwrap().as_str()).unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        //Get path in request
        let http::request::resource::Resource::Path(path) = &req.resource;

        //e.g http::localhost:5000/api/v1/orders => ["http::localhost:5000", "api", "v1", "orders"]
        let route: Vec<&str> = path.split("/").collect();
        if route.len() <= 3 {
            return HttpResponse::new("404", None, Self::load_file("404.html"));
        }

        let resource = route[2];
        match resource {
            "v1" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();

                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }

            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::request::resource::Resource::Path(path) = &req.resource;

        //e.g http::localhost:5000/ => ["http::localhost:5000", ""]
        let route: Vec<&str> = path.split("/").collect();

        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            file_name => match Self::load_file(file_name) {
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
                Some(body) => {
                    let mut headers: HashMap<&str, &str> = HashMap::new();

                    if file_name.ends_with(".js") {
                        headers.insert("Content-Type", "text/javascript");
                    } else if file_name.ends_with(".css") {
                        headers.insert("Content-Type", "text/css");
                    } else if file_name.ends_with(".html") {
                        headers.insert("Content-Type", "text/html");
                    } else {
                        headers.insert("Content-Type", "text/plain");
                    }

                    HttpResponse::new("200", Some(headers), Some(body))
                }
            },
        }
    }
}
