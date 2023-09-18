package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"path/filepath"
	"strings"
)

func appendByteAsHex(bs []byte, b byte) []byte {
	if b < 16 {
		bs = append(bs, '0')
	} else {
		h := b >> 4
		if h < 10 {
			bs = append(bs, '0'+h)
		} else {
			bs = append(bs, 'a'+h-10)
		}
	}

	h2 := b & 0x0f
	if h2 < 10 {
		bs = append(bs, '0'+h2)
	} else {
		bs = append(bs, 'a'+h2-10)
	}

	return bs
}

func hexdump(data []byte, offset int, writer io.Writer) {
	skip := false

	dataOffset := 0
	unitSize := 16 * 1000
	for dataOffset < len(data) {
		unit := data[dataOffset:min(dataOffset+unitSize, len(data))]

		for i := 0; i < len(unit)/16; i += 1 {
			addr := i * 16
			chunk := unit[addr:min(addr+16, len(unit))]
			hexVals := make([]byte, 0, 48)
			asciiVals := make([]byte, 0, 16)
			nonZeroFlag := false
			for _, b := range chunk {
				// if b < 16 {
				// 	hexVals = append(hexVals, '0')
				// }
				// hexVals = strconv.AppendUint(hexVals, uint64(b), 16)
				hexVals = appendByteAsHex(hexVals, b)
				hexVals = append(hexVals, ' ')

				if 32 <= b && b <= 126 {
					asciiVals = append(asciiVals, b)
				} else {
					asciiVals = append(asciiVals, '.')
				}

				if b != 0 {
					nonZeroFlag = true
				}
			}

			if nonZeroFlag {
				fmt.Fprintf(writer, "%08x  %s |%s|\n", addr+offset, string(hexVals), string(asciiVals))
				skip = false
			} else {
				if !skip {
					fmt.Fprintf(writer, "%08x  *\n", addr+offset)

					skip = true
				}
			}
		}

		dataOffset += unitSize
	}
}

func deletePreviousFiles(pattern string) {
	files, err := filepath.Glob(pattern)
	if err != nil {
		panic(err)
	}

	for _, f := range files {
		err := os.Remove(f)
		if err != nil {
			panic(err)
		}
		fmt.Printf("Deleted previous file: %s\n", f)
	}
}

func main() {
	// ファイルパスとチャンクサイズを指定
	filePath := "./build/memory/memory.bin"
	chunkSize := 250 * 1024 * 1024
	ext := filepath.Ext(filePath)
	baseName := strings.Replace(filepath.Base(filePath), ext, "", 1)
	dirPath := filepath.Dir(filePath)

	// 前回作成したファイルを削除
	deletePreviousFiles(filepath.Join(dirPath, fmt.Sprintf("%s_chunk_*", baseName)))

	// ファイルのサイズを取得
	fileInfo, err := os.Stat(filePath)
	if err != nil {
		panic(err)
	}
	fileSize := fileInfo.Size()

	// ファイルを開く
	file, err := os.Open(filePath)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	chunkFilePaths := []string{}

	// チャンクごとに処理
	for i := int64(0); i < fileSize; i += int64(chunkSize) {
		fmt.Printf("Processing chunk starting at byte: %d\n", i)

		chunkData := make([]byte, chunkSize)
		n, err := file.Read(chunkData)
		if err != nil && err != io.EOF {
			panic(err)
		}
		chunkData = chunkData[:n]

		chunkFilePath := filepath.Join(dirPath, fmt.Sprintf("%s_chunk_%d%s", baseName, i/int64(chunkSize), ext))
		chunkFilePaths = append(chunkFilePaths, chunkFilePath)

		chunkFile, err := os.Create(chunkFilePath)
		if err != nil {
			panic(err)
		}

		_, err = chunkFile.Write(chunkData)
		if err != nil {
			panic(err)
		}

		chunkFile.Close()
		fmt.Printf("Created chunk file: %s\n", chunkFilePath)
	}

	for index, chunkFilePath := range chunkFilePaths {
		fmt.Printf("Generating hexdump for file: %s\n", chunkFilePath)

		chunkFile, err := os.Open(chunkFilePath)
		if err != nil {
			panic(err)
		}
		defer chunkFile.Close()

		chunkData := make([]byte, chunkSize)
		n, err := chunkFile.Read(chunkData)
		if err != nil && err != io.EOF {
			panic(err)
		}
		chunkData = chunkData[:n]

		hexdumpFilePath := fmt.Sprintf("%v.hexdump", chunkFilePath)
		hexdumpFile, err := os.Create(hexdumpFilePath)
		if err != nil {
			panic(err)
		}
		writer := bufio.NewWriter(hexdumpFile)

		hexdump(chunkData, index*chunkSize, writer)
		writer.Flush()
		hexdumpFile.Close()

		fmt.Printf("Created hexdump file: %s\n", hexdumpFilePath)
	}
}
