### Continue here: https://youtu.be/doFowk4xj7Q?t=2248


### get oa.txt/ma.txt content via source viewer from:

https://www.nytimes.com/games/wordle/index.html

### download ngrams from

http://storage.googleapis.com/books/ngrams/books/20200217/eng/eng-1-ngrams_exports.html

using get_ngrams.py

### extract data

```console
rg -Iz "^[a-zA-Z]{5}\_[A-Z]+\t" 1-\*of-00024.gz > 5-letters.txt

awk -F'\t' '{print $1"\t"$NF}' 5-letters.txt | sed 's/_/,/' | awk -F, '{print $1 " "$(NF-1)}' > 5-letters-occur.txt

tr A-Z a-z < 5-letters-occur.txt | sort > 5-letters-lc-sorted.txt

awk 'BEGIN {w="";v=0} {if (!w) w=$1; else if ($1==w) v=v+$2; else {print w" "v; w=$1; v=$2;}} END {print w" " v}' < 5-letters-lc-sorted.txt > 5-letters-lc-sorted-combined.txt

cat oa.txt ma.txt > dictionary.txt

cat dictionary.txt | sort > dictionary-sorted.txt

join -a 1 dictionary-sorted.txt 5-letters-lc-sorted-combined.txt > dictionary-with-counts.txt

```

### Other interesting commands

```
wc -l 5-letters.txt
```

