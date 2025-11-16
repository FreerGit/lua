x = 1
y = 2
z = x + y
a = x * 42
b = y / 2
c = a - b
d = 10 // 3
e = 2 ^ 10
f = # "hello"

g = x & y
h = x | y
i = x ~ y
j = x << 2
k = y >> 1

if x == y then x = y end
if a ~= b then a = b end
if c <= d then c = d end
if e >= f then e = f end
if x < y then x = y end
if a > b then a = b end

print("Hello")
print("World")
print("Lua")
print("Test")

my_table = {1,2,3,4,5}
other_table = {a=1, b=2, c=3}

function add(x, y)
    return x + y
end

function sub(x, y)
    return x - y
end

for i=1,100 do
    x = i
    y = i*2
    z = add(x, y)
    w = sub(y, x)
end

x = 123
y = 456
z = x + y
a = x * 2
b = y / 3
c = a - b
d = 100 // 7
e = 3 ^ 4
f = # "world"
u = 5.5

g = x & y
h = x | y
i = x ~ y
j = x << 1
k = y >> 2

if x == y then x = y end
if a ~= b then a = b end
if c <= d then c = d end
if e >= f then e = f end
if x < y then x = y end
if a > b then a = b end

print("Repeat test")
print("Lexer test")
print("Large file")
print("Performance test")
