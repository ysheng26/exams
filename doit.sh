#!/bin/bash

cargo run > questions.txt
cp questions.txt answers.txt

sed -i '' '/Question/d' ./answers.txt
sed -i '' '/Answer/d' ./questions.txt


