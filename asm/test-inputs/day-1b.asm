.data

buffer: .asciiz "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000"

.text

li $t0, 0  # Character index
li $t2, 0  # Running total
li $t3, 0  # Current number

li $t4, 0  # Largest
li $t5, 0  # 2nd Largest
li $t6, 0  # 3rd Largest

main:
	lb $t1, buffer($t0)
	
	beqz $t1, next_num
	beq $t1, '\n', next_num
	
	subi $t1, $t1, 48  # Convert char to num by digit
	
	# Multiplies running total by 10 and adds new digit
	add $t9, $t3, $t3  # T = 2 * cur
	sll $t3, $t3, 3    # cur *= 8
	add $t3, $t3, $t9  # cur += T
	add $t3, $t3, $t1  # cur += new_digit
	
	addi $t0, $t0, 1
	j main
	
next_num:
	add $t2, $t2, $t3
	li $t3, 0
	
	beqz $t1, finished_group
	
	addi $t0, $t0, 1
	lb $t1, buffer($t0)
	beq $t1, '\n', finished_group
	
	j main
	
finished_group:
	addi $t0, $t0, 1
	move $t9, $t2
	li $t2, 0

    bge $t6, $t9, end_group  # If smaller than 3rd, skip

    move $t6, $t9

    bge $t5, $t6, end_group  # If smaller than 2nd

    move $t6, $t5
    move $t5, $t9

    bge $t4, $t5, end_group  # If smaller than largest

    move $t5, $t4
    move $t4, $t9

    j main

end_group:
	beqz $t1, end
	j main

end:
	li $v0, 1
    add $t5, $t5, $t6
	add $a0, $t4, $t5
	syscall

	li $v0, 10
	syscall
