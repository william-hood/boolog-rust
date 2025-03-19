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

pub const EMOJI_SETUP: &str = "🛠";
pub const EMOJI_CLEANUP: &str = "🧹";
pub const EMOJI_PASSING_TEST: &str = "✅";
pub const EMOJI_SUBJECTIVE_TEST: &str = "🤔";
pub const EMOJI_INCONCLUSIVE_TEST: &str = "🛑";
pub const EMOJI_FAILING_TEST: &str = "❌";
pub const EMOJI_DEBUG: &str = "🐞";
pub const EMOJI_ERROR: &str = "😱";
pub const EMOJI_MEMOIR: &str = "📝";
pub const EMOJI_TEXT_MEMOIR_CONCLUDE: &str = "⤴️";
pub const EMOJI_TEXT_BLANK_LINE: &str = "";
pub const EMOJI_OBJECT: &str = "🔲";
pub const EMOJI_CAUSED_BY: &str = "→";
pub const EMOJI_OUTGOING: &str = "↗️";
pub const EMOJI_INCOMING: &str = "↩️";
pub const UNKNOWN: &str = "(unknown)";

pub const NAMELESS: &str = "(name not given)";
pub const ALREADY_CONCLUDED_MESSAGE: &str = "An attempt was made to write to a memoir that was already concluded.\r\n<li>Once a Memoir has been concluded it can no longer be written to.\r\n<li>Passing a Memoir to the ShowMemoir() method will automatically conclude it.";
pub const MAX_OBJECT_FIELDS_TO_DISPLAY: i32 = 10;
pub const MAX_SHOW_OBJECT_RECURSION: i32 = 10;
pub const MAX_HEADERS_TO_DISPLAY: i32 = 10;
pub const MAX_BODY_LENGTH_TO_DISPLAY: i32 = 500;