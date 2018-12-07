def decodeMorse(morse_code):
    morse_code = morse_code.strip()
    res = []
    for word in morse_code.split('   '):
        for letter in word.split(' '):
            res.append(MORSE_CODE[letter])
        res.append(' ')
    return ''.join(res[:-1])


# 惯例的一行代码...作者:Bitman, nkrause323, lucasLB7
def decodeMorse(morseCode):
    return ' '.join(''.join(MORSE_CODE[letter] for letter in word.split(' '))
                    for word in morseCode.strip().split('   '))
