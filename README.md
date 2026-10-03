# Chess Project Beta

## Introdution
Enrique Sanchez! Make sure to read through this!
# For programmers:

## How to install the crates:
```
git clone https://github.com/AliPhilip05/ChessProject.git

cd ChessProject

git remote -v

git pull origin main
````

```
# make a branch to work on a function like this
# git checkout -b [function] | where -b means branch meaning you are making a branch in the code
# for example menu 
git checkout -b menu

# after you finish writing your code, commit it to the branch and then push that branch.

git add . #add each file you worked on
#example add .libs.rs

#make the commit clear so that I know what to code review
git commit -m "Add terminal menu"

git push -u origin menu
```

```
#when the code is good to go or if you think branch is good to run commit it with
git merge menu 
```

```
# Run the main to see the start of the code. Do this in terminal
cd ChessProject
Cargo run
rustc main.rs
/main # or .\main on Windows

# play around with this. figure out how to run and update the function with just cargo.

```
