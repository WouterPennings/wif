size = 255
str = f"{size} {size} "

count = 0
for i in range(size):
    for _ in range(int(size/2)):
        str += f"0000{hex(count)[2:]} "

    count += 1

f = open("red_fade.wif", "w")
f.write(str)
f.close()
