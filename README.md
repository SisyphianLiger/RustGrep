# RustGrep

# Background

I am currently reading and following along with the Rust Manual. 
After chapter 12 I decided to publish and update this CLI tool 
which is a naive version of GREP.

# Setup
The code can be ran with 
```
cargo run -- <GREP TARGET> <FILENAME>
```

For Case Insentive searches in your terminal set up an environment variable.
The Environment Variable is called IGNORE_CASE and is a part of the Config 
struct.

You can run the insensitive command as followed
```
$Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

and change the environment variable for sensitive searching



