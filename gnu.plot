set logscale x 2
set yrange[0:200000000]
set xrange[1:1024]

plot 'truc' using ($1):($2) title "sqlite" with lines lt 4 lw 2, \
	 'truc' using ($1):($2 + $3) title "stddev" with lines, \
	 'truc' using ($1):($2 - $3) title "stddev" with lines

# wait until <enter> is typed in the terminal
pause -1