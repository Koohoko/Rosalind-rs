seq <- readLines("inputs/rna.txt")
# seq <- strsplit(paste0(seq, collapse = ""), "")[[1]]
print(gsub("T", "U", seq))
