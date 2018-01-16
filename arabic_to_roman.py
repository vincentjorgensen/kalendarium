#!/usr/bin/env python3

import os, sys

def a_to_r(num):
  if num//1000000 > 0:
    l=num//1000000
    return ''+('M'+'\u0304'+'?'+'\u0304'+a_to_r((num-4000000)%1000000) if num >= 4000000 else l*('M'+'\u0304')+a_to_r(num%1000000))
  elif num//500000 > 0:
    l=num//500000
    return ''+('C'+'\u0304'+'M'+'\u0304'+a_to_r((num-900000)%500000) if num >= 900000 else l*('D'+'\u0304')+a_to_r(num%500000))
  elif num//100000 > 0:
    l=num//100000
    return ''+('C'+'\u0304'+'D'+'\u0304'+a_to_r((num-400000)%100000) if num >= 400000 else l*('C'+'\u0304')+a_to_r(num%100000))
  elif num//50000 > 0:
    l=num//50000
    return ''+('X'+'\u0304'+'C'+'\u0304'+a_to_r((num-90000)%50000) if num >= 90000 else l*('L'+'\u0304')+a_to_r(num%50000))
  elif num//10000 > 0:
    l=num//10000
    return ''+('X'+'\u0304'+'L'+'\u0304'+a_to_r((num-40000)%10000) if num >= 40000 else l*('X'+'\u0304')+a_to_r(num%10000))
  elif num//5000 > 0:
    l=num//5000
    return ''+('I'+'\u0304'+'X'+'\u0304'+a_to_r((num-9000)%5000) if num >= 9000 else l*('V'+'\u0304')+a_to_r(num%5000))
  elif num//1000 > 0:
    l=num//1000
    return ''+('I'+'\u0304'+'V'+'\u0304' + a_to_r((num-4000)%1000) if num >= 4000 else l*'M'+a_to_r(num%1000))
  elif num//500 > 0:
    l=num//500
    return ''+('CM' + a_to_r((num-900)%500) if num >= 900 else l*'D'+a_to_r(num%500))
  elif num//100 > 0:
    l=num//100
    return ''+('CD' + a_to_r((num-400)%100) if num >= 400 else l*'C'+a_to_r(num%100))
  elif num//50 > 0:
    l=num//50  
    return ''+('XC'+a_to_r((num-90)%50) if num >= 90 else l*'L'+a_to_r(num%50))
  elif num//10 > 0:
    l=num//10  
    return ''+('XL'+a_to_r((num-40)%10) if num >= 40 else l*'X'+a_to_r(num%10))
  elif num//5 > 0:
    l=num//5
    return ''+('IX'+a_to_r((num-9)%5) if num >= 9 else l*'V'+a_to_r(num%5))
  elif num//1 > 0:
    l=num//1
    return ''+('IV'+a_to_r((num-4)%1) if num >= 4 else l*'I'+a_to_r(num%1))
  else:
    return ''


a2r = [[5000000, '?'+'\u0304'],[1000000, 'M'+'\u0304'],[500000, 'D'+'\u0304'],
       [100000, 'C'+'\u0304'], [50000, 'L'+'\u0304'], [10000, 'X'+'\u0304'],
       [5000, 'V'+'\u0304'], [1000, 'M', 'I'+'\u0304'], [500, 'D'], [100, 'C'],
       [50, 'L'], [10, 'X'], [5, 'V'], [1, 'I']]
def a_to_r2(num):
  if num != 0:
    for index, base in enumerate(a2r):
      if num//base[0] > 0:
        l=num//base[0]
        if '5' in str(base[0]):
          if num >= a2r[index-1][0] - a2r[index+1][0]:
            return (a2r[index+1][2] if num >= 4000 and num < 10000 else a2r[index+1][1]) + a2r[index-1][1] + a_to_r2((num-(a2r[index-1][0] - a2r[index+1][0]))%base[0])
          else:
            return l*base[1]+a_to_r2((num-a2r[index-1][0])%base[0])
        else:
          if num >= a2r[index-1][0] - base[0]:
            return (base[2] if num >= 4000 and num < 10000 else base[1]) + a2r[index-1][1]         + a_to_r2((num-(a2r[index-1][0] - base[0]))%base[0])
          else:
            return l*base[1]+a_to_r2((num-a2r[index-1][0])%base[0])
  return ''

def a_to_r3(num):
  if num != 0:
    for index, base in enumerate(a2r):
      if num//base[0] > 0:
        l=num//base[0]
        offset=(1 if '5' in str(base[0]) else 0)
        if num >= a2r[index-1][0] - a2r[index+offset][0]:
          return (a2r[index+offset][2] if 4000 <= num < 10000 else a2r[index+offset][1]) + a2r[index-1][1] + a_to_r3((num-(a2r[index-1][0] - a2r[index+offset][0]))%base[0])
        else:
          return l*base[1]+a_to_r3((num-a2r[index-1][0])%base[0])
  return ''

print(a_to_r(int(sys.argv[1])))
print(a_to_r2(int(sys.argv[1])))
print(a_to_r3(int(sys.argv[1])))

#End
