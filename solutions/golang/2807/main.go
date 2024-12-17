package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type bufReader struct {
	r   *bufio.Reader
	buf []byte
	i   int
}

var reader = &bufReader{
	bufio.NewReader(os.Stdin),
	make([]byte, 0),
	0,
}

var writer = bufio.NewWriter(os.Stdout)

func (r *bufReader) readLine() {
	if r.i < len(r.buf) {
		return
	}
	r.buf = make([]byte, 0)
	r.i = 0
	for {
		line, isPrefix, err := r.r.ReadLine()
		if err != nil {
			panic(err)
		}
		r.buf = append(r.buf, line...)
		if !isPrefix {
			break
		}
	}
}

func (r *bufReader) next() string {
	r.readLine()
	from := r.i
	for ; r.i < len(r.buf); r.i++ {
		if r.buf[r.i] == ' ' {
			break
		}
	}
	s := string(r.buf[from:r.i])
	r.i++
	return s
}

func next() string {
	return reader.next()
}

func nextInt() int {
	i, err := strconv.Atoi(next())
	if err != nil {
		panic(err)
	}
	return i
}

func out(format string, a ...interface{}) {
	fmt.Fprintf(writer, format+"\n", a...)
	writer.Flush()
}

func fibo(n int) []int64 {
	fibSequencia := make([]int64, n)
	var F, ant int64

	for i := 1; i <= n; i++ {
		if i == 1 {
			F = 1
			ant = 0
		} else {
			F += ant
			ant = F - ant
		}

		fibSequencia[i-1] = F
	}

	return fibSequencia
}

func solve() {
	numero := nextInt()

	vetor := fibo(numero)

	for i := numero - 1; i >= 0; i-- {
		out("%d", vetor[i])
		if i > 0 {
			writer.WriteString(" ")
		}
	}

	writer.WriteString("\n")
	writer.Flush()
}

func main() {
	solve()
}
