package main

import (
	"fmt"
)

var (
	n,counter int
)

const three = 1
const five = 2
const seven = 4
const satisfy = 7

func main() {
	fmt.Scan(&n)
	solve(0, 0)
	fmt.Println(counter)
}

func solve(current, flags int) {
	if current > n {
		return
	}
	if flags == satisfy {
		counter++
	}
	solve(current*10 + 3, flags | three)
	solve(current*10 + 5, flags | five)
	solve(current*10 + 7, flags | seven)
}
