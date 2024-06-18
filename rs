a = float(input(" Digite um numero:   "))
b = float(input(" Digite outro numero:   "))

print(''' escolha um dos números que indicara para qual operação quer fazer
[1] multiplicação
[2] divisão
[3] subtração 
[4] soma ''')
c= float(input(" Sua opcão:  "))

if c == 1:
    print('o resultado da operação foi:', a*b)
elif c == 2:
    print('o resultado da operação foi:', a/b)
elif c == 3:
    print('o resultado da operação foi:', a - b)
elif c == 4:
    print('o resultado da operação foi:', a+b)
