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
pub const THEME_LIGHT_FLAT: &str = "
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
            display: inline-block;
            background-color: #DFDFDF;
        }

        .failing_test_result {
            background-color: #E55451;
        }

        .inconclusive_test_result {
            background-color: #FFFFC2;
        }

        .passing_test_result {
            background-color: #C3FDB8;
        }

        .implied_good {
            background-color: #A0D6B4;
        }

        .implied_caution {
            background-color: #FFFFC2;
        }

        .implied_bad {
            background-color: #FA8072;
        }

        .neutral {
            background-color: #DBE9FA;
        }

        .old_parchment {
            background-color: #FFE5B4;
        }

        .plate {
            background-color: #B0CFDE;
        }

        .exception {
            background-color: #FF4500;
        }


        body {
            background-color: #FFFFFF;
            color: #000000;
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
            background-color: #000000;
        }

        .centered {
            text-align: center;
        }

        .highlighted {
            background-color: #FFC600;
            color: #000000;
        }

        .outlined {
            display: inline-block;
            border-radius: 0.5em;
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
            border: 0.02em solid black;
            color: #000000;
        }

        label {
            cursor: pointer;
        }
    </style>
";