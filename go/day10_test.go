package day10

import (
	"testing"
	"github.com/stretchr/testify/assert"
)

func TestDay10_1(t *testing.T) {
	score, _ := day10(`
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
    `)
	assert.Equal(t, 2, score)
}

func TestDay10_2(t *testing.T) {
  score, _ := day10(`
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
    `)
	assert.Equal(t, 4, score)
}

func TestDay10_3(t *testing.T) {
  score, _ := day10(`
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
    `)
	assert.Equal(t, 3, score)
}

func TestDay10_4(t *testing.T) {
  score, _ := day10(`
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
    `)
	assert.Equal(t, 36, score)
}


func TestDay10_pt2_1(t *testing.T) {
  _, rating := day10(`
.....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....
    `)
	assert.Equal(t, 3, rating)
}


func TestDay10_pt2_2(t *testing.T) {
  _, rating := day10(`
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
    `)
	assert.Equal(t, 13, rating)
}


func TestDay10_pt2_3(t *testing.T) {
  _, rating := day10(`
012345
123456
234567
345678
4.6789
56789.
    `)
	assert.Equal(t, 227, rating)
}

func TestDay10_pt2_4(t *testing.T) {
  _, rating := day10(`
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
    `)
	assert.Equal(t, 81, rating)
}
