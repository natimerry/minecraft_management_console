l = eval(input())

new = []

for i in l:
    if i % 2 == 0:
        new.append(i + 10)
    else:
        new.append(i*5)

print(new)