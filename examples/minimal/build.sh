
arm-none-eabi-as -mthumb -mcpu=cortex-m4 -o main.o main.s

objcopy -O ihex main.o main.hex


