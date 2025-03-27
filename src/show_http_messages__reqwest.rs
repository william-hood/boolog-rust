// Copyright (c) 2025 William Arthur Hood
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies
// of the Software, and to permit persons to whom the Software is furnished
// to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
// HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.

use chrono::Local;
use lazy_static::lazy_static;
use reqwest::Url;
use reqwest::header::HeaderMap;
use uuid::Uuid;
use crate::boolog::{ecapsulation_tag, treat_as_code, Boolog};
use crate::constants::{EMOJI_INCOMING, EMOJI_OUTGOING, MAX_BODY_LENGTH_TO_DISPLAY, MAX_HEADERS_TO_DISPLAY};

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub struct ConsumedResponse {
    status: reqwest::StatusCode,
    headers: HeaderMap,
    url: Url,
    content_length: Option<u64>,
    resp_body_bytes: Vec<u8>
}

impl ConsumedResponse {
    pub fn new(resp: reqwest::blocking::Response) -> ConsumedResponse {
        ConsumedResponse {
            status: resp.status(),
            headers: resp.headers().clone(),
            url: resp.url().clone(),
            content_length: resp.content_length().clone(),
            resp_body_bytes: {
                let tmp = resp.bytes();
                match tmp {
                    Ok(tmpBytes) => tmpBytes.clone().to_owned().to_vec(),
                    _ => Vec::new()
                }
            }
        }
    }
}

pub trait ShowHttpViaReqwestExt<'a> {
    fn show_http_request(&mut self, req: reqwest::blocking::Request, callback: fn(&str, &str) -> Vec<u8>);
    fn show_http_response(&mut self, resp: reqwest::blocking::Response, callback: fn(&str, &str) -> Vec<u8>) -> ConsumedResponse;
    fn render_headers_and_body(&mut self, headers: &HeaderMap, body: Vec<u8>, callback: fn(&str, &str) -> Vec<u8>) -> Vec<u8>;
    fn show_http_transaction_blocking(&mut self, req: reqwest::blocking::Request, callback: fn(&str, &str) -> Vec<u8>) -> ConsumedResponse;
}

