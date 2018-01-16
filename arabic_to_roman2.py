#!/usr/bin/env python3

import os, sys

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

print(a_to_r2(int(sys.argv[1])))

#End
