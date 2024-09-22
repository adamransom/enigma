A small Enigma (I/M3) emulator written in Rust.

### Usage

```
Usage: enigma [OPTIONS]
Options:
      --reflector <REFLECTOR>           [possible values: A, B, C]
      --rotors <ROTOR> <ROTOR> <ROTOR>  [possible values: I, II, III, IV, V]
      --rings <NUM> <NUM> <NUM>         [possible values: 1 to 26]
      --plugs [<PLUGS>...]              [possible values: two unique alphabet characters]
      --key <CHAR> <CHAR> <CHAR>
      --input <INPUT>
  -h, --help                            Print help
  -V, --version                         Print version
```

### Example

The following example is taken from the [1930s Enigma Instruction Manual](http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Messages), originally for the Model I.

```bash
> enigma --reflector A --rotors II I III --rings 24 13 22 --plugs AM FI NV PS TU WZ --key A B L --input "GCDSE AHUGW TQGRK VLFGX UCALX VYMIG MMNMF"
FEIND LIQEI NFANT ERIEK OLONN EBEOB AQTET
```

### References

- https://en.wikipedia.org/wiki/Enigma_rotor_details
- http://www.ellsbury.com/ultraenigmawirings.htm
- http://www.ellsbury.com/enigma2.htm
- https://www.cryptomuseum.com/crypto/enigma/working.htm
- https://www.cryptomuseum.com/crypto/enigma/wiring.htm
- http://wiki.franklinheath.co.uk/index.php/Enigma/Sample_Messages
- https://www.cryptomuseum.com/people/hamer/files/double_stepping.pdf
- https://cryptii.com/pipes/enigma-machine
- https://piotte13.github.io/enigma-cipher/
