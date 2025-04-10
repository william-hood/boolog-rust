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

use std::fs;
use std::fs::File;
use std::io::Write;
use std::string::ToString;
use chrono::{DateTime, Local};
use rstring_builder::StringBuilder;
use uuid::Uuid;
use crate::constants::{ALREADY_CONCLUDED_MESSAGE, EMOJI_BOOLOG, EMOJI_DEBUG, EMOJI_ERROR, EMOJI_TEXT_BLANK_LINE, EMOJI_TEXT_BOOLOG_CONCLUDE};

const STARTING_CONTENT: &str = "<table class=\"left_justified\">\r\n";

/// Implements a rich logging system that outputs directly to HTML, with counterpart output to the console or a text file.
pub struct Boolog<'a> {
    title: &'a str,
    for_plain_text: Option<File>,
    for_html: Option<File>,
    show_time_stamps: bool,
    show_emojis: bool,
    content: Vec<u8>,
    is_concluded: bool,
    first_echo: bool
}

impl<'a> Boolog<'a> {
    pub fn new(
        title: &'a str,
        plain_text: Option<String>,
        html: Option<String>,
        theme: &str,
        show_time_stamps: bool,
        show_emojis: bool,
        header_function: fn(&str) -> Vec<u8>
    ) -> Boolog<'a> {
        let for_plain_text: Option<File> = match plain_text {
            Some(ref x) => Some(fs::OpenOptions::new().write(true).create(true).truncate(true).open(&x).unwrap()),
            None => None,
        };
        let for_html: Option<File> = match html {
            Some(ref x) => {
                let mut html_file = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&x).unwrap();
                html_file.write("<html>\r\n<meta charset=\"UTF-8\">\r\n<head>\r\n<title>".as_bytes());
                html_file.write(title.as_bytes());
                html_file.write("</title>\r\n".as_bytes());
                html_file.write(theme.as_bytes());
                html_file.write("</head>\r\n<body>\r\n".as_bytes());
                html_file.write(header_function(title).as_slice());
                Some(html_file)
            },
            None => None,
        };

        let mut result = Boolog {
            title,
            for_plain_text,
            for_html,
            show_time_stamps,
            show_emojis,
            content: Vec::new(),
            is_concluded: false,
            first_echo: true
        };

        result.content.append(STARTING_CONTENT.as_bytes().to_vec().as_mut());

        result
    }

    /// Returns true if this Boolog has been used.
    pub fn was_used(&mut self) -> bool {
        (self.content.len() - STARTING_CONTENT.len()) > 0
    }

    pub fn echo_plain_text(&mut self, message: &[u8], emoji: &[u8], timestamp: DateTime<Local>) -> Result<(), String> {
        if self.is_concluded {
            Err::<(), String>(ALREADY_CONCLUDED_MESSAGE.to_string());
        }

        if self.first_echo {
            self.first_echo = false;
            self.echo_plain_text("".as_bytes(), EMOJI_TEXT_BLANK_LINE, timestamp);
            self.echo_plain_text(self.title.as_bytes(), EMOJI_BOOLOG, timestamp);
        }

        let formatted_date = format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S.%9f"));

        match self.for_plain_text {
            Some(ref mut text_file) => {
                if self.show_time_stamps {
                    text_file.write(formatted_date.as_bytes());
                    text_file.write("\t".as_bytes());
                }

                if self.show_emojis {
                    text_file.write(emoji);
                    text_file.write("\t".as_bytes());
                }

                text_file.write(message);
                text_file.write("\r\n".as_bytes());
            },
            None => {
                if self.show_time_stamps {
                    print!("{}", formatted_date);
                    print!("\t");
                }

                if self.show_emojis {
                    print!("{}", String::from_utf8_lossy(emoji));
                    print!("\t");
                }

                print!("{}", String::from_utf8_lossy(message));
                print!("\r\n");
            }
        }

        Ok(())
    }

    pub fn write_to_html(&mut self, message: &[u8], emoji: &[u8], timestamp: DateTime<Local>) -> Result<(), String> {
        if self.is_concluded {
            Err::<(), String>(ALREADY_CONCLUDED_MESSAGE.to_string());
        }

        self.content.append("<tr>".as_bytes().to_vec().as_mut());

        if self.show_time_stamps {
            let formatted_date = format!("{}", timestamp.format("%Y-%m-%d"));
            let formatted_time = format!("{}", timestamp.format("%H:%M:%S.%9f"));
            self.content.append("<td class=\"min\"><small>".as_bytes().to_vec().as_mut());
            self.content.append(formatted_date.as_bytes().to_vec().as_mut());
            self.content.append("</small></td><td>&nbsp;</td><td class=\"min\"><small>".as_bytes().to_vec().as_mut());
            self.content.append(formatted_time.as_bytes().to_vec().as_mut());
            self.content.append("</small></td><td>&nbsp;</td>".as_bytes().to_vec().as_mut());
        }

        if self.show_emojis {
            self.content.append("<td><h2>".as_bytes().to_vec().as_mut());
            self.content.append(emoji.to_vec().as_mut());
            self.content.append("</h2></td>".as_bytes().to_vec().as_mut());
        }

        self.content.append("<td>".as_bytes().to_vec().as_mut());
        self.content.append(message.to_vec().as_mut());
        self.content.append("</td></tr>\r\n".as_bytes().to_vec().as_mut());

        Ok(())
    }

    /// Concludes this Boolog. All buffered HTML is written to the file if one is associated with it. Once concluded, this Boolog becomes read only.
    pub fn conclude(&mut self) -> Vec<u8> {
        if !self.is_concluded {
            self.is_concluded = true;

            let timestamp = Local::now();
            self.echo_plain_text("".as_bytes(), EMOJI_TEXT_BOOLOG_CONCLUDE, timestamp);
            self.echo_plain_text("".as_bytes(), EMOJI_TEXT_BLANK_LINE, timestamp);

            match self.for_plain_text {
                Some(ref mut text_file) => {
                    text_file.flush();
                },
                None => { }
            }

            self.content.append("\r\n</table>".as_bytes().to_vec().as_mut());


            match self.for_html {
                Some(ref mut html_file) => {
                    html_file.write(self.content.as_slice());
                    html_file.write("\r\n</body>\r\n</html>".as_bytes());
                    html_file.flush();
                },
                None => {  },
            }
        }

        self.content.clone()
    }

    /// This is the standard way to output text through a Boolog. No emoji will be added to the output line.
    pub fn info(&mut self, message: &str) -> Result<(), String> {
        return self.info_detailed(message, EMOJI_TEXT_BLANK_LINE);
    }

    /// This outputs text to this Boolog, but allows you to specify an emoji to appear next to the line.
    pub fn info_detailed(&mut self, message: &str, emoji: &[u8]) -> Result<(), String> {
        let timestamp = Local::now();
        {
            let result = self.write_to_html(message.as_bytes(), emoji, timestamp);
            if result.is_err() {
                return result;
            }
        }

        return self.echo_plain_text(message.as_bytes(), emoji, timestamp);
    }

    /// Use this to output a highlighted debugging message. The HTML output will highlight the text in yellow (or orange, depending on the theme used). Both HTML and Plaintext will output the line with a debugging emoji icon.
    pub fn debug(&mut self, message: &str) -> Result<(), String> {
        let timestamp = Local::now();
        let result = self.write_to_html(highlight(message).as_bytes(), EMOJI_DEBUG, timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text(message.as_bytes(), EMOJI_DEBUG, timestamp);
    }

    /// Use this to output a highlighted error message. The HTML output will highlight the text in yellow (or orange, depending on the theme used). Both HTML and Plaintext will output the line with an error emoji icon.
    pub fn error(&mut self, message: &str) -> Result<(), String> {
        let timestamp = Local::now();
        let result = self.write_to_html(highlight(message).as_bytes(), EMOJI_ERROR, timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text(message.as_bytes(), EMOJI_ERROR, timestamp);
    }

    /// Skips a line in the Boolog output. The skipped line will still include a timestamp.
    pub fn skip_line(&mut self) -> Result<(), String> {
        let timestamp = Local::now();
        let result = self.write_to_html("".as_bytes(), EMOJI_TEXT_BLANK_LINE, timestamp);
        if result.is_err() {
            return result;
        }

        return self.echo_plain_text("".as_bytes(), EMOJI_TEXT_BLANK_LINE, timestamp);
    }

    /// Embeds another Boolog into this one as a subordinate. (The subordinate Boolog will be concluded and become read-only.) The HTML output will show the subordinate as a click-to-expand section.
    pub fn show_boolog(&mut self, subordinate: Boolog) -> Result<Vec<u8>, String> {
        return self.show_boolog_detailed(subordinate, EMOJI_BOOLOG, "boolog", 0);
    }

    /// Embeds another Boolog into this one as a subordinate. (The subordinate Boolog will be concluded and become read-only.) This verion of the function allows you to specify an emoji icon and a theme. Use 0 for the recurseLevel. The HTML output will show the subordinate as a click-to-expand section.
    pub fn show_boolog_detailed(&mut self, mut subordinate: Boolog, emoji: &[u8], style: &str, recurse_level: u8) -> Result<Vec<u8>, String> {
        let timestamp = Local::now();
        let subordinate_content: Vec<u8> = subordinate.conclude();
        let result = wrap_as_subordinate(subordinate.title, subordinate_content, style);

        if recurse_level < 1 {
            let check = self.write_to_html(result.as_slice(), emoji, timestamp);
            if check.is_err() {
                Err::<Vec<u8>, String>(check.err().unwrap());
            }
        }

        Ok(result)
    }
}

/// Pass this function in for the header function when creating a Boolog. This will use the standard, default header. You may also use no_header() or you may write your own.
pub fn default_header(title: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut builder: StringBuilder = StringBuilder::new();
    builder.append("<h1>".to_string());
    builder.append("<img src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEEAAABBCAYAAACO98lFAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAdnJLH8AAAAgY0hSTQAAeiYAAICEAAD6AAAAgOgAAHUwAADqYAAAOpgAABdwnLpRPAAAAAZiS0dEAAAAAAAA+UO7fwAAAAlwSFlzAAAXEgAAFxIBZ5/SUgAAAAd0SU1FB+kDBwQ3FkQyHzoAAA80SURBVHja1Zt5fBRVtse/tzvpJIAEQRHFwXFDVBjFQRQFB+fhNsgTP+P49CkgiiIjggZXPqiDyDKAKDIoIAHDPjxERJR9CwgEBGQfCYQlkI0sZOm1qvu8P7o6Vjed9GLQcD6f++kUdavq3l+d+zvn/k6hOP9mBToDNwC/B654a0jaNSkpKa1u79ixeXJKstbkkkudaHDoyAH7lLc/3rHpxM4vgfVAMRe4dQbm7Ni2rVLisJyjxzzAfOCyC3HyNwObpI4sa8u2AuDeCwmA1zM3ZTqljs0nIkC/CwGAcWeLi+R8mdPh9AA96zMA78mvYP8cN85hkGu9s3sL8k/55FcyIL0+grA1ysFXt0PHjsQNQmFpiQ+4rj4BcGtO9pGIXnDiVF4QCEBEoKZOn14boEPqEwgZsXpBbUBEA5TRb1l9AcA6uE/fkroCYdyEj885f6qwpKb7ueoLCD1iILOIIIQ7763hfosWLhagcX0A4cNoAJgwcWLYCR4+djwiCBGA7VgfQNhyPvkgChDurquJWOK87rpjOTl3xvvQFwb0Dzp2+rwxXX809zRA5W/tBW/EmODU+obP2l0xecGz/QfUC2JcG0emFynsVbfBg16NdK81dTmZhHiuuf+eWzvFepF/7JHP55eUcXmziyPdbt1v7QUdqlya/FZWUFkpwG11OaF4iPHahkk/O5AP8AK68StR3mRN5maUUjE/vMVFFx0Fdv3WnvBZqODhFRHdaIHjWMgyRm6p831DPJxwq/lAAZrp2Gcoq7VZqAcopSJyhtHvB+Aj4/Am4AHgob+/OawpXp3V/8o4ke3K/wb4Cig/n55wyuwFuoh4TM3pE3FH8IZYEyMRkQK7LsBcYMPEaXNcBVU189JLae85gBHnC4BLZsycGfRAr4g4Tc3lE9HiyCLPkwDzf0BSRA+LNTKIyI7AgddwfxF/U4BS4FP+JZEQ5XKIJoTGa0qpb4HHAUddRYcbQhFUBhhe9XNTvyAfD7WKKPqUSK35R3egT10SY6PQ0Ogz3r6SYGB0wBblTXceyK7RW24HdphD61fzKDmTj+aswu6uov8b4/ihwEGzy1Jqe8SgtLubd/SmtqZKv4SkZk35dP4MAUYD2bEuh/4iMuWc5QBopuWgjAiRGOVyCF0KSimWfnI9iQ3aQcIVWBIEp7OCns/PRqQKaGj01CjYv50VlZfxTKfwsqMG2NQ9iGSGG8d+4A+xeuc/AqSjiz8KOEXEYbQAOVYZvzUR5O5DhwWQb9dlnnNu8crN8hLIy83Cb8FHp/UOoWW3fD33M/lkxaFzCja6iGRViKxdsrA28nwwVhBGmR+iGUC4jIlXiYjdAMQlkaNEOHtr2EvVE5742l+NyFEUFFmCzZ+i5e7PEkBuGv61vLP0gJSKyPQ9ZdLhgacjRZAJsYLQK5InBAAJAOGJc7dpbuu/HSeiH5azxduiDqdLM2ZJaW5+xH5D0kZuiZXED5nDisVY+75Ak58zRouJJMU4H43d2bXrOf92zXWNObBvJfv2fhO9ANq7Fxdf2SJiv9s7tv19rCAcGDVuvJ/MTJOzBpoCu9uDNeS8HsPGauv69edGj+39ydryMuvXjQxDbpejVDuUUuzcmYNSCqX6GL9dUWowSilatBmKUorFSzYGXd+wUUpTFUfo3iUi7TF5gBgTVcDUT6dTkF/A+yOGVWdigZBpMbVID1ZKFQBPAc6QU+kicmPgwO4GtxvcHi9r1h2k1+PtYprMmLHT9saTv4wx84LH4Aa38bcmImMmTZFBLw8J2lNoRtNNffXauSFsgmOjR05dptZd7n81M55d5OYih/5m8wYJWE1rXjNxQNrA/sxbsAibUnhFzlkOyuQJEsYrBr076SQwK9zDH+t+eStM1+YWw7CPs3D6LNj1i1h+chusO83I8R5SbMdZv6cvD3Voz//2TCU1zPZ206qP9scrxOw2R2qthrd9+HSBALJ4xepqvUE3oobZc8J4wdgant3mp737xGs89x/9kKMTETmOiNTecrciYx9HFn7zdZ3pE203bN9tN98s3LLQjcGmDR0mgHz/w/5qoDSTCGO2cv/AatIw3/E4i8WjOUUXkbFD20nxrch6kD0gu0C2g3wPsgBkB8jhvyDZ3ZDpIJKPDDeF2FNFIkBbRfw2MN+lT2qRZA0SVAghSnObMjWDgS8+w66fjtO29VXVGy1ziHrgwV7lq1bOuTREq8EIQCcdZ7Kv8CU1wZrUmASbjbIqWLDsCKX5J7EeWUfnu5phLz2De/9oWrcER5F/MLaLYUce6N2yGNCrY4B894SKRPHYsBPldl9oqqqFtNAlMH/J8qBkaNOOPXKitCrgnitqeNZ9Bzd+IiUnt0hFcbY4HcW1EmuUidmT8egJ4ax7lzvumpO57fsmhMhsWohooUK23wpwA4sWLaWsrIx1m7aybPZUxyPPPX/opyLo1qkdjS5u5RkzYHk+TG31wTOpHfoNmYk1tSVJjS7F1rA5ydd+wdxJj7DvSAE9H7oNpxtSUy1oOhzMzqPNNS3IPV1I9vEjDH2+S/X4tu0+Xt7ptqsvD4TgOwyt7hXgdaAv0B1oEgMQLwKyNutHCSXMAGmaN1iukDDpq4Efwtn7Q3tLQfYKKS/aI5kblgopGQKz5LURu+WpV7fJDwdFPpicI8cKw7z5O2eYvWCRmelv6N3r+XbLVq271S7yjojMEJFlpW4pHjf504VAmyhA+AIod1SVcmOn9hw8VYjFlDH6TAmSJSDCmKR6LYaM8rEPMjhyYAtup517uv43OE8DOYx/ZyVzM/PocNN8XA47V182HaXGodTLZG53A/A/twRpr/+uSV5rBnwgIi8E+KpSw9nYpnoAayOMb76IPAEwZe5sBjzdm10/5dCu9dVBZBlueQSAsZj2HbXZA0oxIWs+s2d/S+MW19OgyaUkW5uSf8ZBhW6ltEJj1sdH8H8Z3Ay4FliNx7eHRGXj0acGZC+ZN+XmMOQbTEJ7j58sD7jOnH8vOQY0jUJwCdrgjv3kMwHk0xkZ4jItA7dpy+00jrUo6xWVxk7zxL4VUnBsn5QU5krp2RJxOFxi93jF6RVx+4KXWn7+GZk07QvzUng42nV+V7l/nIEL50UqDs1YMM9tBiHAB9uPHKuOBKu37KqRF6Iq2jw6TfZsnCqHdn8nC+bMlnf7IYd2DpapE5rJ6m/elnXLR8qBXTNkxXfvy+OP3iMrN24Xj88rPq8/lsxbuKQ8BuUvuARfookA7SP032gGIaAzmN/65PSZcm27+4JCZK+0N+SUOzIAN4/eIH1BdmWmy96t38mgvyEiWfLjWqQbyIhXkGfvR1bM89+3suAx0YqflVUbt0uVyyluXZOuD/ddGmv4S/rnlM+cJm8YFaF/uhge4DIBEGgVhjuHii52EUlfvEbGz/pSgK3A5J7D588wZPJqsNbMflA2LxslW9Z+ISNee03E8a6I5ItIgaE+HRORo8bfp0XkuIgcljHvD5WyynKxu+wCvBRPHlAdV7iI0giuNDIAQqWRAp81tXIDiCrj74AWaU566DBkj3GvvulPICsnIuszkMxFPWTzN4Nl88oPZdOKmfJ0J0TkpIisEpFpInJGdq1FDuzoKJWFaca5oyLyqcxa+J2cLiyQt94b7QzHbdGIKmNLja9pHBVyMdC2lr55uikKhFN+tTCUbA5RmV+ObAe0AIZf2R6SG4OtQQ8ksSW6SsGtO+nyYF8eeQJydrVi4Wf3o9QLKHUp3f8Lvl6wHafjFIe2twLnJKZOWs4df7weTatizPC3lwOlcVWgBr3+9n8mjh11g5FvZwDP1ND1JV3kX5oR8twmX1bGntkcEgOyvC1EnldKbZ75DJ1bdoLkZjdjTfkDPutlaL5EPF4LHl8jdJ8NrycJklOx+pLZsTOb0R98DuTSrxdMnw3pc+ZyU5vrSbKB1evllvad+oTboqsYlkRfjIklK/UnIDNMv2kukec1U03CZyQJ1TmCqS4RKNUlhVSBlFJ89SakXg+D+8G+CIPr/WI3Ot/WFVujVBo2bERiQgIJicnYEizYbEkk2Sx06viXCqAlUBVvBWpPNVP6f0YAfwr9eGNw2qDeoVWoQJ0yaO/g9T9ZN+0jQgei2+D+frD6qzV06NyFd9/8nAnpNXOa3Qmn88Cj+XDYXXg0jWuvTqWiElzOKtauz0vy6rL1zJmSg1dc3vTsvfdemQeMB+zRkuM95lDl8keKV0xAdgQ2OQ1CLBOREhE5IyJFPpEir0iRSyTPK5LnFinQ/P9WYhClWZafuumkjAZZ9N650vvqFdl1XLUe+EksBdlNLw5JKzB7Q65d+wg4BuSfKHdkuUU6B0TXwEZA18Crga6DrkDpYFEhhdwQIi0pKyYBcIc47cMdQakzdVqxnjOnc+dYt9KTReTvodtln6k46w4c+/ytupQScPEAJyiwWP0SfaIBarLR56u9FXx+SypPvghN2sKHA+FH47OT7P/s4LobbsHnK8fpdOLx2HG5vPhEQ9f8KCtlQdd1Q3L34vUqlNKxWCz4vF4sFgtW8eKzWHn3vacLZ2XktIgFhKtGfzjpwFtpAxsGAPCEAFFdgPH5C7ShnxwoMwgWEPUzMaaY3FIpxeQH4Ir7oPHvwNqoJ127LyH3+CoqKosoqSig8mwZTmchdrsLr88L2PF4ErFYnFiUE01PwqISsVid6JoNq9WJ1ZqM1+siISEFn89Dv35rXgfGxyqqPJenyfTmCcHs7zUBEiDCIC8Q/8TBvywsBggWU3RINoHgAlKUWgh0evVOfvfRNiqBw7WMq2kYbm0CXATQpw+4PWBLhEQbpE9HAxYb2okzHmUpwyPSO6AKeUOKMAKIITZqEhKLA3xgAOAzRp5ogGD72RMWA38NRB3gNPH9H4cGwI0GzuZUJMt8v3jqDs/alEr0iDxpDdEFPIGXb/H/WkNIx0yAYsoVzBUp9bfhh4HnTF2P/gLucwA76/qbJXO6vdoh8udwnmBOk1WYokug+UwJkw24qlu/E4Vr07sAufyK9kuE1mQgo1jk8QYmALQw1aag6GBSkwKZowCNldoJ9ADyucBMAaM2/HQ86DsFc6swbaErQz7kcIrIgTK7AHNMUfKCtYeB3esPHj2n3mBugfJZiYhw8x0OYEaY9Ps3eZN1aa2BR4E7ad2ize9aXtO8c9u7G2/bf6j8vjv+eFIpnFNHDy8z3vwGoKA+vMH/B0q6RYv1PpSJAAAAAElFTkSuQmCC\" alt=\"Boolog Logo\" />".to_string());
    builder.append("&nbsp;".to_string());
    builder.append(title);
    builder.append("</h1>\r\n<hr>\r\n<small><i>Powered by Boolog...</i></small>\r\n\r\n".to_string());

    result.append(builder.to_string().into_bytes().as_mut());
    result
}

/// Pass this function in for the header function when creating a Boolog if there is no need for a header. Use this for a sublog or any Boolog that does not have an HTML file.
pub fn no_header(_: &str) -> Vec<u8> {
    Vec::new()
}

/// Use this to highlight a message and send the result through the .info() method. The HTML output will highlight the text in yellow (or orange, depending on the theme used).
pub fn highlight(message: &str) -> String {
    return highlight_with_style(message, "highlighted");
}

/// More detailed version of .highlight() that lets you override the style. Send the result through the .info() method.
pub fn highlight_with_style(message: &str, style: &str) -> String {
    return format!("<p class=\"{style} outlined\">&nbsp;{message}&nbsp;</p>")
}

pub fn ecapsulation_tag() -> String {
    let tag = Uuid::new_v4().to_string();
    format!("lvl-{tag}")
}

pub fn wrap_as_subordinate(boolog_title: &str, boolog_content: Vec<u8>, style: &str) -> Vec<u8> {
    let identifier = Uuid::new_v4().to_string();
    let tag = ecapsulation_tag();
    let mut result: Vec<u8> = Vec::new();
    result.append("\r\n\r\n<div class=\"boolog ".as_bytes().to_vec().as_mut());
    result.append(style.as_bytes().to_vec().as_mut());
    result.append("\">\r\n<label for=\"".as_bytes().to_vec().as_mut());
    result.append(identifier.as_bytes().to_vec().as_mut());
    result.append("\">\r\n<input id=\"".as_bytes().to_vec().as_mut());
    result.append(identifier.as_bytes().to_vec().as_mut());
    result.append("\" class=\"gone\" type=\"checkbox\">\r\n<h2>".as_bytes().to_vec().as_mut());
    result.append(boolog_title.as_bytes().to_vec().as_mut());
    result.append("</h2>\r\n<div class=\"".as_bytes().to_vec().as_mut());
    result.append(tag.as_bytes().to_vec().as_mut());
    result.append("\">\r\n".as_bytes().to_vec().as_mut());
    result.append(boolog_content.clone().as_mut());
    result.append("\r\n</div></label></div>".as_bytes().to_vec().as_mut());
    result
}

/// Pass the result of this to .info() (or any other method that outputs to HTML) and the text will show as a monospace font, keeping any line breaks or other formatting used.
pub fn treat_as_code(value: &str) -> String {
    format!("<pre><code><xmp>{value}</xmp></code></pre>")
}

/// Use this for any method that requires a callback function passed in if you don't want/need to write your own.
pub fn callback_do_nothing(field_name: &str, field_value: &str) -> Vec<u8> {
    field_value.as_bytes().to_vec()
}