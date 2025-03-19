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

extern crate chrono;
extern crate string_builder;
extern crate uuid;

use chrono::Local;
use string_builder::Builder;
use uuid::Uuid;

const STARTING_CONTENT: &str = "<table class=\"left_justified\">\r\n";

fn highlight(message: &str, style: &str) -> &str {
    return highlight_with_style(message, "highlighted");
}

fn highlight_with_style(message: &str, style: &str) -> &str {
    return "<p class=\"$style outlined\">&nbsp;$message&nbsp;</p>"
}

pub struct Boolog {
    title: String,
    for_plain_text: &mut impl Write + ?Sized,
    for_html: &mut impl Write + ?Sized,
    show_time_stamps: bool,
    show_emojis: bool,
    header_function: &dyn Fn(i32) -> i32,
    content: Builder,
    is_concluded: bool,
    first_echo: bool
}

impl Boolog {
    pub fn new(
        &mut self,
        title: String,
        for_plain_text: &mut impl Write + ?Sized,
        for_html: &mut impl Write + ?Sized,
        theme: &str,
        show_time_stamps: bool,
        show_emojis: bool,
        header_function: &dyn Fn(String) -> String
    ) -> Boolog {
        let result = Boolog {
            title,
            for_plain_text,
            for_html,
            show_time_stamps,
            show_emojis,
            header_function,
            content: Builder::new(),
            is_concluded: false,
            first_echo: true
        };

        if (self.for_html != null) {
            self.for_html.write("<html>\r\n<meta charset=\"UTF-8\">\r\n<head>\r\n<title>{self.title}</title>\r\n");
            self.for_html.write(MEMOIR_LOG_STYLING);
            self.for_html.write("</head>\r\n<body>\r\n");
            self.for_html.write(self.header_function(title));
        }

        result.content.append(STARTING_CONTENT);

        return result;
    }

    pub fn was_used(&mut self) -> bool {
        (self.content.len() - STARTING_CONTENT.len()) > 0
    }
}