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

pub const EMOJI_SETUP: &[u8] = "🛠".as_bytes();
pub const EMOJI_CLEANUP: &[u8] = "🧹".as_bytes();
pub const EMOJI_PASSING_TEST: &[u8] = "✅".as_bytes();
pub const EMOJI_SUBJECTIVE_TEST: &[u8] = "🤔".as_bytes();
pub const EMOJI_INCONCLUSIVE_TEST: &[u8] = "🛑".as_bytes();
pub const EMOJI_FAILING_TEST: &[u8] = "❌".as_bytes();
pub const EMOJI_DEBUG: &[u8] = "🐞".as_bytes();
pub const EMOJI_ERROR: &[u8] = "😱".as_bytes();
pub const EMOJI_BOOLOG: &[u8] = "📝".as_bytes();
pub const EMOJI_TEXT_BOOLOG_CONCLUDE: &[u8] = "⤴️".as_bytes();
pub const EMOJI_TEXT_BLANK_LINE: &[u8] = "".as_bytes();
pub const EMOJI_OBJECT: &[u8] = "🔲".as_bytes();
pub const EMOJI_CAUSED_BY: &[u8] = "→".as_bytes();
pub const EMOJI_OUTGOING: &[u8] = "↗️".as_bytes();
pub const EMOJI_INCOMING: &[u8] = "↩️".as_bytes();
pub const UNKNOWN: &str = "(unknown)";

pub const NAMELESS: &str = "(name not given)";
pub const ALREADY_CONCLUDED_MESSAGE: &str = "An attempt was made to write to a Boolog that was already concluded.\r\n<li>Once a Boolog has been concluded it can no longer be written to.\r\n<li>Passing a Boolog to the ShowBoolog() method will automatically conclude it.";
pub const MAX_OBJECT_FIELDS_TO_DISPLAY: i32 = 10;
pub const MAX_SHOW_OBJECT_RECURSION: i32 = 10;
pub const MAX_HEADERS_TO_DISPLAY: i32 = 10;
pub const MAX_BODY_LENGTH_TO_DISPLAY: i32 = 500;