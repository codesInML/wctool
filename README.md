### WC-TOOL

To build run the command below

```bash
cargo build
```

Then get the details of a file like below

```bash
./target/debug/wctool filename.txt
```

or like this 
```bash
cat filename.txt | ./target/debug/wctool
```

You can also pass optional arguments to get a particular detail.
Possible arguments includes
  
- -c (gets the number of bytes of a file)
- -l (gets the number of lines in a file)
- -w (gets the number of words in a file)
- -m (gets the number of characters in a file)

This project was inspired by the standard wc cli tool.