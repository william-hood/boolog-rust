use std::string::ToString;

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
pub const THEME_DARK: String = "
    <style>
        html {
            font-family: sans-serif
        }

        [class*='lvl-'] {
            display: none;
            cursor: auto;
        }

        input:checked~[class*='lvl-'] {
            display: block;
        }

        .gone {
            display: none;
        }

        .boolog {
            font-family: sans-serif;
            border-radius: 0.25em;
            border: 0.5em solid GhostWhite;
            display: inline-block;
        }

        .failing_test_result {
            border: 0.5em solid #F62817;
            color: #F62817;
            background-color: #210002;
        }

        .inconclusive_test_result {
            border: 0.5em solid #FFEF00;
            color: #FFEF00;
            background-color: #212000;
        }

        .passing_test_result {
            border: 0.5em solid #59E817;
            color: #59E817;
            background-color: #012100;
        }

        .implied_good {
            border: 0.5em solid #00FA9A;
            color: #00FA9A;
            background-color: #012100;
        }

        .implied_caution {
            border: 0.5em solid #FFFFC2;
            color: #FFFFC2;
            background-color: #212000;
        }

        .implied_bad {
            border: 0.5em solid #FA8072;
            color: #FA8072;
            background-color: #210002;
        }

        .neutral {
            border: 0.5em solid #E0FFFF;
            color: #E0FFFF;
        }

        .old_parchment {
            border: 0.5em solid #8A9A5B;
            color: #8A9A5B;
        }

        .plate {
            border: 0.5em solid #838996;
            background-color: #2B3856;
        }

        .exception {
            border: 0.5em solid #E8A317;
            color: #FFFF33;
            background-color: #660000;
        }


        body {
            background-color: #000000;
            color: GhostWhite;
        }
        table,
        th,
        td {
            padding: 0.1em 0em;
            margin-left: auto;
            margin-right: auto;
        }

        td.min {
            width: 1%;
            white-space: nowrap;
        }

        h1 {
            font-size: 3em;
            margin: 0em
        }

        h2 {
            font-size: 1.75em;
            margin: 0.2em
        }

        hr {
            border: none;
            height: 0.3em;
            background-color: GhostWhite;
        }

        .centered {
            text-align: center;
        }

        .highlighted {
            background-color: #FFFF33;
            color: #FF4500;
            font-weight: bold
        }

        .outlined {
            display: inline-block;
            border-radius: 0.5em;
            border: 0.05em solid Orange;
            padding: 0.2em 0.2em;
        }

        .object {
            border-radius: 1.5em;
            display: inline-block;
            padding: 0.4em 0.4em;
        }

        .incoming {
            border-radius: 3em 0.5em 0.5em 3em;
            display: inline-block;
            padding: 1em 1em;
        }

        .outgoing {
            border-radius: 0.5em 3em 3em 0.5em;
            display: inline-block;
            padding: 1em 1em;
        }

        .left_justified {
	        float: left;
        }

        table.gridlines,
        table.gridlines th,
        table.gridlines td {
            padding: 0.4em 0.4em;
            border-collapse: collapse;
            border: 0.1em solid #6698FF;
        }

        label {
            cursor: pointer;
        }
    </style>
".to_string();