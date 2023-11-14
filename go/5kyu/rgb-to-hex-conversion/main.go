package kata

import (
	"fmt"
	"strings"
)

func RGB(r, g, b int) string {
	red := fmt.Sprintf("%02X", min(max(r, 0), 255))
	green := fmt.Sprintf("%02X", min(max(g, 0), 255))
	blue := fmt.Sprintf("%02X", min(max(b, 0), 255))

	hex := strings.Join([]string{red, green, blue}, "")
	return hex
}
