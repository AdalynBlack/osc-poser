# OSC Poser
This is a basic program I've written in Rust to familiarize myself with Rust, and allow VRChat OSC trackers to be created and positioned with simple commands

## Usage
1. Start the program with `cargo run`
2. Create or move a tracker with `echo "Name|pX,pY,pZ|rX,rY,rZ" > /tmp/osc-poser`
  - Name is any number 1..8, or "head"
  - pX, pY, and pZ are the X, Y, and Z position of the tracker
  - rX, rY, and rZ are the X, Y, and Z euler angles of the tracker
