set max 0x7FFFFFFF
set r [expr ($max*rand() + 1)]
set f [open rand.txt w]
puts $f [format "%s" [expr int($r)] ]
close $f
