package main

import (
	"fmt"
	"strconv"
	"strings"
)

var (
	s_split []string
	subset []string
)

func main() {
	var s string
	fmt.Scan(&s)

	s_split = strings.Split(s, "")
	task(s_split[0], 0)

	var ans int
	for _, v := range subset {
		vv := strings.Split(v, "+")
		for _, vvv := range vv {
			a, _ := strconv.Atoi(vvv)
			ans += a
		}
	}
	fmt.Println(ans)
}

func task(s string, i int) {
	v1 := s + s_split[i+1]
	v2 := s + "+" + s_split[i+1]
	if i == len(s_split)-2 {
		subset = append(subset, v1)
		subset = append(subset, v2)
		return
	}
	task(v1, i+1)
	task(v2, i+1)
}
