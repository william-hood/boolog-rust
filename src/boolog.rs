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
extern crate rstring_builder;
extern crate uuid;

use std::io::{Stdout, Write};
use std::string::ToString;
use std::time::SystemTime;
use chrono::{DateTime, Local, Utc};
use rstring_builder::StringBuilder;
use uuid::Uuid;
use crate::constants::{ALREADY_CONCLUDED_MESSAGE, EMOJI_BOOLOG, EMOJI_DEBUG, EMOJI_ERROR, EMOJI_TEXT_BLANK_LINE, EMOJI_TEXT_BOOLOG_CONCLUDE};

const STARTING_CONTENT: String = "<table class=\"left_justified\">\r\n".to_string();

pub struct Boolog {
    title: String,
    for_plain_text: Box<dyn Write>,
    for_html: Box<dyn Write>,
    show_time_stamps: bool,
    show_emojis: bool,
    header_function: dyn Fn(String) -> String,
    content: StringBuilder,
    is_concluded: bool,
    first_echo: bool
}

impl Boolog {
    pub fn new(
        &mut self,
        title: String,
        plain_text: Option<Box<dyn Write>>,
        html: Option<Box<dyn Write>>,
        theme: String,
        show_time_stamps: bool,
        show_emojis: bool,
        header_function: &dyn Fn(String) -> String
    ) -> Boolog {
        let for_plain_text: Box<dyn Write> = plain_text.unwrap_or(Box::new(std::io::stdout()));
        let for_html: Box<dyn Write> = html.unwrap_or(Box::default());

        let mut result = Boolog {
            title,
            for_plain_text,
            for_html,
            show_time_stamps,
            show_emojis,
            header_function,
            content: StringBuilder::new(),
            is_concluded: false,
            first_echo: true
        };

        if self.for_html != Box::default() {
            self.for_html.write("<html>\r\n<meta charset=\"UTF-8\">\r\n<head>\r\n<title>".as_bytes());
            self.for_html.write(self.title.as_bytes());
            self.for_html.write("</title>\r\n".as_bytes());
            self.for_html.write(theme.as_bytes());
            self.for_html.write("</head>\r\n<body>\r\n".as_bytes());
            self.for_html.write(self.header_function(&self.title));
        }

        result.content.append(STARTING_CONTENT);

        result
    }

    pub fn was_used(&mut self) -> bool {
        (self.content.len() - STARTING_CONTENT.len()) > 0
    }

    pub fn echo_plain_text(&mut self, message: String, emoji: String, timestamp: SystemTime) -> Result<(), &str> {
        if self.for_plain_text != Box::default() {
            if self.is_concluded {
                Err(ALREADY_CONCLUDED_MESSAGE);
            }

            if self.first_echo {
                self.first_echo = false;
                self.echo_plain_text("".to_string(), EMOJI_TEXT_BLANK_LINE.to_string(), timestamp);
                self.echo_plain_text(self.title.to_string(), EMOJI_BOOLOG.to_string(), timestamp);
            }

            if self.show_time_stamps {
                // TODO - Proper timestamp formatting
                self.for_plain_text.write(timestamp.to_string().as_bytes());
                self.for_plain_text.write("\t".as_bytes());
            }

            if self.show_emojis {
                self.for_plain_text.write(format!("\t{emoji}").as_bytes());
            }

            self.for_plain_text.write(format!("{message}\r\n").as_bytes());
        }

        Ok(())
    }

    pub fn write_to_html(&mut self, message: String, emoji: String, timestamp: SystemTime) -> Result<(), &str> {
        if self.is_concluded {
            Err(ALREADY_CONCLUDED_MESSAGE);
        }

        self.content.append("<tr>");

        if self.show_time_stamps {
            // TODO - Proper timestamp formatting
            self.content.append(timestamp.to_string());
        }

        if self.show_emojis {
            self.content.append(format!("<td><h2>{emoji}</h2></td>"));
        }

        self.content.append(format!("<td>{message}</td></tr>\r\n"));

        Ok(())
    }

