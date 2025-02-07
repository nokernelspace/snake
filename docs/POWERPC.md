# PowerPC ASM
To utilize the full potential of the Wii's hardware you must write assembly for the PowerPC Specification


```
Sources: https://wiibrew.org/wiki/Assembler_Tutorial
```

# Wii/Gamecube PowerPC Assembly
PowerPC is an architecture developered by IBM that supports parallel-processing

It is a Reduced Instruction Set Computing processor architecture **(RISC)**
PowerPC is an acronym which stands for  ...
Performance Optimization With Enhanced RISC Performance Chip
It was released in 1993 and is supposed to be a 64-bit specification with a 32-bit subset
_However Almost all PowerPC architectures are 32-bit now and tend to feature a 64-bit data bus due to limitations in lithography techniques at the time_
It operates similar to modern GPUs as a superscalar microprocessor which has seperate excecution units.
A PowerPC CPU has at least integer unit, a floating point unit, and a branching unit
This units all execute in parallel

The following is true only for Wii/Gamecube (Hollywood/Broadway) CPUs

# Le'Puter Terminology
```
Byte - 8 bits
Halfword - 16 bits
Word - 32 bits
String - Characters
```

# Variables
```
bytevar: .byte 0 #length of one byte - init zero
shortvar: .short 0 #length of two byte - init zero
wordvar: .long 0 #length of four byte - init zero
fivebytevar: .byte 11,12,13,14,15 #an array of five variables of one byte each
endof_fivebytevar: #specifies the address immediately following the array
stringvar: .string "Hello\n" #string variable - init to "Hello" plus newline
.size stringvarlen, .-stringvar #length of stringvar

```

# Constants
```
.set var,0
```

# `dst,src` or `src,dst`
`gas` uses `<dst>, <src>` for PowerPC


# Registers
- 32 Integer        Registers Named r0-31
- 32 Floating-Point Registers Named f0-31
- PC                Program Counter
- CR                Conditional           Register (eight (0-7) 4 bit fields holding the result of a compare instruction)
- CTR               Counter               Register
- LR                Link                  Register
- XER               Fixed-Pointer         Register (Used for Exceptions)    Stores temp flags from arithmetic and other stuff
- FPSCR Floating-Pointer Status & Control Register
- 8 Instruction Block Address Translation Registers Named IBAT0-7
    - Used in translating Virtual Addresses to Physical Addresses for fetched instructions
- 8 Data Block Address Translation        Registers Named DBAT0-7
    - Used in translating Virtual Addresses to Physical Addresses for fetched data in RAM
- TB Time Base Register
- MSR Machine State Register         (Vewy Powaful Flag Register)
    - Interrupt Enable/Disable       (Whether the CPU responds to interrupts)
    - Privilege Level                (Whether the processor is running in kernelmode or usermode)
    - Floating-Point Enable/Disable  (Whether the CPU can use floating point instructions)
    - Branch Prediction              (Temp variables for branch prediction calculation)
    - Other OEM specific stuff
- SRR0 Used for Saving and Restoring the PC  during exceptions (interrupts/errors)
- SRR1 Used for Saving and Restoring the MSR during exceptions (interrupts/errors)
- An `Exception` register
- An `Interrupt Mask` Register
- An `Interrupt Cause` Register
- Data Storage Interupt Status Register (DSISR) Used for when an interrupt occurs while RAM is being accessed
- Data Address Register (DAR) When an interrupt occurs it stores the virtual address that caused the fault
- Hash Mask Register (Used for indexing the virtual -> physical hashmap)
- Segement Registers (Used for mapping blocks in RAM from virtual -> physical) Named s0-15
- Graphics Quanternion/Qunatization?? Register GQR0-7
- Hardware Implementation Dependent Registers (HID0-4)
    - Basically whatever the fuck. Probably used for ripping Nintendo's propritary disks

# Instructions
## Integer
## Floating
## Load & Store
## Branch & Flow Control
## Other Misc. Bullshit