impl<'a> ShowHttpViaReqwestExt<'a> for Boolog<'a> {
    fn show_http_request(&mut self, req: reqwest::blocking::Request, callback: fn(&str, &str) -> Vec<u8>) {
        let mut result: Vec<u8> = Vec::new();
        let timestamp = Local::now();

        result.append("<div class=\"outgoing implied_caution\">\r\n".as_bytes().to_vec().as_mut());

        let text_rendition = format!("{} {}", req.method(), req.url().path());

        result.append("<center><h2>".as_bytes().to_vec().as_mut());
        result.append(text_rendition.as_bytes().to_vec().as_mut());
        result.append("</h2><small><b><i>".as_bytes().to_vec().as_mut());
        result.append(req.url().host().unwrap().to_string().as_bytes().to_vec().as_mut());
        result.append("</i></b></small>".as_bytes().to_vec().as_mut());

        let identifier = Uuid::new_v4().to_string();

        result.append("<br><br><label for=\"".as_bytes().to_vec().as_mut());
        result.append(identifier.as_bytes().to_vec().as_mut());
        result.append("\">\r\n<input id=\"".as_bytes().to_vec().as_mut());
        result.append(identifier.as_bytes().to_vec().as_mut());
        result.append("\" type=\"checkbox\"><small><i>(show complete URL)</i></small>\r\n<div class=\"".as_bytes().to_vec().as_mut());
        result.append(ecapsulation_tag().as_bytes().to_vec().as_mut());
        result.append("\">\r\n".as_bytes().to_vec().as_mut());

        result.append("<br>\r\n".as_bytes().to_vec().as_mut());
        let tmp = req.url().to_string().replace("&", "&amp");
        result.append(tmp.as_bytes().to_vec().as_mut());
        result.append("\r\n</div>\r\n</label>".as_bytes().to_vec().as_mut());

        if req.url().query() == None {
            result.append("<br><br><small><i>(no query)</i></small>".as_bytes().to_vec().as_mut());
        } else {
            result.append("<br><br><b>Queries</b><br><table class=\"gridlines\">\r\n".as_bytes().to_vec().as_mut());

            for (_, param) in req.url().query_pairs().enumerate() {
                result.append("<tr><td>".as_bytes().to_vec().as_mut());
                result.append(param.0.to_string().as_bytes().to_vec().as_mut());
                result.append("</td><td>".as_bytes().to_vec().as_mut());

                let tmp = param.1.to_string();
                if tmp == "" {
                    result.append("(unset)".as_bytes().to_vec().as_mut());
                } else {
                    result.append(callback(param.0.trim(), &tmp).as_mut());
                }
            }
            result.append("</td></tr>".as_bytes().to_vec().as_mut());
        }
        result.append("\r\n</table><br>".as_bytes().to_vec().as_mut());



        result.append(&mut self.render_headers_and_body(req.headers(), match req.body() {
            None => Vec::new(),
            _ => req.body().unwrap().as_bytes().unwrap().to_vec()
        }, callback));

        self.write_to_html(result.as_slice(), EMOJI_OUTGOING, timestamp);
        self.echo_plain_text(text_rendition.as_bytes(), EMOJI_OUTGOING, timestamp);
    }

    fn show_http_response(&mut self, resp: reqwest::blocking::Response, callback: fn(&str, &str) -> Vec<u8>) -> ConsumedResponse {
        let mut rendition: Vec<u8> = Vec::new();
        let timestamp = Local::now();

        let result = ConsumedResponse::new(resp);

        let mut style = "implied_bad";

        if result.status.is_success() {
            style = "implied_good";
        }

        rendition.append("<div class=\"incoming ".as_bytes().to_vec().as_mut());
        rendition.append(style.as_bytes().to_vec().as_mut());
        rendition.append("\">\r\n".as_bytes().to_vec().as_mut());

        let text_rendition = result.status.to_string();

        rendition.append("<center><h2>".as_bytes().to_vec().as_mut());
        rendition.append(text_rendition.as_bytes().to_vec().as_mut());
        rendition.append("</h2>".as_bytes().to_vec().as_mut());

        rendition.append(self.render_headers_and_body(&result.headers, result.resp_body_bytes.clone(), callback).as_mut());

        self.write_to_html(rendition.as_slice(), EMOJI_INCOMING, timestamp);
        self.echo_plain_text(text_rendition.as_bytes(), EMOJI_INCOMING, timestamp);

        result
    }
    fn render_headers_and_body(&mut self, headers: &HeaderMap, body_bytes: Vec<u8>, callback: fn(&str, &str) -> Vec<u8>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();

        // Headers
        if headers.len() > 0 {
            result.append("<br><b>Headers</b><br>".as_bytes().to_vec().as_mut());

            let mut rendered_headers: Vec<u8> = Vec::new();
            rendered_headers.append("<table class=\"gridlines\">\r\n".as_bytes().to_vec().as_mut());

            for (key,value) in headers.iter() {
                rendered_headers.append("<tr><td>".as_bytes().to_vec().as_mut());
                rendered_headers.append(key.clone().as_str().as_bytes().to_vec().as_mut());
                rendered_headers.append("</td><td>".as_bytes().to_vec().as_mut());

                // reqwest appears to return only a single value per key; I'm assuming the key would appear more than once if multiple values exist.
                rendered_headers.append(callback(key.as_str(), value.to_str().unwrap()).as_mut());

                rendered_headers.append("</td></tr>".as_bytes().to_vec().as_mut());
            }
            rendered_headers.append("\r\n</table><br>".as_bytes().to_vec().as_mut());

            if headers.len() > MAX_HEADERS_TO_DISPLAY {
                let identifier = Uuid::new_v4().to_string();
                let tag = ecapsulation_tag();
                result.append("<label for=\"".as_bytes().to_vec().as_mut());
                result.append(identifier.as_bytes().to_vec().as_mut());
                result.append("\">\r\n<input id=\"".as_bytes().to_vec().as_mut());
                result.append(identifier.as_bytes().to_vec().as_mut());
                result.append("\" type=\"checkbox\">\r\n(show ".as_bytes().to_vec().as_mut());
                result.append(headers.len().to_string().as_bytes().to_vec().as_mut());
                result.append(" headers)\r\n<div class=\"".as_bytes().to_vec().as_mut());
                result.append(tag.as_bytes().to_vec().as_mut());
                result.append("\">\r\n".as_bytes().to_vec().as_mut());
                result.append(rendered_headers.as_mut());
                result.append("</div></label>".as_bytes().to_vec().as_mut());
            } else {
                result.append(rendered_headers.as_mut());
            }
        } else {
            result.append("<br><br><small><i>(no headers)</i></small><br>\r\n".as_bytes().to_vec().as_mut());
        }

        // Body
        if body_bytes.len() < 1 {
            result.append("<br><br><small><i>(no payload)</i></small></center>".as_bytes().to_vec().as_mut());
        } else {
            let string_payload = String::from_utf8_lossy(body_bytes.as_slice());
            let payload_size = string_payload.as_ref().len();
            result.append("<br><b>Payload</b><br></center>\r\n".as_bytes().to_vec().as_mut());
            let rendered_body = treat_as_code(string_payload.as_ref());

            if payload_size > MAX_BODY_LENGTH_TO_DISPLAY {
                let identifier = Uuid::new_v4().to_string();
                let tag = ecapsulation_tag();
                result.append("<label for=\"".as_bytes().to_vec().as_mut());
                result.append(identifier.as_bytes().to_vec().as_mut());
                result.append("\">\r\n<input id=\"".as_bytes().to_vec().as_mut());
                result.append(identifier.as_bytes().to_vec().as_mut());
                result.append("\" type=\"checkbox\">\r\n(show large payload)\r\n<div class=\"".as_bytes().to_vec().as_mut());
                result.append(tag.as_bytes().to_vec().as_mut());
                result.append("\">\r\n".as_bytes().to_vec().as_mut());
                result.append(rendered_body.as_bytes().to_vec().as_mut());
                result.append("</div></label>".as_bytes().to_vec().as_mut());
            } else {
                result.append(rendered_body.as_bytes().to_vec().as_mut());
            }
        }

        result
    }

    fn show_http_transaction_blocking(&mut self, req: reqwest::blocking::Request, callback: fn(&str, &str) -> Vec<u8>) -> ConsumedResponse {
        self.show_http_request(req.try_clone().unwrap(), callback);
        let original_resp = CLIENT.execute(req).unwrap();
        self.show_http_response(original_resp, callback)
    }
}