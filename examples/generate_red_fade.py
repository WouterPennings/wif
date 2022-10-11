size = 255
str = f"{size} {size} "

count = 0
step = 0
for i in range(size+1):
    for _ in range(int(size/2)+1):
        #str += f"0000{hex(count)[2:]} "
        str += f"00ff00 "
        str += f"ff0000 "

    # if step % 3 == 0:   
    #     count += 1
    # step += 1
    count += 1

f = open("red_fade.wif", "w")
f.write(str)
f.close()
