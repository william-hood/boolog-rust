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

mod theme_classic;
mod theme_light_flat;
mod theme_light;
mod theme_dark;
mod theme_dark_flat;
mod theme_dark_gradient;
mod constants;
mod boolog;


#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use homedir::my_home;
    use crate::boolog::{default_header, Boolog};
    use crate::theme_light::THEME_LIGHT;

    #[test]
    fn it_works() {
        let path: PathBuf = [my_home().unwrap().unwrap().to_str().unwrap(), "Documents", "Test Results", "Boolog Rust Example.html"].iter().collect();
        let mut log = Boolog::new(
            "Testing Boolog for Rust",
            None,
            Some(path.to_str().unwrap().to_string()),
            THEME_LIGHT,
            true,
            true,
            default_header
        );

        log.info("Boolog is an HTML-based rich logging system capable of visual renditions of HTTP requests & responses, errors, and any other struct or type. One Boolog instance can even embed another as a log subsection.");
        log.info("When used for debugging control flow, HTTP requests & responses, activity logging, or any other purpose, output from Boolog will easier to read and work with than ordinary console output (though it does provide counterpart output to the console in addition to its HTML log file).");
        log.info("All of the above messages represent \"normal\" log output with the .info() function.");
        log.info("When debugging a program, you might need a single line of information to stand out.");
        log.info("If you use the .debug() function instead of .info() the message will be highlighted in yellow like this...");
        log.debug("Boolog is the spritual successor to a similar Golang log system I created at work years ago!");
        log.info("Similar to that is the .error() function. The only difference is an icon in the log identifying the line as an error...");
        log.error("Uh-oh... That wasn't supposed to happen!");
        log.skip_line();
        log.info("Why would you want to log directly to HTML?");
        log.info("Because it's very hard, using ordinary plain-text logging, to visualize the workings of a cloud service, test suite, or other back-end process.");
        log.info("Let's suppose you need to check on the state of a data structure at a certain point in the program.");
        log.info("Look at the class \"TestStruct\" at the bottom of this source code file. Let's render one!");

        log.conclude();
    }
}
