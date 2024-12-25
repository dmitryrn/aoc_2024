package day10

import (
	"fmt"
	"strings"
)

func day10(raw string) int {
	m := parseMatrix(raw)

	globalScore := 0
	for y := range m {
		for x := range m[y] {
			if m[y][x] == '0' {
				score := 0
        visited9 := make(map[[2]int]struct{})
				travel(m, x, y, &score, visited9)
				fmt.Printf("Score for '0' at (%d, %d): %d\n", x, y, score)
				globalScore += score
			}
		}
	}

	return globalScore
}

func travel(m [][]byte, x, y int, score *int, visited9 map[[2]int]struct{}) {
	current := m[y][x]

	for _, pair := range [][]int{{x - 1, y}, {x + 1, y}, {x, y + 1}, {x, y - 1}} {
		nextX := pair[0]
		nextY := pair[1]

		if !isInBounds(m, nextX, nextY) {
			continue
		}

		next := m[nextY][nextX]
		if next == current+1 {
      // println(nextX, nextY)
      // debugMatrix(m, nextX, nextY)
      // println()
			if next == '9' {
        // println("inc")
        if _, ok := visited9[[2]int{nextX, nextY}]; !ok {
          *score++
          visited9[[2]int{nextX, nextY}] = struct{}{}
        }
			}
			travel(m, nextX, nextY, score, visited9)
		}
	}
}

func isInBounds(m [][]byte, x, y int) bool {
	return x >= 0 && y >= 0 && x < len(m[0]) && y < len(m)
}

func printMatrix(matrix [][]byte) {
	for _, row := range matrix {
		fmt.Println(string(row))
	}
}

func copyMatrix(src [][]byte) [][]byte {
	dst := make([][]byte, len(src))
	for i := range src {
		dst[i] = make([]byte, len(src[i]))
		copy(dst[i], src[i])
	}
	return dst
}

func debugMatrix(mOrig [][]byte, x, y int) {
	m := copyMatrix(mOrig)
	m[y][x] = 'X'
	printMatrix(m)
}

func parseMatrix(raw string) [][]byte {
	lines := strings.Split(strings.TrimSpace(raw), "\n")
	matrix := make([][]byte, len(lines))
	for i, line := range lines {
		if len(line) == 0 {
			continue
		}
		matrix[i] = []byte(line)
	}
	return matrix
}
