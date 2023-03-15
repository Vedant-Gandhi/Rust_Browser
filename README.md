# Rust_Browser
This is just a simple effort to create an browser using Rust language which has a GUI and can parse HTML and CSS files easily. 

This project aims to demonstrate how a web browser parses HTML files to create a DOM and applies CSS to it.It is not as fast as the browsers used for everyday work but it is just a test sample.The code is easily readable so anyone who understands rust can easily read this code and also develop it by himself/herself.

I will soon update the readme accordingly as the updates happen

## Files
1 dom.rs-This files hold the elements which are useful generation of DOM .It provides various non primitive datatypes as well as methods used for handling of the DOM

2htmlparser.rs-This file is the html parse engine that will be used by the browser to create DOM.It can parse most the elements such as text,Comments and Data Elements.

3.csselements.rs-This file contains the datastructures and other necessary data to operate with the CSS file.This file has a longer length.

4.cssparser.rs-This file contains the parser engine for css files and is also optimized.

5.layouttree.rs-This file contains the datastructures and functions to generate a layout tree.This tree is useful as it contains the coordinates and other values of every node and it helps to determine which element fits where on the viewport

6.styletree.rs-This tree is generated from css parser tree and it helps to determine the actual dimensions and colors of each element in the DOM.

7.screenrender.rs-This file contains the commands to display the actual elements on the screen using gfx library.

8.screencommanddisplay-Convert the layout tree to display commands so that it can be rendered on the screen

9./websrc-Contains the template parsable html and css code

10./bin -Contains the actual code which can execute after running .

## There is an issue in CSS Parser. If someone is intrested you can reach me out I am ready to explain the complete problem with cause as I need help in fixing it.


