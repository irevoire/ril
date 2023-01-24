set logscale x 2
set yrange[0:200000000]
set xrange[1:1024]

plot 'bench.dat' using ($1):($2) title "sqlite" with lines lt 4 lw 2 lc rgb "blue", \
	 'bench.dat' using ($1):($2 + $3) title "stddev" with lines lc rgb "red", \
	 'bench.dat' using ($1):($2 - $3) title "stddev" with lines lc rgb "red", \
	 'bench.dat' using ($1):($4) title "custom" with lines lt 4 lw 2 lc rgb "green", \
	 'bench.dat' using ($1):($4 + $5) title "stddev" with lines lc rgb "red", \
	 'bench.dat' using ($1):($4 - $5) title "stddev" with lines lc rgb "red"

# wait until <enter> is typed in the terminal
pause -1