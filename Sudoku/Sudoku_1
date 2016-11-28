import numpy as np
#519
import time

game_str_1 = '.7....49...3.1........6.......98.7..2........1...........7...3..4.8.....6.......1'
game_str_2 = '6.579.3.14.218...99.7.32.8.8.43251.7.9.861.425..9.78...796.34..15.47.236..6..8975'
#2068
game_str_6 = '.4....3..2..5.............1.78.........4...2...1...6..3.....27.....81.......6....'
game_str_3 = '8719.56..2..31.754..3.7619.392.5.48.7.8..29...1.....23.8712.365.36.84279.2576..41'
game_str_4 = '8..2..53454..7.219.1.5.387.75.9.2.4.4.1.8569.....61725.3462895.982.57.6.6...943.2'
#35702
game_str_7 = '71...2.....3...5......8...46.....8.5......6....27........3...2.9.........8.......'
# 13 it 0.1225s
game_str_8 = '.........71..6..93.46...71....3.2...1.3...9.8...9.7....52...14.97..4..52.........'

fd=np.matrix(np.ndarray((9,9), buffer=np.array(list(game_str_1)), dtype=np.dtype('<U1')))

notInField = set(['1','2','3','4','5','6','7','8','9'])

def get_row(field, row):
    return notInField - set(field[row:row+1,0:9].getA1())

def get_colum(field, col):
    return notInField - set(field[0:9,col:col+1].getA1())

def get_field(field,x,y):
    return notInField - set(field[y*3:(y+1)*3,x*3:(x+1)*3].getA1())

def fill_missing_step(field):
    count = 0
    for ro in range(0,9):
        getrow = get_row(field, ro)
        for co in range(0,9):
            if (field[ro, co] == '.'):
                notin = getrow & get_colum(field, co) & get_field(field, co // 3, ro // 3)
                if len(notin) == 1:
                    field[ro,co] = list(notin)[0]
                elif len(notin) == 0:
                    return -1
                else:
                    count = count + 1
    return count


def find_best(field):
    for value in range(2,9):
        for ro in range(0,9):
            for co in range(0,9):
                if (field[ro, co] == '.'):
                    notin = get_row(field, ro) & get_colum(field, co) & get_field(field, co // 3, ro // 3)
                    if len(notin) == value:
                        return ro,co,notin
    return


def fill_missing(field):
    global itnum
    start = 0
    fr = 999
    while True:
        start = fr
        fr = fill_missing_step(field)
        if start == fr:
            break
        if (fr==-1):
            return -1
        if (fr==0):
            print ('Finish')
            print (field)
            print ('Solved after {0} iterations'.format(itnum))
            return 0
    return fr


def solve(field):
    global itnum
    retvalue = fill_missing(field)
    if (retvalue > 0):
        ro,co,num = find_best(field)
        numbers = list(num)
        numbers.sort()
        for trynumber in numbers:
            newfield = field.copy()
            newfield[ro,co] = trynumber
            itnum = itnum + 1
            if solve(newfield) == 0:
                return 0
    return retvalue



print ('Start :')
itnum = 1
print (fd)
start = time.process_time()
solve(fd)
stop = time.process_time()
print ('Zeit : {0} Sekunden'.format(stop-start))
