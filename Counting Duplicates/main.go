package kata

func duplicate_count(s1 string) int {
	// s1 = strings.ToLower(s1)
	res := 0
	dic := map[byte]int{}
	for i := 0; i < len(s1); i++ {
		ch := s1[i]
		if ch > 96 {
			ch -= 32
		}
		dic[ch]++
		if dic[ch] == 2 {
			res++
		}

	}

	return res
}
