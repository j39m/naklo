#! /usr/bin/env python

from subprocess import call 

fp_to_files = open("files")
fp_to_titles = open("titles")

files = []
titles = []

for line in fp_to_files: 
  if (line[-1] == "\n"): 
    line = line[:-1]
  files.append(line)

for line in fp_to_titles: 
  if (line[-1] == "\n"): 
    line = line[:-1]
  titles.append(line)

# GO
if len(files) != len(titles): 
  print("unequal lengths!")
  exit

i = 0
while ( i < len(files)): 
  call(["metaflac", "--set-tag=title="+titles[i], files[i]])
  print("Tagged " + files[i])
  i += 1
