_estack = 0x20020000;

MEMORY
{
    FLASH (rx)  : ORIGIN = 0x0, LENGTH = 65540
    RAM (xrw) : ORIGIN = 0x20000000, LENGTH = 16380
}


SECTIONS
{
    .text : {
        *(.text)
    }>FLASH

    _flash_dstart = .;

    .data :  {
        _dstart = .;
        *(.data)
        _dend = .; 
    }>RAM AT> FLASH  /* Load into FLASH, but live in RAM */
}

