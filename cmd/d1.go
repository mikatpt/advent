package main

import (
	"strconv"
	"strings"
)

func d1_1(input string) int {
	res := 0

	lines := strings.Split(input, "\n")
	first, err := strconv.ParseInt(lines[0], 10, 32)

	if err != nil {
		panic(err)
	}

    lines = lines[1:]

	for _, line := range lines {
        if line == "" {
            continue
        }
		second, err := strconv.ParseInt(line, 10, 32)
		if err != nil {
			panic(err)
		}

		if second > first {
			res += 1
		}

		first = second
	}

	return res
}

func d1_2(input string) int {
    res := 0

    lines := strings.Split(input, "\n")

    integers := []int{}

    for _, line := range lines {
        if line == "" {
            continue
        }

        n, err := strconv.ParseInt(line, 10, 0)
        if err != nil {
            panic(err)
        }
        integers = append(integers, int(n))
    }

    first := 0

    for i := 0; i < 3; i++ {
        first += integers[i]
    }

    for i := 3; i < len(integers); i++ {
        n := integers[i]
        second := first - integers[i-3] + n

        if second > first {
            res++
        }
        first = second
    }

    return res
}
