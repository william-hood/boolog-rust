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
use uuid::Uuid;
use serde::Serialize;
use crate::boolog::{ecapsulation_tag, treat_as_code, Boolog};
use crate::constants::{EMOJI_ERROR, EMOJI_OBJECT, MAX_BODY_LENGTH_TO_DISPLAY, NAMELESS};

pub trait ShowObjectExt {
    fn show_as_json<T: Serialize>(&mut self, target: T, target_type_name: &str, target_variable_name: &str) -> Vec<u8>;
    fn show_as_json_detailed<T: Serialize>(&mut self, target: T, target_type_name: &str, target_variable_name: &str, emoji: &[u8], style: &str) -> Vec<u8>;

    fn show_as_error<T: Serialize, Err>(&mut self, target: T) -> Vec<u8>;
}

impl ShowObjectExt for Boolog<'_> {
    fn show_as_json<T: Serialize>(&mut self, target: T, target_type_name: &str, target_variable_name: &str) -> Vec<u8> {
        self.show_as_json_detailed(target, target_type_name, target_variable_name, EMOJI_OBJECT, "plate")
    }

    fn show_as_json_detailed<T: Serialize>(&mut self, target: T, target_type_name: &str, target_variable_name: &str, emoji: &[u8], style: &str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let timestamp = Local::now();

        let message = format!("Showing {target_type_name}: {target_variable_name} (details in HTML log)");
        self.echo_plain_text(message.as_bytes(), EMOJI_OBJECT, timestamp);

        let json_target = serde_json::to_string_pretty(&target).unwrap();
        let rendered_target = treat_as_code(&json_target);

        result.append("\r\n<div class=\"object ".as_bytes().to_vec().as_mut());
        result.append(style.as_bytes().to_vec().as_mut());
        result.append("\">\r\n".as_bytes().to_vec().as_mut());

        result.append("<center><h2>".as_bytes().to_vec().as_mut());
        result.append(target_type_name.as_bytes().to_vec().as_mut());
        result.append("</h2>\r\n<small>".as_bytes().to_vec().as_mut());

        if target_variable_name != NAMELESS {
            result.append("<b>\"".as_bytes().to_vec().as_mut());
        }

        result.append(target_variable_name.as_bytes().to_vec().as_mut());

        if target_variable_name != NAMELESS {
            result.append("\"</b>".as_bytes().to_vec().as_mut());
        }

        result.append("</small></center>\r\n".as_bytes().to_vec().as_mut());

        if rendered_target.len() > MAX_BODY_LENGTH_TO_DISPLAY {
            let identifier = Uuid::new_v4().to_string();
            let tag = ecapsulation_tag();
            result.append("<label for=\"".as_bytes().to_vec().as_mut());
            result.append(identifier.as_bytes().to_vec().as_mut());
            result.append("\">\r\n<input id=\"".as_bytes().to_vec().as_mut());
            result.append(identifier.as_bytes().to_vec().as_mut());
            result.append("\" type=\"checkbox\">\r\n(show large object)\r\n<div class=\"".as_bytes().to_vec().as_mut());
            result.append(tag.as_bytes().to_vec().as_mut());
            result.append("\">\r\n".as_bytes().to_vec().as_mut());
            result.append(rendered_target.as_bytes().to_vec().as_mut());
            result.append("</div></label>".as_bytes().to_vec().as_mut());
        } else {
            result.append(rendered_target.as_bytes().to_vec().as_mut());
        }

        result.append("\r\n</div>".as_bytes().to_vec().as_mut());

        self.write_to_html(result.as_slice(), emoji, timestamp);

        result
    }

    fn show_as_error<T: Serialize, Err>(&mut self, target: T) -> Vec<u8> {
        self.show_as_json_detailed(target, "Error", "", EMOJI_ERROR, "error")
    }
}