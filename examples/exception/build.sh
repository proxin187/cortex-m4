
arm-none-eabi-as -mthumb -mcpu=cortex-m4 -o main.o main.s

arm-none-eabi-ld main.o -o main.elf -Ttext=0x10

objcopy -O ihex main.elf main.hex


