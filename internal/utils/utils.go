package utils

import (
	"errors"
	"fmt"
	"io"
	"net/http"
	"os"

	"github.com/logrusorgru/aurora/v4"
)

const YEAR = 24

type Day interface {
	Solve() (int, int, error)
}

func GetInput(day int) (string, error) {
	if day == 0 {
		return "", errors.New("Please input a day.")
	}

	filePath := fmt.Sprintf("input/%d/%02d.txt", YEAR, day)

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

func DownloadInput(filePath string, day int) (string, error) {
	fmt.Println(aurora.Green(fmt.Sprintf("Downloading input for day %d", day)))

	cookie := "session=" + os.Getenv("SESSION")
	url := fmt.Sprintf("https://adventofcode.com/20%d/day/%d/input", YEAR, day)
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
