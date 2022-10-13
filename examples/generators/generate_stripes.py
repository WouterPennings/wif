size = 1024
str = f"{size} {size} "

step = 0
for i in range(size+1):
    for _ in range(size+1):
        if step % 2 == 0:
            str += f"000000 "
        else:
            str += f"ffffff "
        step += 1

f = open("checkerboard.wif", "w")
f.write(str)
f.close()
