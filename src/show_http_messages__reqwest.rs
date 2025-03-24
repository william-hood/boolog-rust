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
use reqwest::Body;
use reqwest::header::HeaderMap;
use uuid::Uuid;
use crate::boolog::{ecapsulation_tag, treat_as_code, Boolog};
use crate::constants::{EMOJI_OBJECT, EMOJI_OUTGOING, MAX_BODY_LENGTH_TO_DISPLAY, NAMELESS};

pub trait ShowHttpReqwestExt {
    fn show_http_request(&mut self, req: reqwest::Request, callback: fn(&str, &str) -> &str);
    fn show_http_response(&mut self, resp: reqwest::Response, callback: fn(&str, &str) -> &str);
    fn render_headers_and_body(&self, headers: &HeaderMap, body: Option<&Body>, callback: fn(&str, &str) -> &str);
    fn show_http_transaction_blocking(&self, req: reqwest::Request, callback: fn(&str, &str) -> &str);
}

impl ShowHttpReqwestExt for Boolog<'_> {
    fn show_http_request(&mut self, req: reqwest::Request, callback: fn(&str, &str) -> &str) {
        let mut result: Vec<u8> = Vec::new();
        let timestamp = Local::now();

        result.append("<div class=\"outgoing implied_caution\">\r\n".as_bytes().to_vec().as_mut());

        let text_rendition = format!("{} {}", req.method(), req.url().path());

        result.append("<center><h2>".as_bytes().to_vec().as_mut());
        result.append(text_rendition.as_bytes().to_vec().as_mut());
        result.append("</h2><small><b><i>".as_bytes().to_vec().as_mut());
        result.append(req.url().host().unwrap().as_bytes().to_vec().as_mut());
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
                    result.append(callback(param.0, &tmp).as_mut());
                }
            }
            result.append("</td></tr>".as_bytes().to_vec().as_mut());
        }
        result.append("\r\n</table><br>".as_bytes().to_vec().as_mut());

        result.append(self.render_headers_and_body(req.headers(), req.body(), &callback));

        self.write_to_html(result.as_slice(), EMOJI_OUTGOING, timestamp);
        self.echo_plain_text(text_rendition, EMOJI_OUTGOING, timestamp);
    }

    fn show_http_response(&mut self, resp: reqwest::Response, callback: fn(&str, &str) -> &str){
        let mut result: Vec<u8> = Vec::new();
        let timestamp = Local::now();

    }
    fn render_headers_and_body(&self, headers: &HeaderMap, body: Option<&Body>, callback: fn(&str, &str) -> &str) -> Vec<u8> {

    }
    fn show_http_transaction_blocking(&self, req: reqwest::Request, callback: fn(&str, &str) -> &str){

    }
}