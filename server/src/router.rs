use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{
    request::{self, HttpRequest},
    response::HttpResponse,
};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, write_stream: &mut impl Write) -> () {
        match req.method {
            request::method::Method::GET => Self::get(req, write_stream),
            _ => {
                let res: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = res.send_response(write_stream);
            }
        }
    }

    pub fn get(req: HttpRequest, write_stream: &mut impl Write) -> () {
        let http::request::resource::Resource::Path(path) = &req.resource;

        //e.g http::localhost:5000/api/v1/orders or http::localhost:5000/
        let route: Vec<&str> = path.split("/").collect();

        match route[1] {
            "api" => {
                let res: HttpResponse = WebServiceHandler::handle(&req);
                let _ = res.send_response(write_stream);
            }
            _ => {
                let res: HttpResponse = StaticPageHandler::handle(&req);
                let _ = res.send_response(write_stream);
            }
        }
    }
}
