seq <- readLines("inputs/dna.txt")
count <- strsplit(paste0(seq, collapse = ""), "")[[1]]
a_count <- 0
c_count <- 0
g_count <- 0
t_count <- 0
for (nt in count){
	if(nt=="A") a_count <- a_count+1
	if(nt=="C") c_count <- c_count+1
	if(nt=="G") g_count <- g_count+1
	if(nt=="T") t_count <- t_count+1
}
print(c(a_count, c_count, g_count, t_count))
