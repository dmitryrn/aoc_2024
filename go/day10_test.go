package day10

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func TestDay10_1(t *testing.T) {
	result := day10(`
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
    `)
	assert.Equal(t, 2, result)
}

func TestDay10_2(t *testing.T) {
  result := day10(`
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
    `)
	assert.Equal(t, 4, result)
}

func TestDay10_3(t *testing.T) {
  result := day10(`
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
    `)
	assert.Equal(t, 3, result)
}

func TestDay10_4(t *testing.T) {
  result := day10(`
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
    `)
	assert.Equal(t, 36, result)
}
