# MessageBox CLI

#### Shows a w32 MessageBox with given args

-------

## Pre-requistes
This app requires windows to run!

## How to run this app

1. Make sure [Rust](https://www.rust-lang.org/tools/install) is installed and configured.
2. Clone the repo
3. Build the project by running ```cargo build``` on your terminal
4. To run the project type ```cargo run "Hello World" "Msgbox App" 1``` and press Enter

## Arguments

1. MSG: MessageBox Text
   - No need of quotations if its a one word
   - Otherwise wrap the text with quotations
2. CAPTION: MessageBox window title
   - No need of quotations if its a one word
   - Otherwise wrap the text with quotations
3. ICON: Icon type of the Messagebox
   - 1 => Information Icon
   - 2 => Stop/Error Icon
   - 3 => Question Icon
   - 4 => Warning Icon
   
   Entering anything other than above will display "Info" icon
   
## Additional Reading

Dont forget to check out [Pachi's winapi examples](https://github.com/pachi/rust_winapi_examples). It is well explained and helped me to understand the basics really fast!

