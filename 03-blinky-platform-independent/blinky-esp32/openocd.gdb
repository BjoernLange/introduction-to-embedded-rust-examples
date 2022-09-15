target extended-remote :3333
monitor reset halt
break blinky::blinky_main
continue