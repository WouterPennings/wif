size = 1024
str = f"{size} {size} "

count = 0
for i in range(size):
    for _ in range(int(size)):
        str += f"0000ff "

    count += 1

f = open("plain_field.wif", "w")
f.write(str)
f.close()