.data

input: .asciiz "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"

.text

li $t0, 0  # Character index
li $t2, 0  # Current max
li $t3, 0  # Running total
li $t4, 0  # Current number

main:
	lb $t1, input($t0)
	
	beqz $t1, next_num
	beq $t1, '\n', next_num
	
	subi $t1, $t1, 48  # Convert char to num by digit
	
	# Multiplies running total by 10 and adds new digit
	add $t9, $t4, $t4  # T = 2 * cur
	sll $t4, $t4, 3    # cur *= 8
	add $t4, $t4, $t9  # cur += T
	add $t4, $t4, $t1  # cur += new_digit
	
	addi $t0, $t0, 1
	j main
	
next_num:
	add $t3, $t3, $t4
	li $t4, 0
	
	beqz $t1, end
	
	addi $t0, $t0, 1
	lb $t1, input($t0)
	beq $t1, '\n', finished_group
	
	j main
	
finished_group:
	addi $t0, $t0, 1
	move $t9, $t3
	li $t3, 0
	bgt $t2, $t9, main
	move $t2, $t9
	j main

end:
	li $v0, 1
	move $a0, $t2
	syscall

	li $v0, 10
	syscall
