.data

input: .asciiz "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"

.text

end:
	li $v0, 10
	syscall
