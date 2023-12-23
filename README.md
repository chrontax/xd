Just a simple hexdump program I made because xxd doesn't support naming the output consts
usage: `xd <file> <name>`
outputs to stdout ready to be pasted into a c file
const with data is named <name>
const with data length is named <name>_len
