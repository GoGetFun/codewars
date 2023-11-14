package kata

func Interpreter(code string) string {
 var letter byte = 0
 var byteArr []byte
 for _, j := range code {
  if j == '+' {
   letter += 1
  } else if j == '.' {
   byteArr = append(byteArr, letter)
  }
 }
 return string(byteArr)
}