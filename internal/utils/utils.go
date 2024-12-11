package utils

import (
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"
	"testing"

	"github.com/joho/godotenv"
	"github.com/logrusorgru/aurora/v4"
	"github.com/stretchr/testify/assert"
)

const YEAR = 24

type Day interface {
	Part1(input string) (int, error)
	Part2(input string) (int, error)
}

type Part func(input string) (int, error)

func GetInput(day int) (string, error) {
	if day == 0 {
		return "", errors.New("Please input a day.")
	}

	filePath := fmt.Sprintf("../../input/%d/%02d.txt", YEAR, day)

	if _, err := os.Stat(filePath); os.IsNotExist(err) {
		input, err := DownloadInput(filePath, day)
		if err != nil {
			return "", err
		}
		return input, nil
	} else if err != nil {
		return "", err
	}

	input, err := os.ReadFile(filePath)
	if err != nil {
		return "", err
	}

	return string(input), nil
}

func TestDay(t *testing.T, d Day, d1, d2 string, e1, e2, r1, r2, dayN int) {
	testPart(t, "Part 1: Test Case", d.Part1, d1, e1)
	testActual(t, "Part 1: Real", d.Part1, r1, dayN)
	testPart(t, "Part 2: Test Case", d.Part2, d2, e2)
	testActual(t, "Part 2: Real", d.Part2, r2, dayN)
}

func testActual(t *testing.T, partName string, p Part, expected int, dayN int) {
	input, err := GetInput(dayN)
	if err != nil {
		t.Fatal(aurora.Red("Failed "+partName), err)
	}
	testPart(t, partName, p, input, expected)
}

func testPart(t *testing.T, partName string, p Part, input string, expected int) {
	t.Run(partName, func(t *testing.T) {
		actual, err := p(input)
		if err != nil {
			t.Fatal(aurora.Red("Failed "+partName), err)
		}
		if !assert.Equal(t, expected, actual) {
			t.Fatal(aurora.Red("Failed "+partName), err)
		}
	})
}

func DownloadInput(filePath string, day int) (string, error) {
	_ = godotenv.Load()
	cookie := "session=" + os.Getenv("SESSION")
	url := fmt.Sprintf("https://adventofcode.com/20%d/day/%d/input", YEAR, day)
	fmt.Println(aurora.Green(fmt.Sprintf("Downloading input for day %d from %s", day, url)))

	req, err := http.NewRequest("GET", url, nil)
	if err != nil {
		return "", err
	}

	req.Header.Set("Cookie", cookie)
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		return "", err
	}
	defer res.Body.Close()

	body, err := io.ReadAll(res.Body)
	if err != nil {
		return "", err
	}

	if err := writeBytes(filePath, body); err != nil {
		return "", err
	}

	return string(body), nil
}

func writeBytes(filePath string, b []byte) error {
	file, err := os.Create(filePath)
	if err != nil {
		return err
	}
	defer file.Close()
	_, err = file.Write(b)
	if err != nil {
		return err
	}

	return nil
}
