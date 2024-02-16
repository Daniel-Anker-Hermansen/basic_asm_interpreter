# Basic assembly interpreter

Interpreter for the basic assembly language invented by [Peyman Afshani](https://pure.au.dk/portal/da/persons/peyman%40cs.au.dk), used in the course, Computer architecture, network and operating systems.

## Usage
First argument is the source file. The file must follow the specification with one instruction per line. Comments can be written after '//' or ';'. Empty lines are ignored. At the end the state of the registers and zero flag are dumped. Additionally, it supports the instructions `debug` which dumps the registers and the zero flag and waits for the user to press enter to continue.
