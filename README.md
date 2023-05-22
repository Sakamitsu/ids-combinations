## Description

A script for generating all possible combinations that a Chinese character can consist of. 

The IDS (Ideographic Description Sequence) is used to represent the composition.

This script can be useful in creating Kinship diagrams.

For example:

<img src="https://i.redd.it/ns502i22nv041.jpg" alt="img" width="300" height="300">

## Usage
The script reads a json file of the following form (the order of the characters doesn't matter):

```json
[
    {"id": "一", "children" : null},
    {"id": "丨", "children" : null},
    {"id": "丿", "children" : null},
    {"id": "乀", "children" : null},
    {"id": "十", "children" : ["一","丨"]},
    {"id": "八", "children" : ["丿","乀"]},
    {"id": "木", "children" : ["十","八"]}
]
```

And creates a json file in the `src/files/output.json` folder.
Example of output for `木`:

```json
 {
    "id": "木",
    "children": [
      "十",
      "八"
    ],
    "result": [
      "十八",
      "十丿乀",
      "一丨八",
      "一丨丿乀"
    ]
  }
```

## Installation
Make sure that you install rust. Then clone this repository and run this command in project directory:

```bash
cargo run --release  
```

## License

MIT