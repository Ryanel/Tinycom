# R32k
The R32k is a in-house CPU designed for multiple uses. It is a part of the Tinycom home computer.
To improve performance and cost effectiveness, it uses a Load/Store architecture.

## Basics
The CPU is a 16/32 bit system. 
Registers and Data are 32-bit, while the ALU is 16-bit. 
The address space is limited to 24-bits (16MB), because the external address bus is limited to 24-bits.
Memory is not segmented, it is flatly addressed.

### Registers
All registers are 32-bit.

* 8 data registers (r0 - r7)
* 8 address registers (r8 - r15)
* Stack pointer (sp)
* Program Counter (pc)
* CPU Status Register (fl)

### Pipelining
The CPU contains 5 stages. These are Fetch, Decode, Execute, Memory Access, and Writeback.

## Pipeline in detail
The CPU has instructions (with 2 data registers) allocated for each stage in the pipeline cache. When an instruction advances, these are loaded into the next stages cache. They are discarded once the Writeback stage is completed.

### 