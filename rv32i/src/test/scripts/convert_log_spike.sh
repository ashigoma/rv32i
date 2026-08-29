perl -i -ne '
if (/core\s+\d+:\s+\d+\s+(0x[0-9a-f]+)/i) {
    $pc = $1;
    if (/(x\d+)\s+(0x[0-9a-f]+)\s+mem\s+(0x[0-9a-f]+)/i) {
        print "($pc) $1 = [$3]\n";
    } elsif (/mem\s+(0x[0-9a-f]+)(?:\s+(0x[0-9a-f]+))?/i) {
        $val = $2 // "0x0";
        print "($pc) [$1] = $val\n";
    } elsif (/(x\d+)\s+(0x[0-9a-f]+)/i) {
        print "($pc) $1 = $2\n";
    }
}' "$1"