notes.md

# Notes

## Intro 

(von neumann model)
fetch instruction (from memory)
decode instruction
execute instruction

How does the OS virtualize resources?

"Each process accesses its own private virtual address space (sometimes just called its address space), which the OS somehow maps onto the physical memory of the machine."

Brinch Hansen history of OS

"In moving beyond being a simple library of commonly-used services, operating systems took on a more central role in managing machines. One important aspect of this was the realization that code run on behalf of the OS was special; it had control of devices and thus should be treated differently than normal application code."

"The key difference between a system call and a procedure call is that a system call transfers control (i.e., jumps) into the OS while simultaneously raising the hardware privilege level."

virtualization (memory, virtual machine, sys calls)
concurrency (threads, processes, memory)
persistence (devices, io)

### questions

- why is the memory address not the same in rust as C?
- why cant i ctrl-C when I run multiple programs?