    pub fn conclude(&mut self) -> String {
        if !self.is_concluded {
            let timestamp = SystemTime::now();
            self.echo_plain_text("".to_string(), EMOJI_TEXT_BOOLOG_CONCLUDE.to_string(), timestamp);
            self.echo_plain_text("".to_string(), EMOJI_TEXT_BLANK_LINE.to_string(), timestamp);

            // TODO - Translate from pseudo-rust to actual rust
            if self.for_plain_text != Stdout {
                self.for_plain_text.flush();
                drop(self.for_plain_text);
            }

            self.is_concluded = true;

            self.content.append("\r\n</table>");

            if self.for_html != Box::default() {
                // TODO - Translate from pseudo-rust to actual rust
                self.for_html.write(self.content.to_string().as_bytes());
                self.for_html.write("\r\n</body>\r\n</html>".as_bytes())
                self.for_html.flush();
                drop(self.for_html);
            }
        }

        // TODO - Translate from pseudo-rust to actual rust
        self.content.string().unwrap().to_string();
    }

    pub fn info(&mut self, message: String) -> Result<(), &str> {
        return self.info_detailed(message, EMOJI_TEXT_BLANK_LINE.to_string());
    }

    pub fn info_detailed(&mut self, message: String, emoji: String) -> Result<(), &str> {
        let timestamp = SystemTime::now();
        let result = self.write_to_html(message, emoji, timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text(message, emoji, timestamp);
    }

    pub fn debug(&mut self, message: String) -> Result<(), &str> {
        let timestamp = SystemTime::now();
        let result = self.write_to_html(highlight(message), EMOJI_DEBUG.to_string(), timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text(message, EMOJI_DEBUG.to_string(), timestamp);
    }

    pub fn error(&mut self, message: String) -> Result<(), &str> {
        let timestamp = SystemTime::now();
        let result = self.write_to_html(highlight(message), EMOJI_ERROR.to_string(), timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text(message, EMOJI_ERROR.to_string(), timestamp);
    }

    pub fn skip_line(&mut self) -> Result<(), &str> {
        let timestamp = SystemTime::now();
        let result = self.write_to_html("".to_string(), EMOJI_TEXT_BLANK_LINE.to_string(), timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text("".to_string(), EMOJI_TEXT_BLANK_LINE.to_string(), timestamp);
    }

    pub fn show_boolog(&mut self, subordinate: Boolog) -> Result<String, &str> {
        return self.show_boolog_detailed(subordinate, EMOJI_BOOLOG.to_string(), "boolog".to_string(), 0);
    }

    pub fn show_boolog_detailed(&mut self, mut subordinate: Boolog, emoji: String, style: String, recurse_level: u8) -> Result<String, &str> {
        let timestamp = SystemTime::now();
        let subordinate_content = subordinate.conclude();
        let result = wrap_as_subordinate(subordinate.title, subordinate_content, style);

        if (recurse_level > 0) {
            // TODO: probably make write_to_html and echo_plain_text take bytes instead of String
            let check = self.write_to_html(result, emoji, timestamp);
            if check.is_err() {
                Err(check.err().unwrap());
            }
        }

        Ok(result);
    }
}

pub fn default_header(title: String) -> String {
    let mut builder: StringBuilder = StringBuilder::new();
    builder.append("<h1>".to_string());
    builder.append("<img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEEAAABBCAYAAACO98lFAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAdnJLH8AAAAgY0hSTQAAeiYAAICEAAD6AAAAgOgAAHUwAADqYAAAOpgAABdwnLpRPAAAAAZiS0dEAAAAAAAA+UO7fwAAAAlwSFlzAAAXEgAAFxIBZ5/SUgAAAAd0SU1FB+kDBwQ3FkQyHzoAAA80SURBVHja1Zt5fBRVtse/tzvpJIAEQRHFwXFDVBjFQRQFB+fhNsgTP+P49CkgiiIjggZXPqiDyDKAKDIoIAHDPjxERJR9CwgEBGQfCYQlkI0sZOm1qvu8P7o6Vjed9GLQcD6f++kUdavq3l+d+zvn/k6hOP9mBToDNwC/B654a0jaNSkpKa1u79ixeXJKstbkkkudaHDoyAH7lLc/3rHpxM4vgfVAMRe4dQbm7Ni2rVLisJyjxzzAfOCyC3HyNwObpI4sa8u2AuDeCwmA1zM3ZTqljs0nIkC/CwGAcWeLi+R8mdPh9AA96zMA78mvYP8cN85hkGu9s3sL8k/55FcyIL0+grA1ysFXt0PHjsQNQmFpiQ+4rj4BcGtO9pGIXnDiVF4QCEBEoKZOn14boEPqEwgZsXpBbUBEA5TRb1l9AcA6uE/fkroCYdyEj885f6qwpKb7ueoLCD1iILOIIIQ7763hfosWLhagcX0A4cNoAJgwcWLYCR4+djwiCBGA7VgfQNhyPvkgChDurquJWOK87rpjOTl3xvvQFwb0Dzp2+rwxXX809zRA5W/tBW/EmODU+obP2l0xecGz/QfUC2JcG0emFynsVbfBg16NdK81dTmZhHiuuf+eWzvFepF/7JHP55eUcXmziyPdbt1v7QUdqlya/FZWUFkpwG11OaF4iPHahkk/O5AP8AK68StR3mRN5maUUjE/vMVFFx0Fdv3WnvBZqODhFRHdaIHjWMgyRm6p831DPJxwq/lAAZrp2Gcoq7VZqAcopSJyhtHvB+Aj4/Am4AHgob+/OawpXp3V/8o4ke3K/wb4Cig/n55wyuwFuoh4TM3pE3FH8IZYEyMRkQK7LsBcYMPEaXNcBVU189JLae85gBHnC4BLZsycGfRAr4g4Tc3lE9HiyCLPkwDzf0BSRA+LNTKIyI7AgddwfxF/U4BS4FP+JZEQ5XKIJoTGa0qpb4HHAUddRYcbQhFUBhhe9XNTvyAfD7WKKPqUSK35R3egT10SY6PQ0Ogz3r6SYGB0wBblTXceyK7RW24HdphD61fzKDmTj+aswu6uov8b4/ihwEGzy1Jqe8SgtLubd/SmtqZKv4SkZk35dP4MAUYD2bEuh/4iMuWc5QBopuWgjAiRGOVyCF0KSimWfnI9iQ3aQcIVWBIEp7OCns/PRqQKaGj01CjYv50VlZfxTKfwsqMG2NQ9iGSGG8d+4A+xeuc/AqSjiz8KOEXEYbQAOVYZvzUR5O5DhwWQb9dlnnNu8crN8hLIy83Cb8FHp/UOoWW3fD33M/lkxaFzCja6iGRViKxdsrA28nwwVhBGmR+iGUC4jIlXiYjdAMQlkaNEOHtr2EvVE5742l+NyFEUFFmCzZ+i5e7PEkBuGv61vLP0gJSKyPQ9ZdLhgacjRZAJsYLQK5InBAAJAOGJc7dpbuu/HSeiH5azxduiDqdLM2ZJaW5+xH5D0kZuiZXED5nDisVY+75Ak58zRouJJMU4H43d2bXrOf92zXWNObBvJfv2fhO9ANq7Fxdf2SJiv9s7tv19rCAcGDVuvJ/MTJOzBpoCu9uDNeS8HsPGauv69edGj+39ydryMuvXjQxDbpejVDuUUuzcmYNSCqX6GL9dUWowSilatBmKUorFSzYGXd+wUUpTFUfo3iUi7TF5gBgTVcDUT6dTkF/A+yOGVWdigZBpMbVID1ZKFQBPAc6QU+kicmPgwO4GtxvcHi9r1h2k1+PtYprMmLHT9saTv4wx84LH4Aa38bcmImMmTZFBLw8J2lNoRtNNffXauSFsgmOjR05dptZd7n81M55d5OYih/5m8wYJWE1rXjNxQNrA/sxbsAibUnhFzlkOyuQJEsYrBr076SQwK9zDH+t+eStM1+YWw7CPs3D6LNj1i1h+chusO83I8R5SbMdZv6cvD3Voz//2TCU1zPZ206qP9scrxOw2R2qthrd9+HSBALJ4xepqvUE3oobZc8J4wdgant3mp737xGs89x/9kKMTETmOiNTecrciYx9HFn7zdZ3pE203bN9tN98s3LLQjcGmDR0mgHz/w/5qoDSTCGO2cv/AatIw3/E4i8WjOUUXkbFD20nxrch6kD0gu0C2g3wPsgBkB8jhvyDZ3ZDpIJKPDDeF2FNFIkBbRfw2MN+lT2qRZA0SVAghSnObMjWDgS8+w66fjtO29VXVGy1ziHrgwV7lq1bOuTREq8EIQCcdZ7Kv8CU1wZrUmASbjbIqWLDsCKX5J7EeWUfnu5phLz2De/9oWrcER5F/MLaLYUce6N2yGNCrY4B894SKRPHYsBPldl9oqqqFtNAlMH/J8qBkaNOOPXKitCrgnitqeNZ9Bzd+IiUnt0hFcbY4HcW1EmuUidmT8egJ4ax7lzvumpO57fsmhMhsWohooUK23wpwA4sWLaWsrIx1m7aybPZUxyPPPX/opyLo1qkdjS5u5RkzYHk+TG31wTOpHfoNmYk1tSVJjS7F1rA5ydd+wdxJj7DvSAE9H7oNpxtSUy1oOhzMzqPNNS3IPV1I9vEjDH2+S/X4tu0+Xt7ptqsvD4TgOwyt7hXgdaAv0B1oEgMQLwKyNutHCSXMAGmaN1iukDDpq4Efwtn7Q3tLQfYKKS/aI5kblgopGQKz5LURu+WpV7fJDwdFPpicI8cKw7z5O2eYvWCRmelv6N3r+XbLVq271S7yjojMEJFlpW4pHjf504VAmyhA+AIod1SVcmOn9hw8VYjFlDH6TAmSJSDCmKR6LYaM8rEPMjhyYAtup517uv43OE8DOYx/ZyVzM/PocNN8XA47V182HaXGodTLZG53A/A/twRpr/+uSV5rBnwgIi8E+KpSw9nYpnoAayOMb76IPAEwZe5sBjzdm10/5dCu9dVBZBlueQSAsZj2HbXZA0oxIWs+s2d/S+MW19OgyaUkW5uSf8ZBhW6ltEJj1sdH8H8Z3Ay4FliNx7eHRGXj0acGZC+ZN+XmMOQbTEJ7j58sD7jOnH8vOQY0jUJwCdrgjv3kMwHk0xkZ4jItA7dpy+00jrUo6xWVxk7zxL4VUnBsn5QU5krp2RJxOFxi93jF6RVx+4KXWn7+GZk07QvzUng42nV+V7l/nIEL50UqDs1YMM9tBiHAB9uPHKuOBKu37KqRF6Iq2jw6TfZsnCqHdn8nC+bMlnf7IYd2DpapE5rJ6m/elnXLR8qBXTNkxXfvy+OP3iMrN24Xj88rPq8/lsxbuKQ8BuUvuARfookA7SP032gGIaAzmN/65PSZcm27+4JCZK+0N+SUOzIAN4/eIH1BdmWmy96t38mgvyEiWfLjWqQbyIhXkGfvR1bM89+3suAx0YqflVUbt0uVyyluXZOuD/ddGmv4S/rnlM+cJm8YFaF/uhge4DIBEGgVhjuHii52EUlfvEbGz/pSgK3A5J7D588wZPJqsNbMflA2LxslW9Z+ISNee03E8a6I5ItIgaE+HRORo8bfp0XkuIgcljHvD5WyynKxu+wCvBRPHlAdV7iI0giuNDIAQqWRAp81tXIDiCrj74AWaU566DBkj3GvvulPICsnIuszkMxFPWTzN4Nl88oPZdOKmfJ0J0TkpIisEpFpInJGdq1FDuzoKJWFaca5oyLyqcxa+J2cLiyQt94b7QzHbdGIKmNLja9pHBVyMdC2lr55uikKhFN+tTCUbA5RmV+ObAe0AIZf2R6SG4OtQQ8ksSW6SsGtO+nyYF8eeQJydrVi4Wf3o9QLKHUp3f8Lvl6wHafjFIe2twLnJKZOWs4df7weTatizPC3lwOlcVWgBr3+9n8mjh11g5FvZwDP1ND1JV3kX5oR8twmX1bGntkcEgOyvC1EnldKbZ75DJ1bdoLkZjdjTfkDPutlaL5EPF4LHl8jdJ8NrycJklOx+pLZsTOb0R98DuTSrxdMnw3pc+ZyU5vrSbKB1evllvad+oTboqsYlkRfjIklK/UnIDNMv2kukec1U03CZyQJ1TmCqS4RKNUlhVSBlFJ89SakXg+D+8G+CIPr/WI3Ot/WFVujVBo2bERiQgIJicnYEizYbEkk2Sx06viXCqAlUBVvBWpPNVP6f0YAfwr9eGNw2qDeoVWoQJ0yaO/g9T9ZN+0jQgei2+D+frD6qzV06NyFd9/8nAnpNXOa3Qmn88Cj+XDYXXg0jWuvTqWiElzOKtauz0vy6rL1zJmSg1dc3vTsvfdemQeMB+zRkuM95lDl8keKV0xAdgQ2OQ1CLBOREhE5IyJFPpEir0iRSyTPK5LnFinQ/P9WYhClWZafuumkjAZZ9N650vvqFdl1XLUe+EksBdlNLw5JKzB7Q65d+wg4BuSfKHdkuUU6B0TXwEZA18Crga6DrkDpYFEhhdwQIi0pKyYBcIc47cMdQakzdVqxnjOnc+dYt9KTReTvodtln6k46w4c+/ytupQScPEAJyiwWP0SfaIBarLR56u9FXx+SypPvghN2sKHA+FH47OT7P/s4LobbsHnK8fpdOLx2HG5vPhEQ9f8KCtlQdd1Q3L34vUqlNKxWCz4vF4sFgtW8eKzWHn3vacLZ2XktIgFhKtGfzjpwFtpAxsGAPCEAFFdgPH5C7ShnxwoMwgWEPUzMaaY3FIpxeQH4Ir7oPHvwNqoJ127LyH3+CoqKosoqSig8mwZTmchdrsLr88L2PF4ErFYnFiUE01PwqISsVid6JoNq9WJ1ZqM1+siISEFn89Dv35rXgfGxyqqPJenyfTmCcHs7zUBEiDCIC8Q/8TBvywsBggWU3RINoHgAlKUWgh0evVOfvfRNiqBw7WMq2kYbm0CXATQpw+4PWBLhEQbpE9HAxYb2okzHmUpwyPSO6AKeUOKMAKIITZqEhKLA3xgAOAzRp5ogGD72RMWA38NRB3gNPH9H4cGwI0GzuZUJMt8v3jqDs/alEr0iDxpDdEFPIGXb/H/WkNIx0yAYsoVzBUp9bfhh4HnTF2P/gLucwA76/qbJXO6vdoh8udwnmBOk1WYokug+UwJkw24qlu/E4Vr07sAufyK9kuE1mQgo1jk8QYmALQw1aag6GBSkwKZowCNldoJ9ADyucBMAaM2/HQ86DsFc6swbaErQz7kcIrIgTK7AHNMUfKCtYeB3esPHj2n3mBugfJZiYhw8x0OYEaY9Ps3eZN1aa2BR4E7ad2ize9aXtO8c9u7G2/bf6j8vjv+eFIpnFNHDy8z3vwGoKA+vMH/B0q6RYv1PpSJAAAAAElFTkSuQmCC\" alt=\"Boolog Logo\" />".to_string());
    builder.append("&nbsp;".to_string());
    builder.append(title);
    builder.append("</h1>\r\n<hr>\r\n<small><i>Powered by Boolog...</i></small>\r\n\r\n".to_string());

    builder.to_string()
}

pub fn highlight_with_style(message: &str, style: &str) -> String {
    return format!("<p class=\"{style} outlined\">&nbsp;{message}&nbsp;</p>")
}

pub fn highlight(message: &str) -> String {
    return highlight_with_style(message, "highlighted");
}

pub fn ecapsulation_tag() -> String {
    let tag = Uuid::new_v4().to_string();
    format!("lvl-{tag}")
}

pub fn wrap_as_subordinate(boolog_title: String, boolog_content: String, style: String) -> String {
    let identifier = Uuid::new_v4().to_string();
    let tag = ecapsulation_tag();
    format!("\r\n\r\n<div class=\"boolog {style}\">\r\n<label for=\"{identifier}\">\r\n<input id=\"{identifier}\" class=\"gone\" type=\"checkbox\">\r\n<h2>{boolog_title}</h2>\r\n<div class=\"{tag}\">\r\n{boolog_content}\r\n</div></label></div>")
}