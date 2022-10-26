
target ext :3333
monitor tpiu config internal itm.txt uart off 8000000
monitor itm port 0 on
load

break main
continue
