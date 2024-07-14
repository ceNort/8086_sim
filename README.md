# What This?
This is a personal project to simulate an 8086 CPU in Rust based on the specifications in the manual [here](https://edge.edx.org/c4x/BITSPilani/EEE231/asset/8086_family_Users_Manual_1_.pdf).  This is a learning project for me and is companion work to my progress through Casey Muratori's [Performance-Aware Programming Series](https://www.computerenhance.com/).  It is a work-in-progress.

## Running
`cargo run -- [bindump] [file] {filepath}`  
ARGS:  
filepath = binary to disassemble *(required)*  

FLAGS:  
bindump = writes hex and bytes to file in output/ directory for easy reading *(optional)*  
file = output disassembled ASM to file in output/ directory *(optional)*  