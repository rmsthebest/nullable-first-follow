nff.rs
======

A program that calculates the nullable, first, and follow sets from a specified grammar

# Installation and Usage

## Github
```
git clone https://github.com/rmsthebest/nullable-first-follow
cd nullable-first-follow
cargo run --release <path to grammar file>
```

## Cargo
```
cargo install nff
nff <path to grammar file>
```

# Grammar
nff only supports one format of grammar  
Nonterminals are one uppercase character  
Terminals are one lowercase character  
Null is 0
```
A -> B a
A -> 0
B -> A b
```
