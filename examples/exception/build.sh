
arm-none-eabi-as -mthumb -mcpu=cortex-m4 -o main.o main.s

arm-none-eabi-ld main.o -T link.ld -o main.elf


