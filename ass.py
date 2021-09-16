f = open("a.out", "wb")
f.write(0b0101_0000_0011_0001.to_bytes(2, byteorder='big')) # ADDI      3  $1
f.write(0b1010_0000_0001_0010.to_bytes(2, byteorder='big')) # MOV      $1  $2
f.write(0b0101_0010_0001_0010.to_bytes(2, byteorder='big')) # SHL       1  $2
f.write(0b0101_0001_0001_0010.to_bytes(2, byteorder='big')) # SUBI      1  $2
f.write(0b0000_0001_0010_0011.to_bytes(2, byteorder='big')) # ADD  $1  $2  $3
f.write(0b1100_1111_0001_0011.to_bytes(2, byteorder='big')) # STO  $15  1  $3
f.write(0b1111_0000_0000_0000.to_bytes(2, byteorder='big')) # HLT
f.write(0b0000_0000_0000_0000.to_bytes(2, byteorder='big')) # blank space
f.close()