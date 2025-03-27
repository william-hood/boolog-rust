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

mod theme_none;
mod theme_classic;
mod theme_light_flat;
mod theme_light;
mod theme_dark;
mod theme_dark_flat;
mod theme_dark_gradient;
mod constants;
mod boolog;
mod show_as_json;
mod show_http_messages__reqwest;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::path::PathBuf;
    use homedir::my_home;
    use serde::Serialize;
    use crate::boolog::{callback_do_nothing, default_header, no_header, Boolog};
    use crate::constants::EMOJI_BOOLOG;
    use crate::show_as_json::ShowObjectExt;
    use crate::show_http_messages__reqwest::ShowHttpViaReqwestExt;
    use crate::theme_light::THEME_LIGHT;
    use crate::theme_none::THEME_NONE;

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

        let check = get_test_struct();
        log.show_as_json(check, "TestStruct", "check");
        log.skip_line();

        log.info("Boolog can be very useful for testing HTTP Requests. Let's use Golang's standard HTTP client to send a request and get a response.");

        let client = reqwest::blocking::Client::new();
        let req = client.get("https://httpbin.org/get?param1=latida&param2=tweedledee&param3=whatever").build().unwrap();
        log.show_http_transaction_blocking(req, callback_do_nothing);
        log.skip_line();

        log.info("Boolog also has a .show_as_error() function for error types.");

        let check = File::open("nonexistent_file.txt");
        match check {
            Ok(mut file) => {log.error("Someone actually created a file called \"shouldnt exist.xyz\". Delete it."); ()}
            Err(error) => {log.show_as_error(serde_error::Error::new(&error), "Tried opening nonexistent_file.txt"); ()}
        }

        log.skip_line();

        log.debug("complex boolog here");





        let mut sublog = Boolog::new(
            "Keep on embedding Boologs within Boologs within Boologs",
            None,
            None,
            THEME_NONE,
            true,
            true,
            no_header
        );

        sublog.info("The other day");
        sublog.info("Upon the stair");
        sublog.info("I saw a man");
        sublog.info("Who wasn't there");
        sublog.skip_line();
        sublog.info("He wasn't there");
        sublog.info("Again today...");
        sublog.skip_line();
        sublog.error("Gee, I wish he'd go away!");

        log.show_boolog_detailed(sublog, EMOJI_BOOLOG, "neutral", 0);

        log.debug("One caveat: If you .conclude() a Boolog, it's done. That function closes any output streams and makes it read-only.");
        log.info("A Boolog also gets concluded if you embed it in another Boolog with the .show_boolog() function.");
        log.skip_line();
        log.info("Well, that's the demo. Go forth and do great things!");

        log.conclude();
    }

    #[derive(Serialize)]
    struct TestStruct {
        name: String,
        value: i32,
        other_value: f32,
        // Type recursion in Rust is illegal by design
        //child: Option<TestStruct>,
        troll: String,
        rogue: HashMap<String, String>
    }

    fn get_test_struct() -> TestStruct {
        let mut rogue = HashMap::new();
        rogue.insert(String::from("LOTR"), String::from("Sauron"));
        rogue.insert(String::from("Star Wars"), String::from("Darth Vader"));
        rogue.insert(String::from("It"), String::from("Pennywise"));

        TestStruct {
            name: String::from("Hi"),
            value: 7,
            other_value: 42.9,
            //child: None,
            troll: String::from("(nothing)"),
            rogue
        }
    }
}
