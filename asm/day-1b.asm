.data

input_file: .asciiz "/Users/gregormaclaine/Projects/AoC-2022/input/day-1.txt"
buffer: .space 12000  # 12KB

.text

read_file:
	li $v0, 13           # Load the system call code for open file
	la $a0, input_file   # Load the address of the input file name
	li $a1, 0            # Flag for reading
	li $a2, 0                                       # Mode is ignored
	syscall

	move $s0, $v0                                   # Save the file descriptor returned by syscall

	li $v0, 14                                      # Load the system call code for reading from file
	move $a0, $s0                                   # Load the file descriptor to read
	la $a1, buffer                                  # Load the address of the buffer to write into
	li $a2, 12000                                    # Read the entire file into the buffer
	syscall


	li $v0, 16                                      # Load the system call for close file
	move $a0, $s0                                   # Load the file descriptor to close
	syscall

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
